use beavermath_core::types::{
    AccessibilityPreference, DecisionReason, OutputContextV1, OutputKind, TrustedCapability,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationDecision {
    SemanticOnly,
    VisualUnicode,
    KittyPng,
    Iterm2Png,
}

/// Passive decision engine: zero terminal queries. Env may deny/nominate only via
/// caller-supplied OutputContextV1; never authorizes alone.
pub fn decide_presentation(ctx: &OutputContextV1) -> (PresentationDecision, DecisionReason) {
    if ctx.accessibility == AccessibilityPreference::SemanticOnly {
        return (
            PresentationDecision::SemanticOnly,
            DecisionReason::SemanticOnlyPreference,
        );
    }
    if !ctx.is_direct_tty
        || matches!(
            ctx.output_kind,
            OutputKind::Pipe | OutputKind::File | OutputKind::Structured
        )
    {
        return (
            PresentationDecision::SemanticOnly,
            DecisionReason::MachineChannel,
        );
    }
    match ctx.trusted_capability {
        TrustedCapability::KittyDirect => (
            PresentationDecision::KittyPng,
            DecisionReason::KittyAuthorized,
        ),
        TrustedCapability::Iterm2Inline => (
            PresentationDecision::Iterm2Png,
            DecisionReason::Iterm2Authorized,
        ),
        TrustedCapability::None => (
            PresentationDecision::VisualUnicode,
            DecisionReason::CapabilityMissing,
        ),
    }
}

/// Named graphics preference cannot authorize without trusted capability.
pub fn graphics_preference_allows(
    prefer: Option<&str>,
    ctx: &OutputContextV1,
) -> PresentationDecision {
    let (dec, _) = decide_presentation(ctx);
    match (prefer, ctx.trusted_capability) {
        (Some("kitty"), TrustedCapability::KittyDirect) if ctx.is_direct_tty => {
            PresentationDecision::KittyPng
        }
        (Some("iterm2"), TrustedCapability::Iterm2Inline) if ctx.is_direct_tty => {
            PresentationDecision::Iterm2Png
        }
        (Some("kitty") | Some("iterm2"), _) => {
            // Preference alone insufficient
            PresentationDecision::SemanticOnly
        }
        _ => dec,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_pixels_without_capability() {
        let mut ctx = OutputContextV1::default();
        ctx.is_direct_tty = true;
        ctx.output_kind = OutputKind::Tty;
        ctx.trusted_capability = TrustedCapability::None;
        let (d, _) = decide_presentation(&ctx);
        assert_ne!(d, PresentationDecision::KittyPng);
        assert_ne!(d, PresentationDecision::Iterm2Png);
    }

    #[test]
    fn named_graphics_without_capability_denied() {
        let mut ctx = OutputContextV1::default();
        ctx.is_direct_tty = true;
        ctx.output_kind = OutputKind::Tty;
        ctx.trusted_capability = TrustedCapability::None;
        assert_eq!(
            graphics_preference_allows(Some("kitty"), &ctx),
            PresentationDecision::SemanticOnly
        );
    }

    #[test]
    fn kitty_with_attestation() {
        let mut ctx = OutputContextV1::default();
        ctx.is_direct_tty = true;
        ctx.output_kind = OutputKind::Tty;
        ctx.trusted_capability = TrustedCapability::KittyDirect;
        let (d, r) = decide_presentation(&ctx);
        assert_eq!(d, PresentationDecision::KittyPng);
        assert_eq!(r, DecisionReason::KittyAuthorized);
    }

    #[test]
    fn pipe_is_semantic() {
        let mut ctx = OutputContextV1::default();
        ctx.is_direct_tty = false;
        ctx.output_kind = OutputKind::Pipe;
        ctx.trusted_capability = TrustedCapability::KittyDirect;
        let (d, _) = decide_presentation(&ctx);
        assert_eq!(d, PresentationDecision::SemanticOnly);
    }
}
