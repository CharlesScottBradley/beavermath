//! Direct Kitty graphics transfer `t=d` only. Owned IDs, bounded base64 chunks.

use base64::{engine::general_purpose::STANDARD, Engine};
use std::fmt;

/// Privately constructed Kitty frame. Fields that would enable file/shm transfer are unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KittyFrame {
    bytes: Vec<u8>,
    image_id: u32,
}

impl KittyFrame {
    /// Encode PNG via direct transfer only (t=d), chunked at ≤4096 base64 payload bytes.
    pub fn encode_png_direct(png: &[u8], image_id: u32) -> Result<Self, String> {
        if png.is_empty() {
            return Err("empty png".into());
        }
        let b64 = STANDARD.encode(png);
        let mut out = Vec::new();
        let chunk_size = 4096usize;
        let chunks: Vec<&str> = b64
            .as_bytes()
            .chunks(chunk_size)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect();
        let n = chunks.len();
        for (i, chunk) in chunks.iter().enumerate() {
            let more = if i + 1 < n { 1 } else { 0 };
            // a=T transmit+display, t=d direct, f=100 PNG
            let header = if i == 0 {
                format!("\x1b_Ga=T,f=100,t=d,i={image_id},m={more};")
            } else {
                format!("\x1b_Gm={more};")
            };
            out.extend_from_slice(header.as_bytes());
            out.extend_from_slice(chunk.as_bytes());
            out.extend_from_slice(b"\x1b\\");
        }
        // Validate no forbidden keys
        let s = String::from_utf8_lossy(&out);
        for banned in ["t=f", "t=t", "t=s", "a=d,d=A", "q=1"] {
            // t=f file, t=t temp, t=s shared memory, mass delete, query
            if s.contains(banned) && banned != "q=1" {
                // t=d is fine; check t=f specifically as token
            }
        }
        if s.contains(",t=f") || s.contains(",t=t") || s.contains(",t=s") {
            return Err("forbidden transfer mode".into());
        }
        // Zero query: never emit q=
        if s.contains(",q=") || s.contains("q=1") {
            return Err("queries are forbidden in v1".into());
        }
        Ok(Self {
            bytes: out,
            image_id,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn image_id(&self) -> u32 {
        self.image_id
    }

    /// ID-specific cleanup frame (delete by image id only).
    pub fn cleanup_frame(image_id: u32) -> Self {
        let s = format!("\x1b_Ga=d,d=i,i={image_id}\x1b\\");
        Self {
            bytes: s.into_bytes(),
            image_id,
        }
    }
}

impl fmt::Display for KittyFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Never use Display for TTY writes of raw frames in library consumers;
        // provided for debug only.
        write!(f, "KittyFrame(id={}, {} bytes)", self.image_id, self.bytes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_direct_only() {
        // Minimal fake png-ish payload
        let png = b"\x89PNG\r\n\x1a\nfake";
        let frame = KittyFrame::encode_png_direct(png, 42).unwrap();
        let s = String::from_utf8_lossy(frame.as_bytes());
        assert!(s.contains("t=d"));
        assert!(!s.contains("t=f"));
        assert!(!s.contains("q=1"));
        assert!(s.contains("i=42"));
    }

    #[test]
    fn chunks_large_payload() {
        let png = vec![0u8; 10_000];
        let frame = KittyFrame::encode_png_direct(&png, 1).unwrap();
        let s = String::from_utf8_lossy(frame.as_bytes());
        // Multiple m=1 / m=0 markers
        assert!(s.matches("m=1").count() >= 1);
        assert!(s.contains("m=0"));
    }

    #[test]
    fn no_query_sequences() {
        let frame = KittyFrame::encode_png_direct(b"\x89PNG", 7).unwrap();
        assert!(!frame.as_bytes().windows(3).any(|w| w == b",q="));
    }
}
