use crate::diagnostics::{DiagnosticCode, DiagnosticSeverity, DiagnosticV1};
use crate::hash::{sha256_hex, validate_utf8, HashError};
use crate::limits::LimitsV1;
use crate::partition::{build_partition, partition_is_lossless, reconstruct};
use crate::protect::merge_protected;
use crate::scanner::{discover_strict, MathSpan, ProtectedRange};
use crate::semantic::{semantic_for_tex, source_display};
use crate::types::*;
use crate::{BEAVERMATH_VERSION, SCHEMA_VERSION};

#[derive(Debug, Clone)]
pub struct ProcessOptions {
    pub limits: LimitsV1,
    pub request_png: bool,
    pub request_svg: bool,
    /// Optional hook for visual rendering (filled by beavermath-render).
    pub render_hook: bool,
    /// When true (default), merge URL/code default protection with host ranges.
    pub auto_protect: bool,
}

impl Default for ProcessOptions {
    fn default() -> Self {
        Self {
            limits: LimitsV1::default(),
            request_png: false,
            request_svg: false,
            render_hook: false,
            auto_protect: true,
        }
    }
}

/// Process a full document: discover, partition, semantic for each expression.
pub fn process_document(
    input_bytes: &[u8],
    protected: &[ProtectedRange],
    opts: &ProcessOptions,
) -> Result<DocumentResultV1, DiagnosticV1> {
    let input = match validate_utf8(input_bytes) {
        Ok(s) => s,
        Err(HashError::InvalidUtf8 { offset }) => {
            return Err(DiagnosticV1::new(
                DiagnosticCode::InvalidUtf8,
                DiagnosticSeverity::Error,
                format!("invalid UTF-8 at byte {offset}"),
            )
            .with_range(offset, offset.saturating_add(1)));
        }
    };

    let input_sha = sha256_hex(input.as_bytes());
    let merged: Vec<ProtectedRange> = if opts.auto_protect {
        merge_protected(input, protected)
    } else {
        protected.to_vec()
    };
    let (math_spans, scan_diags) =
        discover_strict(input, &merged, &opts.limits).map_err(|e| {
            DiagnosticV1::new(
                DiagnosticCode::ProtectedRangeInvalid,
                DiagnosticSeverity::Error,
                e.to_string(),
            )
        })?;

    let partition = build_partition(input.len(), &math_spans);
    let lossless = partition_is_lossless(input, &partition);
    if !lossless {
        return Err(DiagnosticV1::new(
            DiagnosticCode::Internal,
            DiagnosticSeverity::Error,
            "partition is not lossless",
        ));
    }

    if math_spans.is_empty() {
        let mut diagnostics = scan_diags;
        diagnostics.push(DiagnosticV1::new(
            DiagnosticCode::Unchanged,
            DiagnosticSeverity::Info,
            "no accepted math spans",
        ));
        return Ok(DocumentResultV1 {
            schema_version: SCHEMA_VERSION.into(),
            status: RenderStatusV1::Unchanged,
            input_exact_utf8: input.to_string(),
            input_sha256: input_sha,
            partition_ok: true,
            expressions: vec![],
            diagnostics,
            filtered_text: Some(input.to_string()),
        });
    }

    let mut expressions = Vec::with_capacity(math_spans.len());
    for (idx, span) in math_spans.iter().enumerate() {
        expressions.push(artifact_for_span(input, &input_sha, span, idx, opts));
    }

    let filtered = filter_document(input, &math_spans, &expressions);

    Ok(DocumentResultV1 {
        schema_version: SCHEMA_VERSION.into(),
        status: RenderStatusV1::Ok,
        input_exact_utf8: input.to_string(),
        input_sha256: input_sha,
        partition_ok: true,
        expressions,
        diagnostics: scan_diags,
        filtered_text: Some(filtered),
    })
}

/// Direct expression endpoint: whole input is TeX (with or without delimiters stripped by caller).
pub fn process_expression(
    input_bytes: &[u8],
    mode: DisplayMode,
    opts: &ProcessOptions,
) -> Result<MathArtifactV1, DiagnosticV1> {
    let input = match validate_utf8(input_bytes) {
        Ok(s) => s,
        Err(HashError::InvalidUtf8 { offset }) => {
            return Err(DiagnosticV1::new(
                DiagnosticCode::InvalidUtf8,
                DiagnosticSeverity::Error,
                format!("invalid UTF-8 at byte {offset}"),
            )
            .with_range(offset, offset.saturating_add(1)));
        }
    };
    let input_sha = sha256_hex(input.as_bytes());
    let span = MathSpan {
        full_range: ByteRangeV1 {
            start: 0,
            end: input.len(),
        },
        inner_range: ByteRangeV1 {
            start: 0,
            end: input.len(),
        },
        mode,
        delimiter: crate::scanner::DelimiterKind::DollarInline,
    };
    // Prefer stripping outer delimiters if present for direct mode
    let mut span = span;
    let trimmed = strip_outer_delimiters(input);
    if trimmed.start != 0 || trimmed.end != input.len() {
        span.inner_range = trimmed;
    }
    let mut art = artifact_for_span(input, &input_sha, &span, 0, opts);
    art.discovery = DiscoveryKind::DirectExpression;
    Ok(art)
}

