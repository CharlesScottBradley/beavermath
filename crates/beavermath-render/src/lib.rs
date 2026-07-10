//! Bounded RaTeX adapter and immutable visual artifacts.

pub mod cache;
pub mod confinement;
pub mod ratex_adapter;

use beavermath_core::diagnostics::{DiagnosticCode, DiagnosticSeverity, DiagnosticV1};
use beavermath_core::hash::sha256_hex;
use beavermath_core::types::{MathArtifactV1, VisualArtifactV1};
use beavermath_core::{process_document, process_expression, ProcessOptions, DisplayMode};
use ratex_adapter::{render_png, render_svg, RENDERER_VERSION};

pub use cache::{CacheStats, PersistentCache};
pub use confinement::{assert_no_hostile_side_effects, HostileCheck};
pub use ratex_adapter::SupportedTeX;

/// Enrich a document result's expressions with PNG/SVG when requested.
pub fn process_document_with_visuals(
    input: &[u8],
    protected: &[beavermath_core::ProtectedRange],
    opts: &ProcessOptions,
) -> Result<beavermath_core::DocumentResultV1, DiagnosticV1> {
    let mut result = process_document(input, protected, opts)?;
    for expr in &mut result.expressions {
        attach_visuals(expr, opts)?;
    }
    Ok(result)
}

pub fn process_expression_with_visuals(
    input: &[u8],
    mode: DisplayMode,
    opts: &ProcessOptions,
) -> Result<MathArtifactV1, DiagnosticV1> {
    let mut art = process_expression(input, mode, opts)?;
    attach_visuals(&mut art, opts)?;
    Ok(art)
}

fn attach_visuals(art: &mut MathArtifactV1, opts: &ProcessOptions) -> Result<(), DiagnosticV1> {
    let inner = strip_delims(&art.expression_exact_utf8);
    if opts.request_svg {
        match render_svg(inner) {
            Ok(svg) => {
                let digest = sha256_hex(svg.as_bytes());
                art.visual.svg = Some(VisualArtifactV1 {
                    media_type: "image/svg+xml".into(),
                    bytes_b64: base64_encode(svg.as_bytes()),
                    digest_sha256: digest,
                    width: None,
                    height: None,
                    byte_length: svg.len(),
                });
                art.provenance.renderer_version = Some(RENDERER_VERSION.into());
            }
            Err(e) => {
                art.parse.diagnostics.push(DiagnosticV1::new(
                    DiagnosticCode::RenderFailed,
                    DiagnosticSeverity::Warning,
                    e,
                ));
                // Visual failure must not remove semantic/source
            }
        }
    }
    if opts.request_png {
        match render_png(inner) {
            Ok((png, w, h)) => {
                let digest = sha256_hex(&png);
                art.visual.png = Some(VisualArtifactV1 {
                    media_type: "image/png".into(),
                    bytes_b64: base64_encode(&png),
                    digest_sha256: digest,
                    width: Some(w),
                    height: Some(h),
                    byte_length: png.len(),
                });
                art.provenance.renderer_version = Some(RENDERER_VERSION.into());
            }
            Err(e) => {
                art.parse.diagnostics.push(DiagnosticV1::new(
                    DiagnosticCode::RenderFailed,
                    DiagnosticSeverity::Warning,
                    e,
                ));
            }
        }
    }
    Ok(())
}

fn strip_delims(expr: &str) -> &str {
    let t = expr.trim();
    if t.starts_with("$$") && t.ends_with("$$") && t.len() >= 4 {
        return &t[2..t.len() - 2];
    }
    if t.starts_with('$') && t.ends_with('$') && t.len() >= 2 {
        return &t[1..t.len() - 1];
    }
    if t.starts_with("\\(") && t.ends_with("\\)") {
        return &t[2..t.len() - 2];
    }
    if t.starts_with("\\[") && t.ends_with("\\]") {
        return &t[2..t.len() - 2];
    }
    t
}

fn base64_encode(bytes: &[u8]) -> String {
    // Minimal base64 without extra dep on this crate — use simple alphabet
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i] as u32;
        let b1 = if i + 1 < bytes.len() {
            bytes[i + 1] as u32
        } else {
            0
        };
        let b2 = if i + 2 < bytes.len() {
            bytes[i + 2] as u32
        } else {
            0
        };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[((triple >> 18) & 63) as usize] as char);
        out.push(T[((triple >> 12) & 63) as usize] as char);
        if i + 1 < bytes.len() {
            out.push(T[((triple >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < bytes.len() {
            out.push(T[(triple & 63) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_survives_render_request() {
        let opts = ProcessOptions {
            request_svg: true,
            request_png: false,
            ..Default::default()
        };
        let res = process_document_with_visuals(b"See $x+1$.", &[], &opts).unwrap();
        assert_eq!(res.expressions.len(), 1);
        assert!(!res.expressions[0].semantic.text.is_empty());
        assert!(!res.expressions[0].expression_exact_utf8.is_empty());
        if let Some(svg) = &res.expressions[0].visual.svg {
            assert_eq!(svg.digest_sha256.len(), 64);
            assert_eq!(svg.media_type, "image/svg+xml");
            assert!(svg.byte_length > 0);
        }
    }

    #[test]
    fn failure_preserves_source() {
        let opts = ProcessOptions {
            request_svg: true,
            ..Default::default()
        };
        let res =
            process_document_with_visuals(b"bad $\\input{/etc/passwd}$.", &[], &opts).unwrap();
        assert_eq!(
            res.expressions[0].expression_exact_utf8,
            "$\\input{/etc/passwd}$"
        );
        assert!(!res.expressions[0].semantic.text.is_empty());
    }
}
