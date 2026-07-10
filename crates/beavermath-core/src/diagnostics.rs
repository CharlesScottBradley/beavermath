use serde::{Deserialize, Serialize};

/// Stable diagnostic codes owned by BeaverMath (not upstream renderer wording).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCode {
    Ok,
    InvalidUtf8,
    EmptyInput,
    Unchanged,
    UnclosedDelimiter,
    MalformedExpression,
    UnsupportedCommand,
    ProtectedRangeInvalid,
    LimitExceeded,
    ParseFailed,
    RenderFailed,
    Timeout,
    Cancelled,
    RepairRequiresConsent,
    RepairProviderError,
    CapabilityMissing,
    MachineChannel,
    SchemaValidationFailed,
    Internal,
}

impl DiagnosticCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::InvalidUtf8 => "invalid_utf8",
            Self::EmptyInput => "empty_input",
            Self::Unchanged => "unchanged",
            Self::UnclosedDelimiter => "unclosed_delimiter",
            Self::MalformedExpression => "malformed_expression",
            Self::UnsupportedCommand => "unsupported_command",
            Self::ProtectedRangeInvalid => "protected_range_invalid",
            Self::LimitExceeded => "limit_exceeded",
            Self::ParseFailed => "parse_failed",
            Self::RenderFailed => "render_failed",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::RepairRequiresConsent => "repair_requires_consent",
            Self::RepairProviderError => "repair_provider_error",
            Self::CapabilityMissing => "capability_missing",
            Self::MachineChannel => "machine_channel",
            Self::SchemaValidationFailed => "schema_validation_failed",
            Self::Internal => "internal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// Process exit statuses (stable for CLI and scripts).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitStatus {
    Success = 0,
    Usage = 2,
    InvalidInput = 3,
    ParseOrRender = 4,
    Limit = 5,
    Io = 6,
    Internal = 1,
}

impl ExitStatus {
    pub fn code(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticV1 {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_range: Option<crate::types::ByteRangeV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
}

impl DiagnosticV1 {
    pub fn new(
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            byte_range: None,
            remediation: None,
        }
    }

    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.byte_range = Some(crate::types::ByteRangeV1 { start, end });
        self
    }

    pub fn with_remediation(mut self, rem: impl Into<String>) -> Self {
        self.remediation = Some(rem.into());
        self
    }

    /// Human one-liner sharing the same code as structured form.
    pub fn human_line(&self) -> String {
        match self.byte_range {
            Some(r) => format!(
                "beavermath[{}]: {} (bytes {}..{})",
                self.code.as_str(),
                self.message,
                r.start,
                r.end
            ),
            None => format!("beavermath[{}]: {}", self.code.as_str(), self.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_and_structured_share_code() {
        let d = DiagnosticV1::new(
            DiagnosticCode::UnclosedDelimiter,
            DiagnosticSeverity::Warning,
            "unclosed $",
        )
        .with_range(3, 4);
        assert!(d.human_line().contains("unclosed_delimiter"));
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("unclosed_delimiter"));
        let back: DiagnosticV1 = serde_json::from_str(&json).unwrap();
        assert_eq!(back.code, DiagnosticCode::UnclosedDelimiter);
    }
}
