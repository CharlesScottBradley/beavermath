use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HashError {
    #[error("invalid UTF-8 starting at byte offset {offset}")]
    InvalidUtf8 { offset: usize },
}

/// Validate UTF-8 and return the string slice. Reports the first invalid byte offset.
pub fn validate_utf8(bytes: &[u8]) -> Result<&str, HashError> {
    match std::str::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => Err(HashError::InvalidUtf8 {
            offset: e.valid_up_to(),
        }),
    }
}

/// SHA-256 of exact bytes as lowercase hex (64 chars).
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hash_is_stable() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn invalid_utf8_reports_offset() {
        let bytes = b"ok\xffmore";
        let err = validate_utf8(bytes).unwrap_err();
        assert_eq!(err, HashError::InvalidUtf8 { offset: 2 });
    }

    #[test]
    fn hash_before_parse_semantics() {
        let src = b"$x$";
        let h1 = sha256_hex(src);
        let h2 = sha256_hex(src);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }
}
