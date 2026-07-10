use beavermath_core::types::MathArtifactV1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    MessageStart,
    MessageUpdate,
    MessageEnd,
    TurnEnd,
    Branch,
    Resume,
    SessionSwitch,
    Shutdown,
    Cancel,
    Replace,
    Delete,
}

/// Neutral adapter event: artifacts and metadata only — never terminal escapes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdapterEventV1 {
    pub schema_version: String,
    pub kind: EventKind,
    pub message_id: String,
    pub revision: u64,
    pub content_sha256: String,
    #[serde(default)]
    pub artifacts: Vec<MathArtifactV1>,
    /// Machine-safe payload (JSON text). Must not contain ESC/CSI/OSC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_preview: Option<String>,
}

impl AdapterEventV1 {
    pub fn new(
        kind: EventKind,
        message_id: impl Into<String>,
        revision: u64,
        content_sha256: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: beavermath_core::SCHEMA_VERSION.into(),
            kind,
            message_id: message_id.into(),
            revision,
            content_sha256: content_sha256.into(),
            artifacts: vec![],
            semantic_preview: None,
        }
    }

    pub fn to_machine_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beavermath_core::hash::sha256_hex;

    #[test]
    fn machine_json_has_no_escapes() {
        let mut ev = AdapterEventV1::new(EventKind::MessageEnd, "m1", 1, sha256_hex(b"hello $x$"));
        ev.semantic_preview = Some("hello (x)".into());
        let s = ev.to_machine_json().expect("json");
        assert!(!s.as_bytes().contains(&0x1b));
    }
}
