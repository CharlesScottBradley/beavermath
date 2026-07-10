use crate::diagnostics::DiagnosticV1;
use crate::repair::RepairProposalV1;
use serde::{Deserialize, Serialize};

/// Closed display mode for an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode {
    Inline,
    Block,
}

/// How an expression was discovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryKind {
    Explicit,
    DirectExpression,
    RepairProposal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseStatus {
    Ok,
    Failed,
    Unsupported,
    Skipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderStatusV1 {
    Ok,
    Unchanged,
    Partial,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputKind {
    Tty,
    Pipe,
    File,
    Structured,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Direct,
    Ssh,
    Tmux,
    Screen,
    Nested,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityPreference {
    Default,
    SemanticOnly,
}

/// Trusted capability for pixel emission (transaction-scoped; never from env alone).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedCapability {
    None,
    KittyDirect,
    Iterm2Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionReason {
    SemanticOnlyPreference,
    NonTty,
    MachineChannel,
    CapabilityMissing,
    KittyAuthorized,
    Iterm2Authorized,
    VisualUnicode,
    ExplicitArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputContextV1 {
    pub output_kind: OutputKind,
    pub transport: TransportKind,
    pub accessibility: AccessibilityPreference,
    pub trusted_capability: TrustedCapability,
    pub is_direct_tty: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_reason: Option<DecisionReason>,
}

impl Default for OutputContextV1 {
    fn default() -> Self {
        Self {
            output_kind: OutputKind::Unknown,
            transport: TransportKind::None,
            accessibility: AccessibilityPreference::Default,
            trusted_capability: TrustedCapability::None,
            is_direct_tty: false,
            decision_reason: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteRangeV1 {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderRequestV1 {
    pub schema_version: String,
    /// Exact UTF-8 input (document or expression).
    pub input: String,
    #[serde(default)]
    pub protected_ranges: Vec<ByteRangeV1>,
    /// When set, treat whole input as a single expression (direct endpoint).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_mode: Option<DisplayMode>,
    #[serde(default)]
    pub output_context: OutputContextV1,
    #[serde(default)]
    pub request_png: bool,
    #[serde(default)]
    pub request_svg: bool,
    #[serde(default)]
    pub consent_repair: bool,
    /// Unknown optional fields are ignored by serde default behavior when deserializing
    /// with deny_unknown_fields off (default).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl RenderRequestV1 {
    pub fn document(input: impl Into<String>) -> Self {
        Self {
            schema_version: crate::SCHEMA_VERSION.to_string(),
            input: input.into(),
            protected_ranges: Vec::new(),
            direct_mode: None,
            output_context: OutputContextV1::default(),
            request_png: false,
            request_svg: false,
            consent_repair: false,
            client_id: None,
        }
    }

    pub fn expression(input: impl Into<String>, mode: DisplayMode) -> Self {
        Self {
            schema_version: crate::SCHEMA_VERSION.to_string(),
            input: input.into(),
            protected_ranges: Vec::new(),
            direct_mode: Some(mode),
            output_context: OutputContextV1::default(),
            request_png: false,
            request_svg: false,
            consent_repair: false,
            client_id: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticBlockV1 {
    pub text: String,
    pub locale: String,
    pub status: String,
    #[serde(default)]
    pub node_spans: Vec<ByteRangeV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AltBlockV1 {
    pub text: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseBlockV1 {
    pub status: ParseStatus,
    pub diagnostics: Vec<DiagnosticV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualArtifactV1 {
    pub media_type: String,
    pub bytes_b64: String,
    pub digest_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    pub byte_length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VisualBlockV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascii: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<VisualArtifactV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<VisualArtifactV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceV1 {
    pub beavermath_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderer_version: Option<String>,
    pub config_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MathArtifactV1 {
    pub schema_version: String,
    pub input_exact_utf8: String,
    pub input_sha256: String,
    pub expression_id: String,
    pub expression_byte_range: ByteRangeV1,
    pub expression_exact_utf8: String,
    pub expression_sha256: String,
    pub source_display: String,
    pub discovery: DiscoveryKind,
    pub parse: ParseBlockV1,
    pub semantic: SemanticBlockV1,
    pub alt: AltBlockV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair: Option<RepairProposalV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mathml: Option<String>,
    #[serde(default)]
    pub visual: VisualBlockV1,
    pub provenance: ProvenanceV1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentResultV1 {
    pub schema_version: String,
    pub status: RenderStatusV1,
    pub input_exact_utf8: String,
    pub input_sha256: String,
    pub partition_ok: bool,
    pub expressions: Vec<MathArtifactV1>,
    pub diagnostics: Vec<DiagnosticV1>,
    /// Filtered document when filter mode is used (non-math text preserved).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trip_preserves_utf8() {
        let mut req = RenderRequestV1::document("The answer is $x^2$.\n");
        req.client_id = Some("t".into());
        let json = serde_json::to_string(&req).unwrap();
        let back: RenderRequestV1 = serde_json::from_str(&json).unwrap();
        assert_eq!(back.input, "The answer is $x^2$.\n");
        assert_eq!(back.schema_version, "1");
    }

    #[test]
    fn unknown_optional_field_ignored() {
        let raw = r#"{"schema_version":"1","input":"hi","future_field":42}"#;
        let req: RenderRequestV1 = serde_json::from_str(raw).unwrap();
        assert_eq!(req.input, "hi");
    }

    #[test]
    fn missing_required_input_fails() {
        let raw = r#"{"schema_version":"1"}"#;
        let err = serde_json::from_str::<RenderRequestV1>(raw).unwrap_err();
        assert!(err.to_string().contains("missing field"));
    }
}
