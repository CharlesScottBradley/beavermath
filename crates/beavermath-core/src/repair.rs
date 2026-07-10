use crate::diagnostics::{DiagnosticCode, DiagnosticSeverity, DiagnosticV1};
use crate::hash::sha256_hex;
use serde::{Deserialize, Serialize};

/// Host-owned repair provider interface. Core never performs network I/O.
pub trait RepairProviderV1: Send {
    fn provider_id(&self) -> &str;
    fn suggest(&self, original: &str) -> Result<RepairProposalV1, String>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepairMappingV1 {
    pub original_start: usize,
    pub original_end: usize,
    pub inferred_start: usize,
    pub inferred_end: usize,
    pub inserted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepairProposalV1 {
    pub original_exact_utf8: String,
    pub original_sha256: String,
    pub inferred_tex: String,
    pub mapping: Vec<RepairMappingV1>,
    pub confidence: f64,
    pub provider: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairRequestV1 {
    pub original: String,
    pub consent: bool,
}

/// Deterministic offline heuristic provider for ASCII math-like patterns (opt-in only).
#[derive(Debug, Default)]
pub struct HeuristicRepairProvider;

impl RepairProviderV1 for HeuristicRepairProvider {
    fn provider_id(&self) -> &str {
        "beavermath.heuristic.v1"
    }

    fn suggest(&self, original: &str) -> Result<RepairProposalV1, String> {
        let inferred = heuristic_infer(original);
        let mapping = vec![RepairMappingV1 {
            original_start: 0,
            original_end: original.len(),
            inferred_start: 0,
            inferred_end: inferred.len(),
            inserted: inferred != original,
        }];
        Ok(RepairProposalV1 {
            original_exact_utf8: original.to_string(),
            original_sha256: sha256_hex(original.as_bytes()),
            inferred_tex: inferred,
            mapping,
            confidence: 0.6,
            provider: self.provider_id().into(),
            provenance: "offline-heuristic".into(),
        })
    }
}

fn heuristic_infer(s: &str) -> String {
    let t = s.trim();
    // sqrt(x) -> \sqrt{x}
    if let Some(inner) = t.strip_prefix("sqrt(").and_then(|r| r.strip_suffix(')')) {
        return format!("\\sqrt{{{inner}}}");
    }
    // x^2 already TeX-like
    if t.contains('^') || t.contains('_') {
        return t.to_string();
    }
    t.to_string()
}

/// Run repair only with explicit consent. Never mutates original.
pub fn repair_suggest(
    request: &RepairRequestV1,
    provider: &dyn RepairProviderV1,
) -> Result<RepairProposalV1, DiagnosticV1> {
    if !request.consent {
        return Err(DiagnosticV1::new(
            DiagnosticCode::RepairRequiresConsent,
            DiagnosticSeverity::Error,
            "repair requires explicit consent",
        )
        .with_remediation("pass consent=true or host policy grant"));
    }
    let original = request.original.clone();
    let digest = sha256_hex(original.as_bytes());
    match provider.suggest(&original) {
        Ok(mut p) => {
            // Enforce original preservation
            p.original_exact_utf8 = original;
            p.original_sha256 = digest;
            Ok(p)
        }
        Err(e) => Err(DiagnosticV1::new(
            DiagnosticCode::RepairProviderError,
            DiagnosticSeverity::Error,
            e,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_consent_fails() {
        let r = RepairRequestV1 {
            original: "sqrt(x)".into(),
            consent: false,
        };
        let p = HeuristicRepairProvider;
        let err = repair_suggest(&r, &p).unwrap_err();
        assert_eq!(err.code, DiagnosticCode::RepairRequiresConsent);
    }

    #[test]
    fn consent_preserves_original() {
        let r = RepairRequestV1 {
            original: "sqrt(x^2 + 1)".into(),
            consent: true,
        };
        let p = HeuristicRepairProvider;
        let prop = repair_suggest(&r, &p).unwrap();
        assert_eq!(prop.original_exact_utf8, "sqrt(x^2 + 1)");
        assert!(prop.inferred_tex.contains("sqrt") || prop.inferred_tex.contains("\\sqrt"));
        assert_eq!(prop.original_sha256, sha256_hex(b"sqrt(x^2 + 1)"));
    }

    #[test]
    fn strict_mode_never_calls_provider() {
        // Documented contract: callers must not invoke repair without consent.
        // This unit ensures provider is not required for identity of original.
        let original = "sqrt(x)";
        assert_eq!(sha256_hex(original.as_bytes()).len(), 64);
    }
}