fn strip_outer_delimiters(input: &str) -> ByteRangeV1 {
    let t = input.trim();
    let start_trim = input.find(t).unwrap_or(0);
    let bytes = t.as_bytes();
    if t.starts_with("$$") && t.ends_with("$$") && t.len() >= 4 {
        return ByteRangeV1 {
            start: start_trim + 2,
            end: start_trim + t.len() - 2,
        };
    }
    if t.starts_with('$') && t.ends_with('$') && t.len() >= 2 && !t.starts_with("$$") {
        return ByteRangeV1 {
            start: start_trim + 1,
            end: start_trim + t.len() - 1,
        };
    }
    if t.starts_with("\\(") && t.ends_with("\\)") && t.len() >= 4 {
        return ByteRangeV1 {
            start: start_trim + 2,
            end: start_trim + t.len() - 2,
        };
    }
    if t.starts_with("\\[") && t.ends_with("\\]") && t.len() >= 4 {
        return ByteRangeV1 {
            start: start_trim + 2,
            end: start_trim + t.len() - 2,
        };
    }
    let _ = bytes;
    ByteRangeV1 {
        start: 0,
        end: input.len(),
    }
}

fn artifact_for_span(
    input: &str,
    input_sha: &str,
    span: &MathSpan,
    idx: usize,
    _opts: &ProcessOptions,
) -> MathArtifactV1 {
    let full = &input[span.full_range.start..span.full_range.end];
    let inner = &input[span.inner_range.start..span.inner_range.end];
    let expr_sha = sha256_hex(full.as_bytes());
    let sem = semantic_for_tex(inner, span.mode);
    let config = sha256_hex(b"beavermath-config-v1-default");

    MathArtifactV1 {
        schema_version: SCHEMA_VERSION.into(),
        input_exact_utf8: input.to_string(),
        input_sha256: input_sha.to_string(),
        expression_id: format!("expr-{idx}"),
        expression_byte_range: span.full_range.clone(),
        expression_exact_utf8: full.to_string(),
        expression_sha256: expr_sha,
        source_display: source_display(full),
        discovery: DiscoveryKind::Explicit,
        parse: ParseBlockV1 {
            status: ParseStatus::Ok,
            diagnostics: vec![],
        },
        semantic: SemanticBlockV1 {
            text: sem.text,
            locale: sem.locale,
            status: sem.status,
            node_spans: vec![],
        },
        alt: AltBlockV1 {
            text: sem.alt_text,
            status: sem.alt_status,
        },
        repair: None,
        mathml: None,
        visual: VisualBlockV1 {
            unicode: sem.unicode,
            ascii: sem.ascii,
            png: None,
            svg: None,
        },
        provenance: ProvenanceV1 {
            beavermath_version: BEAVERMATH_VERSION.into(),
            renderer_version: None,
            config_sha256: config,
        },
    }
}

/// Replace math spans with semantic text placeholders while keeping non-math exact.
fn filter_document(input: &str, spans: &[MathSpan], arts: &[MathArtifactV1]) -> String {
    let mut out = String::new();
    let mut cursor = 0usize;
    for (i, span) in spans.iter().enumerate() {
        out.push_str(&input[cursor..span.full_range.start]);
        let replacement = arts
            .get(i)
            .map(|a| a.semantic.text.as_str())
            .unwrap_or(span_inner(input, span));
        out.push_str(replacement);
        cursor = span.full_range.end;
    }
    out.push_str(&input[cursor..]);
    out
}

fn span_inner<'a>(input: &'a str, span: &MathSpan) -> &'a str {
    &input[span.inner_range.start..span.inner_range.end]
}

/// Verify partition reconstruction (public for tests).
pub fn assert_partition_roundtrip(input: &str) -> bool {
    let limits = LimitsV1::default();
    let (math, _) = match discover_strict(input, &[], &limits) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let p = build_partition(input.len(), &math);
    partition_is_lossless(input, &p) && reconstruct(input, &p) == input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_with_math() {
        let input = b"The answer is $x^2+y^2=z^2$.";
        let res = process_document(input, &[], &ProcessOptions::default()).unwrap();
        assert_eq!(res.status, RenderStatusV1::Ok);
        assert_eq!(res.expressions.len(), 1);
        assert!(!res.expressions[0].semantic.text.is_empty());
        assert!(!res.expressions[0].alt.text.is_empty());
        assert_eq!(
            res.input_sha256,
            sha256_hex(input)
        );
        assert_eq!(
            res.expressions[0].expression_sha256,
            sha256_hex(b"$x^2+y^2=z^2$")
        );
        // Full input recoverable
        assert_eq!(res.input_exact_utf8.as_bytes(), input);
    }

    #[test]
    fn unchanged_without_math() {
        let input = b"just prose and $5 money";
        let res = process_document(input, &[], &ProcessOptions::default()).unwrap();
        assert_eq!(res.status, RenderStatusV1::Unchanged);
        assert_eq!(res.input_exact_utf8.as_bytes(), input);
    }

    #[test]
    fn invalid_utf8() {
        let input = b"ok\xff";
        let err = process_document(input, &[], &ProcessOptions::default()).unwrap_err();
        assert_eq!(err.code, DiagnosticCode::InvalidUtf8);
    }

    #[test]
    fn chunk_invariance_logical() {
        // Same content discovered regardless of how we conceptually chunk
        let a = "A $x$ B $y$ C";
        assert!(assert_partition_roundtrip(a));
        let res = process_document(a.as_bytes(), &[], &ProcessOptions::default()).unwrap();
        assert_eq!(res.expressions.len(), 2);
    }
}
