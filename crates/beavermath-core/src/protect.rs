//! Default host-side protection for code spans and URLs (FR-007, FR-008).
//! Host-supplied ranges remain authoritative and are merged with these.

use crate::scanner::ProtectedRange;

/// Detect inline code, non-math fenced code, and URL byte ranges that must not
/// participate in math discovery.
pub fn default_protected_ranges(input: &str) -> Vec<ProtectedRange> {
    let mut ranges = Vec::new();
    protect_urls(input, &mut ranges);
    protect_fenced_non_math(input, &mut ranges);
    protect_inline_code(input, &mut ranges);
    normalize_ranges(ranges, input.len())
}

/// Merge host ranges with defaults. Host ranges win on conflicts (union).
pub fn merge_protected(input: &str, host: &[ProtectedRange]) -> Vec<ProtectedRange> {
    let mut ranges = default_protected_ranges(input);
    ranges.extend(host.iter().copied());
    normalize_ranges(ranges, input.len())
}

fn normalize_ranges(mut ranges: Vec<ProtectedRange>, len: usize) -> Vec<ProtectedRange> {
    ranges.retain(|r| r.start <= r.end && r.end <= len && r.start < r.end);
    ranges.sort_by_key(|r| (r.start, r.end));
    let mut out: Vec<ProtectedRange> = Vec::new();
    for r in ranges {
        if let Some(last) = out.last_mut() {
            if r.start <= last.end {
                last.end = last.end.max(r.end);
                continue;
            }
        }
        out.push(r);
    }
    out
}

fn protect_urls(input: &str, ranges: &mut Vec<ProtectedRange>) {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        // Only index str on char boundaries (multi-byte UTF-8 appears in session text).
        if !input.is_char_boundary(i) {
            i += 1;
            continue;
        }
        let rest = &input[i..];
        let scheme = if rest.starts_with("https://") {
            Some(8)
        } else if rest.starts_with("http://") {
            Some(7)
        } else if rest.starts_with("ftp://") {
            Some(6)
        } else {
            None
        };
        if let Some(_scheme_len) = scheme {
            let start = i;
            i += 1;
            while i < bytes.len() {
                let b = bytes[i];
                // URL continues until whitespace, angle brackets, or quotes
                if b.is_ascii_whitespace()
                    || b == b'<'
                    || b == b'>'
                    || b == b'"'
                    || b == b'\''
                    || b == b')'
                    || b == b']'
                    || b == b'`'
                {
                    break;
                }
                i += 1;
            }
            let mut end = i;
            while end > start {
                match bytes[end - 1] {
                    b'.' | b',' | b';' | b':' | b'!' | b'?' => end -= 1,
                    _ => break,
                }
            }
            // Snap to char boundaries for safety
            while end > start && !input.is_char_boundary(end) {
                end -= 1;
            }
            if end > start {
                ranges.push(ProtectedRange { start, end });
            }
            continue;
        }
        i += 1;
    }
}

fn protect_inline_code(input: &str, ranges: &mut Vec<ProtectedRange>) {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        // Skip triple fences (handled separately)
        if i + 2 < bytes.len() && &bytes[i..i + 3] == b"```" {
            i += 3;
            continue;
        }
        if bytes[i] == b'`' {
            // Count run of backticks
            let mut n = 0usize;
            while i + n < bytes.len() && bytes[i + n] == b'`' {
                n += 1;
            }
            if n >= 3 {
                i += n;
                continue;
            }
            let start = i;
            let open = n;
            i += open;
            // Find matching close of same length
            while i < bytes.len() {
                if bytes[i] == b'`' {
                    let mut m = 0usize;
                    while i + m < bytes.len() && bytes[i + m] == b'`' {
                        m += 1;
                    }
                    if m == open {
                        let end = i + m;
                        ranges.push(ProtectedRange { start, end });
                        i = end;
                        break;
                    }
                    i += m;
                } else if bytes[i] == b'\n' {
                    // single-line inline code: abandon unclosed
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        i += 1;
    }
}

fn protect_fenced_non_math(input: &str, ranges: &mut Vec<ProtectedRange>) {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i + 3 <= bytes.len() {
        if &bytes[i..i + 3] != b"```" {
            i += 1;
            continue;
        }
        let fence_start = i;
        let mut line_end = i + 3;
        while line_end < bytes.len() && bytes[line_end] != b'\n' {
            line_end += 1;
        }
        let tag = input[i + 3..line_end].trim().to_ascii_lowercase();
        let is_math = tag == "math" || tag == "latex" || tag == "tex" || tag.is_empty() && false;
        // empty tag or other languages = protected code; math/latex/tex left for discovery
        if tag == "math" || tag == "latex" || tag == "tex" {
            // skip past this fence opener; discovery owns it
            i = line_end;
            continue;
        }
        // Find closing ```
        let search_from = if line_end < bytes.len() && bytes[line_end] == b'\n' {
            line_end + 1
        } else {
            line_end
        };
        if let Some(rel) = input[search_from..].find("\n```") {
            let close = search_from + rel + 1; // points at ```
            let full_end = close + 3;
            if !is_math {
                ranges.push(ProtectedRange {
                    start: fence_start,
                    end: full_end.min(bytes.len()),
                });
            }
            i = full_end;
        } else {
            i = line_end.max(i + 3);
        }
    }
}

/// True if a byte offset sits inside any protected range.
pub fn offset_protected(offset: usize, ranges: &[ProtectedRange]) -> bool {
    ranges.iter().any(|r| offset >= r.start && offset < r.end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::limits::LimitsV1;
    use crate::scanner::discover_strict;

    #[test]
    fn protects_url_dollar_pairs() {
        let input = "see https://x.com/$user$/profile and $a$";
        let prot = default_protected_ranges(input);
        let (spans, _) = discover_strict(input, &prot, &LimitsV1::default()).unwrap();
        assert_eq!(spans.len(), 1);
        assert_eq!(&input[spans[0].inner_range.start..spans[0].inner_range.end], "a");
    }

    #[test]
    fn protects_inline_code() {
        let input = "code `$x$` and real $y$";
        let prot = default_protected_ranges(input);
        let (spans, _) = discover_strict(input, &prot, &LimitsV1::default()).unwrap();
        assert_eq!(spans.len(), 1);
        assert_eq!(&input[spans[0].inner_range.start..spans[0].inner_range.end], "y");
    }

    #[test]
    fn protects_fenced_rust_not_math() {
        let input = "```rust\nlet x = $a$;\n```\nreal $b$";
        let prot = default_protected_ranges(input);
        let (spans, _) = discover_strict(input, &prot, &LimitsV1::default()).unwrap();
        assert_eq!(spans.len(), 1);
        assert_eq!(&input[spans[0].inner_range.start..spans[0].inner_range.end], "b");
    }
}
