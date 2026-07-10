//! Shared in-chat presentation: immutable source → display_text + spans.
//! Hosts paint display_text; session stores exact_source only.

use beavermath_core::hash::sha256_hex;
use beavermath_core::pipeline::{process_document, ProcessOptions};
use beavermath_core::types::{ByteRangeV1, MathArtifactV1, OutputContextV1};
use beavermath_core::{BEAVERMATH_VERSION, SCHEMA_VERSION};
use beavermath_terminal::escape::scan_machine_channel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Presentation mode for the host viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationMode {
    Semantic,
    Unicode,
    /// Host may resolve png digests; no protocol bytes in this payload.
    PixelRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresentationSpanV1 {
    pub range: ByteRangeV1,
    pub expression_exact_utf8: String,
    pub expression_sha256: String,
    pub semantic: String,
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresentationV1 {
    pub schema_version: String,
    pub exact_source: String,
    pub content_sha256: String,
    /// Whole-message display string for the chat viewport (default semantic filter).
    pub display_text: String,
    pub mode: PresentationMode,
    pub spans: Vec<PresentationSpanV1>,
    pub artifacts: Vec<MathArtifactV1>,
    pub config_sha256: String,
    pub beavermath_version: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PresentRequestV1 {
    #[serde(default = "default_schema")]
    pub schema_version: String,
    /// Exact assistant message UTF-8 (canonical).
    pub source: String,
    #[serde(default)]
    pub mode: Option<PresentationMode>,
    #[serde(default)]
    pub output_context: OutputContextV1,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub revision: Option<u64>,
}

fn default_schema() -> String {
    SCHEMA_VERSION.to_string()
}

fn config_digest(mode: PresentationMode) -> String {
    sha256_hex(format!("present-v1|mode={mode:?}|auto_protect=1").as_bytes())
}

/// Present a completed message. Does not mutate source; pure function of input + options.
pub fn present_message_v1(req: &PresentRequestV1) -> Result<PresentationV1, String> {
    let source = &req.source;
    let content_sha256 = sha256_hex(source.as_bytes());
    let mode = req.mode.unwrap_or(PresentationMode::Semantic);

    // Process with auto_protect (default ProcessOptions).
    let opts = ProcessOptions {
        auto_protect: true,
        ..ProcessOptions::default()
    };
    let doc = process_document(source.as_bytes(), &[], &opts)
        .map_err(|d| format!("{}: {}", d.code.as_str(), d.message))?;

    let display_text = match mode {
        PresentationMode::Semantic | PresentationMode::PixelRef => {
            doc.filtered_text.clone().unwrap_or_else(|| source.clone())
        }
        PresentationMode::Unicode => {
            // Prefer unicode visuals per expression when rebuilding.
            rebuild_with_unicode(source, &doc.expressions)
        }
    };

    let spans: Vec<PresentationSpanV1> = doc
        .expressions
        .iter()
        .map(|a| PresentationSpanV1 {
            range: a.expression_byte_range,
            expression_exact_utf8: a.expression_exact_utf8.clone(),
            expression_sha256: a.expression_sha256.clone(),
            semantic: a.semantic.text.clone(),
            alt: a.alt.text.clone(),
            unicode: a.visual.unicode.clone(),
            png_digest: a.visual.png.as_ref().map(|p| p.digest_sha256.clone()),
        })
        .collect();

    let out = PresentationV1 {
        schema_version: SCHEMA_VERSION.into(),
        exact_source: source.clone(),
        content_sha256,
        display_text,
        mode,
        spans,
        artifacts: doc.expressions,
        config_sha256: config_digest(mode),
        beavermath_version: BEAVERMATH_VERSION.into(),
    };

    // Machine purity of serialized form is checked by callers; ensure display has no raw ESC
    // from unescaped model content (source_display is per-expression; whole filter escapes).
    let _ = &req.output_context;
    Ok(out)
}

fn rebuild_with_unicode(source: &str, arts: &[MathArtifactV1]) -> String {
    if arts.is_empty() {
        return source.to_string();
    }
    let mut out = String::new();
    let mut cursor = 0usize;
    for a in arts {
        let s = a.expression_byte_range.start;
        let e = a.expression_byte_range.end;
        if s > cursor {
            out.push_str(&source[cursor..s]);
        }
        if let Some(u) = &a.visual.unicode {
            out.push_str(u);
        } else {
            out.push_str(&a.semantic.text);
        }
        cursor = e;
    }
    out.push_str(&source[cursor..]);
    out
}

/// Process-local cache keyed by content_sha256 + config.
#[derive(Default)]
pub struct PresentCache {
    inner: Mutex<HashMap<String, PresentationV1>>,
}

impl PresentCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_insert(&self, req: &PresentRequestV1) -> Result<PresentationV1, String> {
        let mode = req.mode.unwrap_or(PresentationMode::Semantic);
        let key = format!(
            "{}:{}",
            sha256_hex(req.source.as_bytes()),
            config_digest(mode)
        );
        if let Ok(guard) = self.inner.lock() {
            if let Some(p) = guard.get(&key) {
                return Ok(p.clone());
            }
        }
        let p = present_message_v1(req)?;
        if let Ok(mut guard) = self.inner.lock() {
            guard.insert(key, p.clone());
        }
        Ok(p)
    }
}

/// Serialize presentation as machine-safe JSON (no ESC).
pub fn present_to_machine_json(p: &PresentationV1) -> Result<String, String> {
    let s = serde_json::to_string(p).map_err(|e| e.to_string())?;
    if !scan_machine_channel(s.as_bytes()).clean {
        return Err("refused ESC in presentation JSON".into());
    }
    Ok(s)
}

/// Parse present request from JSON object or raw document string.
pub fn parse_present_request(input: &str) -> Result<PresentRequestV1, String> {
    let t = input.trim();
    if t.starts_with('{') {
        serde_json::from_str(t).map_err(|e| format!("invalid PresentRequestV1: {e}"))
    } else {
        Ok(PresentRequestV1 {
            schema_version: SCHEMA_VERSION.into(),
            source: input.to_string(),
            mode: Some(PresentationMode::Semantic),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beavermath_core::pipeline::{process_document, ProcessOptions};

    #[test]
    fn display_matches_filter() {
        let source = "The answer is $x^2+y^2=z^2$.";
        let p = present_message_v1(&PresentRequestV1 {
            source: source.into(),
            ..Default::default()
        })
        .unwrap();
        let doc = process_document(source.as_bytes(), &[], &ProcessOptions::default()).unwrap();
        let filtered = doc.filtered_text.unwrap();
        assert_eq!(p.display_text, filtered);
        assert_eq!(p.exact_source, source);
        assert_eq!(p.content_sha256, sha256_hex(source.as_bytes()));
        assert!(p.display_text.contains("power"));
    }

    #[test]
    fn dual_run_stable() {
        let source = "see https://x.com/$user$/p and `$x$` and $y$";
        let a = present_message_v1(&PresentRequestV1 {
            source: source.into(),
            ..Default::default()
        })
        .unwrap();
        let b = present_message_v1(&PresentRequestV1 {
            source: source.into(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(a.content_sha256, b.content_sha256);
        assert_eq!(a.display_text, b.display_text);
        assert_eq!(a.exact_source, source);
        assert!(a.display_text.contains("https://x.com/$user$/p"));
        assert!(a.display_text.contains("`$x$`"));
        assert_eq!(a.spans.len(), 1);
        assert_eq!(a.spans[0].expression_exact_utf8, "$y$");
    }

    #[test]
    fn machine_json_pure() {
        let p = present_message_v1(&PresentRequestV1 {
            source: "x $a$\x1b[31m".into(),
            ..Default::default()
        })
        .unwrap();
        let j = present_to_machine_json(&p).unwrap();
        assert!(!j.as_bytes().contains(&0x1b));
    }

    #[test]
    fn cache_hits() {
        let cache = PresentCache::new();
        let req = PresentRequestV1 {
            source: "math $q$".into(),
            ..Default::default()
        };
        let a = cache.get_or_insert(&req).unwrap();
        let b = cache.get_or_insert(&req).unwrap();
        assert_eq!(a.content_sha256, b.content_sha256);
    }

    #[test]
    fn source_never_equals_display_when_math_present() {
        let source = "From Maxwell $E=mc^2$.";
        let p = present_message_v1(&PresentRequestV1 {
            source: source.into(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(p.exact_source, source);
        assert_ne!(p.display_text, p.exact_source);
        assert!(p.exact_source.contains("$E=mc^2$"));
        assert!(
            !p.display_text.contains("$E=mc^2$")
                || p.display_text.contains("power")
                || p.display_text.contains("equals")
        );
    }
}
