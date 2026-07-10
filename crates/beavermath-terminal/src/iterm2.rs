//! Direct iTerm2 inline PNG via OSC 1337. Append-only; no download / user filename.

use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iterm2Frame {
    bytes: Vec<u8>,
}

impl Iterm2Frame {
    /// OSC 1337 File= inline=1 with fixed trusted metadata only.
    pub fn encode_inline_png(png: &[u8]) -> Result<Self, String> {
        if png.is_empty() {
            return Err("empty png".into());
        }
        let limits = beavermath_core::LimitsV1::default();
        if png.len() > limits.max_encoded_artifact_bytes {
            return Err("payload too large".into());
        }
        let b64 = STANDARD.encode(png);
        // No name= from user; size= is length of raw file; inline=1
        let body = format!(
            "\x1b]1337;File=inline=1;size={};width=auto;height=auto;preserveAspectRatio=1:{}\x07",
            png.len(),
            b64
        );
        // ST alternative would be ESC \, we use BEL termination as classic iTerm2
        if body.contains("name=") {
            return Err("user-controlled filename forbidden".into());
        }
        if body.contains("http://") || body.contains("https://") {
            return Err("urls forbidden".into());
        }
        Ok(Self {
            bytes: body.into_bytes(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_png_frame() {
        let f = Iterm2Frame::encode_inline_png(b"\x89PNG\r\n\x1a\nxx").unwrap();
        let s = String::from_utf8_lossy(f.as_bytes());
        assert!(s.starts_with("\x1b]1337;File=inline=1"));
        assert!(!s.contains("name="));
        assert!(s.ends_with('\u{0007}') || s.as_bytes().ends_with(&[0x07]));
    }

    #[test]
    fn rejects_empty() {
        assert!(Iterm2Frame::encode_inline_png(b"").is_err());
    }
}
