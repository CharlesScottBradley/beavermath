//! RaTeX behind a BeaverMath-owned adapter. Upstream types never leave this module.

use ratex_layout::{layout, to_display_list, LayoutOptions};
use ratex_parser::parse;
use ratex_render::{render_to_png, RenderOptions};
use ratex_svg::{render_to_svg, SvgOptions};
use ratex_types::math_style::MathStyle;
use std::sync::OnceLock;

pub const RENDERER_VERSION: &str = "ratex-0.1.13";

/// Closed supported-command notes for v1 (see corpus/SUPPORTED_TEX.md).
pub struct SupportedTeX;

impl SupportedTeX {
    pub const REJECT: &'static [&'static str] = &[
        "\\input",
        "\\include",
        "\\write18",
        "\\openin",
        "\\openout",
        "\\usepackage",
        "\\href",
        "\\url",
        "\\includegraphics",
        "\\catcode",
        "\\def",
        "\\gdef",
        "\\edef",
        "\\let",
        "\\command",
        "\\special",
    ];

    pub fn is_rejected(tex: &str) -> Option<&'static str> {
        for cmd in Self::REJECT {
            if tex.contains(cmd) {
                return Some(*cmd);
            }
        }
        None
    }
}

fn config_fingerprint() -> &'static str {
    static F: OnceLock<String> = OnceLock::new();
    F.get_or_init(|| format!("{RENDERER_VERSION}|font=embed-katex|svg=standalone"))
        .as_str()
}

pub fn renderer_config_id() -> &'static str {
    config_fingerprint()
}

pub fn render_svg(tex: &str) -> Result<String, String> {
    if let Some(cmd) = SupportedTeX::is_rejected(tex) {
        return Err(format!("unsupported command {cmd}"));
    }
    let nodes = parse(tex).map_err(|e| format!("parse: {e}"))?;
    let opts = LayoutOptions {
        style: MathStyle::Display,
        ..LayoutOptions::default()
    };
    let lay = layout(&nodes, &opts);
    let list = to_display_list(&lay);
    let svg_opts = SvgOptions {
        embed_glyphs: true,
        font_size: 40.0,
        ..SvgOptions::default()
    };
    let svg = render_to_svg(&list, &svg_opts);
    // Reject active content
    if svg.contains("<script") || svg.contains("javascript:") || svg.contains("xlink:href") {
        return Err("svg failed safety validation".into());
    }
    Ok(svg)
}

pub fn render_png(tex: &str) -> Result<(Vec<u8>, u32, u32), String> {
    if let Some(cmd) = SupportedTeX::is_rejected(tex) {
        return Err(format!("unsupported command {cmd}"));
    }
    let nodes = parse(tex).map_err(|e| format!("parse: {e}"))?;
    let opts = LayoutOptions {
        style: MathStyle::Display,
        ..LayoutOptions::default()
    };
    let lay = layout(&nodes, &opts);
    let list = to_display_list(&lay);
    let ro = RenderOptions {
        font_size: 40.0,
        padding: 10.0,
        ..RenderOptions::default()
    };
    let png = render_to_png(&list, &ro)?;
    // Parse IHDR for dimensions
    let (w, h) = png_dimensions(&png).unwrap_or((0, 0));
    let limits = beavermath_core::LimitsV1::default();
    if w > limits.max_axis_px || h > limits.max_axis_px {
        return Err("raster axis limit exceeded".into());
    }
    if (w as u64) * (h as u64) > limits.max_pixels {
        return Err("pixel limit exceeded".into());
    }
    if png.len() > limits.max_encoded_artifact_bytes {
        return Err("encoded artifact limit exceeded".into());
    }
    Ok((png, w, h))
}

fn png_dimensions(png: &[u8]) -> Option<(u32, u32)> {
    // PNG signature 8 bytes, then IHDR chunk: len(4) "IHDR"(4) width(4) height(4)
    if png.len() < 24 {
        return None;
    }
    if &png[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let w = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
    let h = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
    Some((w, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_input_command() {
        let err = render_svg("\\input{x}").unwrap_err();
        assert!(err.contains("unsupported"));
    }

    #[test]
    fn renders_simple_svg() {
        let svg = render_svg("x^2").expect("svg");
        assert!(svg.contains("<svg") || svg.contains("svg"));
        assert!(!svg.contains("<script"));
    }

    #[test]
    fn renders_simple_png() {
        let (png, w, h) = render_png("a+b").expect("png");
        assert!(png.starts_with(b"\x89PNG"));
        assert!(w > 0 && h > 0);
    }

    #[test]
    fn deterministic_svg_digest() {
        use beavermath_core::hash::sha256_hex;
        let a = render_svg("x+y").unwrap();
        let b = render_svg("x+y").unwrap();
        assert_eq!(sha256_hex(a.as_bytes()), sha256_hex(b.as_bytes()));
    }
}
