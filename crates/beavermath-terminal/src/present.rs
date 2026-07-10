//! Presentation emission: semantic text first, optional typed pixel frames.

use crate::decision::PresentationDecision;
use crate::iterm2::Iterm2Frame;
use crate::kitty::KittyFrame;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicsEmit {
    pub protocol: &'static str,
    pub bytes: Vec<u8>,
    pub image_id: Option<u32>,
}

/// Encode a PNG into a typed terminal graphics frame when the decision allows pixels.
/// Returns None for semantic/unicode-only decisions (no protocol bytes).
pub fn encode_graphics_for_png(
    png: &[u8],
    decision: PresentationDecision,
    image_id: u32,
) -> Result<Option<GraphicsEmit>, String> {
    match decision {
        PresentationDecision::KittyPng => {
            let frame = KittyFrame::encode_png_direct(png, image_id)?;
            Ok(Some(GraphicsEmit {
                protocol: "kitty_direct",
                bytes: frame.as_bytes().to_vec(),
                image_id: Some(image_id),
            }))
        }
        PresentationDecision::Iterm2Png => {
            let frame = Iterm2Frame::encode_inline_png(png)?;
            Ok(Some(GraphicsEmit {
                protocol: "iterm2_inline",
                bytes: frame.as_bytes().to_vec(),
                image_id: None,
            }))
        }
        PresentationDecision::SemanticOnly | PresentationDecision::VisualUnicode => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::escape::scan_machine_channel;
    use beavermath_core::types::{
        OutputContextV1, OutputKind, TrustedCapability,
    };
    use crate::decision::{decide_presentation, graphics_preference_allows};

    #[test]
    fn kitty_path_emits_typed_frame() {
        let png = b"\x89PNG\r\n\x1a\nfake";
        let g = encode_graphics_for_png(png, PresentationDecision::KittyPng, 9)
            .unwrap()
            .expect("frame");
        assert_eq!(g.protocol, "kitty_direct");
        let s = String::from_utf8_lossy(&g.bytes);
        assert!(s.contains("t=d"));
        assert!(s.contains("i=9"));
        assert!(!s.contains("q=1"));
    }

    #[test]
    fn iterm_path_emits_inline() {
        let png = b"\x89PNG\r\n\x1a\nxx";
        let g = encode_graphics_for_png(png, PresentationDecision::Iterm2Png, 1)
            .unwrap()
            .expect("frame");
        assert_eq!(g.protocol, "iterm2_inline");
        assert!(g.bytes.starts_with(b"\x1b]1337"));
    }

    #[test]
    fn semantic_decision_emits_nothing() {
        let g = encode_graphics_for_png(b"\x89PNG", PresentationDecision::SemanticOnly, 1)
            .unwrap();
        assert!(g.is_none());
    }

    #[test]
    fn named_graphics_without_capability_no_pixels() {
        let mut ctx = OutputContextV1::default();
        ctx.is_direct_tty = true;
        ctx.output_kind = OutputKind::Tty;
        ctx.trusted_capability = TrustedCapability::None;
        let dec = graphics_preference_allows(Some("kitty"), &ctx);
        assert_eq!(dec, PresentationDecision::SemanticOnly);
        assert!(encode_graphics_for_png(b"\x89PNG", dec, 1).unwrap().is_none());
    }

    #[test]
    fn trusted_kitty_on_tty_authorizes() {
        let mut ctx = OutputContextV1::default();
        ctx.is_direct_tty = true;
        ctx.output_kind = OutputKind::Tty;
        ctx.trusted_capability = TrustedCapability::KittyDirect;
        let (dec, _) = decide_presentation(&ctx);
        assert_eq!(dec, PresentationDecision::KittyPng);
        let g = encode_graphics_for_png(b"\x89PNG", dec, 2).unwrap().unwrap();
        // Protocol frames intentionally contain ESC; machine channels must not use this path
        assert!(!scan_machine_channel(&g.bytes).clean);
    }
}
