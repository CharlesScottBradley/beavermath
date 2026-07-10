use crate::diagnostics::{DiagnosticCode, DiagnosticSeverity, DiagnosticV1};
use crate::limits::LimitsV1;
use crate::types::{ByteRangeV1, DisplayMode};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ScanError {
    #[error("protected range invalid: {0}")]
    ProtectedRange(String),
    #[error("limit exceeded: {0}")]
    Limit(String),
}

/// A discovered explicit math span (byte ranges into original input).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathSpan {
    /// Inclusive of delimiters for dollar/paren forms; for fences, full fence block.
    pub full_range: ByteRangeV1,
    /// Inner TeX without delimiters / fence markers.
    pub inner_range: ByteRangeV1,
    pub mode: DisplayMode,
    pub delimiter: DelimiterKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelimiterKind {
    DollarInline,
    DollarDisplay,
    ParenInline,
    BracketDisplay,
    Fence,
}

fn is_protected(offset: usize, ranges: &[ProtectedRange]) -> bool {
    ranges.iter().any(|r| offset >= r.start && offset < r.end)
}

fn ranges_overlap_span(start: usize, end: usize, ranges: &[ProtectedRange]) -> bool {
    ranges
        .iter()
        .any(|r| start < r.end && end > r.start)
}

/// Validate protected ranges: ordered, in bounds, UTF-8 char boundaries.
pub fn validate_protected(
    input: &str,
    ranges: &[ProtectedRange],
) -> Result<(), ScanError> {
    let len = input.len();
    let mut prev_end = 0usize;
    for (i, r) in ranges.iter().enumerate() {
        if r.start > r.end {
            return Err(ScanError::ProtectedRange(format!(
                "range {i}: start > end"
            )));
        }
        if r.end > len {
            return Err(ScanError::ProtectedRange(format!(
                "range {i}: out of bounds"
            )));
        }
        if r.start < prev_end {
            return Err(ScanError::ProtectedRange(format!(
                "range {i}: not ordered / overlapping"
            )));
        }
        if !input.is_char_boundary(r.start) || !input.is_char_boundary(r.end) {
            return Err(ScanError::ProtectedRange(format!(
                "range {i}: not on UTF-8 boundary"
            )));
        }
        prev_end = r.end;
    }
    Ok(())
}

/// One-pass strict discovery. Unclosed delimiters remain source text (diagnostics only).
pub fn discover_strict(
    input: &str,
    protected: &[ProtectedRange],
    limits: &LimitsV1,
) -> Result<(Vec<MathSpan>, Vec<DiagnosticV1>), ScanError> {
    validate_protected(input, protected)?;
    if input.len() > limits.max_input_bytes {
        return Err(ScanError::Limit(format!(
            "input {} > max {}",
            input.len(),
            limits.max_input_bytes
        )));
    }

    let bytes = input.as_bytes();
    let mut i = 0usize;
    let mut spans = Vec::new();
    let mut diagnostics = Vec::new();

    while i < bytes.len() {
        if is_protected(i, protected) {
            // Skip to end of this protected range
            if let Some(r) = protected.iter().find(|r| i >= r.start && i < r.end) {
                i = r.end;
                continue;
            }
        }

        // Fenced ```math / ```latex / ```tex
        if bytes[i] == b'`'
            && i + 3 <= bytes.len()
            && &bytes[i..i + 3] == b"```"
        {
            if let Some(span) = try_fence(input, i, protected) {
                if span.full_range.end - span.full_range.start > limits.max_expression_bytes {
                    diagnostics.push(
                        DiagnosticV1::new(
                            DiagnosticCode::LimitExceeded,
                            DiagnosticSeverity::Error,
                            "expression exceeds max size",
                        )
                        .with_range(span.full_range.start, span.full_range.end),
                    );
                    i = span.full_range.end;
                    continue;
                }
                if spans.len() >= limits.max_expressions_per_message {
                    diagnostics.push(DiagnosticV1::new(
                        DiagnosticCode::LimitExceeded,
                        DiagnosticSeverity::Error,
                        "too many expressions",
                    ));
                    break;
                }
                spans.push(span);
                i = spans.last().unwrap().full_range.end;
                continue;
            }
        }

        // $$ display
        if bytes[i] == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'$' {
            if is_escaped_dollar(bytes, i) {
                i += 1;
                continue;
            }
            match find_close_dollar_display(bytes, i + 2) {
                Some(end_open) => {
                    let full_end = end_open + 2;
                    if ranges_overlap_span(i, full_end, protected) {
                        i += 2;
                        continue;
                    }
                    let inner = ByteRangeV1 {
                        start: i + 2,
                        end: end_open,
                    };
                    if full_end - i > limits.max_expression_bytes {
                        diagnostics.push(
                            DiagnosticV1::new(
                                DiagnosticCode::LimitExceeded,
                                DiagnosticSeverity::Error,
                                "expression exceeds max size",
                            )
                            .with_range(i, full_end),
                        );
                    } else if spans.len() < limits.max_expressions_per_message {
                        spans.push(MathSpan {
                            full_range: ByteRangeV1 {
                                start: i,
                                end: full_end,
                            },
                            inner_range: inner,
                            mode: DisplayMode::Block,
                            delimiter: DelimiterKind::DollarDisplay,
                        });
                    }
                    i = full_end;
                    continue;
                }
                None => {
                    diagnostics.push(
                        DiagnosticV1::new(
                            DiagnosticCode::UnclosedDelimiter,
                            DiagnosticSeverity::Warning,
                            "unclosed $$ display delimiter",
                        )
                        .with_range(i, i + 2),
                    );
                    i += 2;
                    continue;
                }
            }
        }

        // $ inline
        if bytes[i] == b'$' {
            if is_escaped_dollar(bytes, i) {
                i += 1;
                continue;
            }
            // currency heuristic: digit before or after lone $ often currency — still only
            // select if closed. Spec: never select currency as math when ambiguous.
            // We only match closed $...$; unclosed stays text.
            match find_close_dollar_inline(bytes, i + 1) {
                Some(end) => {
                    let full_end = end + 1;
                    if ranges_overlap_span(i, full_end, protected) {
                        i += 1;
                        continue;
                    }
                    // Reject empty $$
                    if end == i + 1 {
                        i += 1;
                        continue;
                    }
                    if looks_like_currency(bytes, i, end) {
                        i += 1;
                        continue;
                    }
                    if full_end - i > limits.max_expression_bytes {
                        diagnostics.push(
                            DiagnosticV1::new(
                                DiagnosticCode::LimitExceeded,
                                DiagnosticSeverity::Error,
                                "expression exceeds max size",
                            )
                            .with_range(i, full_end),
                        );
                    } else if spans.len() < limits.max_expressions_per_message {
                        spans.push(MathSpan {
                            full_range: ByteRangeV1 {
                                start: i,
                                end: full_end,
                            },
                            inner_range: ByteRangeV1 {
                                start: i + 1,
                                end,
                            },
                            mode: DisplayMode::Inline,
                            delimiter: DelimiterKind::DollarInline,
                        });
                    }
                    i = full_end;
                    continue;
                }
                None => {
                    diagnostics.push(
                        DiagnosticV1::new(
                            DiagnosticCode::UnclosedDelimiter,
                            DiagnosticSeverity::Warning,
                            "unclosed $ delimiter",
                        )
                        .with_range(i, i + 1),
                    );
                    i += 1;
                    continue;
                }
            }
        }

        // \( ... \)
        if bytes[i] == b'\\' && i + 1 < bytes.len() && bytes[i + 1] == b'(' {
            match find_close_paren_math(bytes, i + 2) {
                Some(end) => {
                    let full_end = end + 2;
                    if !ranges_overlap_span(i, full_end, protected) {
                        maybe_push(
                            &mut spans,
                            &mut diagnostics,
                            limits,
                            MathSpan {
                                full_range: ByteRangeV1 {
                                    start: i,
                                    end: full_end,
                                },
                                inner_range: ByteRangeV1 {
                                    start: i + 2,
                                    end,
                                },
                                mode: DisplayMode::Inline,
                                delimiter: DelimiterKind::ParenInline,
                            },
                        );
                    }
                    i = full_end;
                    continue;
                }
                None => {
                    diagnostics.push(
                        DiagnosticV1::new(
                            DiagnosticCode::UnclosedDelimiter,
                            DiagnosticSeverity::Warning,
                            "unclosed \\( delimiter",
                        )
                        .with_range(i, i + 2),
                    );
                    i += 2;
                    continue;
                }
            }
        }

        // \[ ... \]
        if bytes[i] == b'\\' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            match find_close_bracket_math(bytes, i + 2) {
                Some(end) => {
                    let full_end = end + 2;
                    if !ranges_overlap_span(i, full_end, protected) {
                        maybe_push(
                            &mut spans,
                            &mut diagnostics,
                            limits,
                            MathSpan {
                                full_range: ByteRangeV1 {
                                    start: i,
                                    end: full_end,
                                },
                                inner_range: ByteRangeV1 {
                                    start: i + 2,
                                    end,
                                },
                                mode: DisplayMode::Block,
                                delimiter: DelimiterKind::BracketDisplay,
                            },
                        );
                    }
                    i = full_end;
                    continue;
                }
                None => {
                    diagnostics.push(
                        DiagnosticV1::new(
                            DiagnosticCode::UnclosedDelimiter,
                            DiagnosticSeverity::Warning,
                            "unclosed \\[ delimiter",
                        )
                        .with_range(i, i + 2),
                    );
                    i += 2;
                    continue;
                }
            }
        }

        i += 1;
    }

    Ok((spans, diagnostics))
}

fn maybe_push(
    spans: &mut Vec<MathSpan>,
    diagnostics: &mut Vec<DiagnosticV1>,
    limits: &LimitsV1,
    span: MathSpan,
) {
    let size = span.full_range.end - span.full_range.start;
    if size > limits.max_expression_bytes {
        diagnostics.push(
            DiagnosticV1::new(
                DiagnosticCode::LimitExceeded,
                DiagnosticSeverity::Error,
                "expression exceeds max size",
            )
            .with_range(span.full_range.start, span.full_range.end),
        );
        return;
    }
    if spans.len() >= limits.max_expressions_per_message {
        diagnostics.push(DiagnosticV1::new(
            DiagnosticCode::LimitExceeded,
            DiagnosticSeverity::Error,
            "too many expressions",
        ));
        return;
    }
    spans.push(span);
}

fn is_escaped_dollar(bytes: &[u8], i: usize) -> bool {
    // Count preceding backslashes
    let mut n = 0usize;
    let mut j = i;
    while j > 0 && bytes[j - 1] == b'\\' {
        n += 1;
        j -= 1;
    }
    n % 2 == 1
}

fn find_close_dollar_display(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' && bytes[i + 1] == b'$' && !is_escaped_dollar(bytes, i) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_close_dollar_inline(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == b'$' && !is_escaped_dollar(bytes, i) {
            // Don't treat $$ as inline close
            if i + 1 < bytes.len() && bytes[i + 1] == b'$' {
                return None;
            }
            return Some(i);
        }
        // newline inside inline $ is allowed in TeX; keep simple
        i += 1;
    }
    None
}

fn find_close_paren_math(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < bytes.len() {
        if bytes[i] == b'\\' && bytes[i + 1] == b')' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_close_bracket_math(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < bytes.len() {
        if bytes[i] == b'\\' && bytes[i + 1] == b']' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn try_fence(input: &str, start: usize, protected: &[ProtectedRange]) -> Option<MathSpan> {
    let bytes = input.as_bytes();
    if start + 3 > bytes.len() || &bytes[start..start + 3] != b"```" {
        return None;
    }
    // Read language tag
    let mut line_end = start + 3;
    while line_end < bytes.len() && bytes[line_end] != b'\n' {
        line_end += 1;
    }
    let tag = input[start + 3..line_end].trim().to_ascii_lowercase();
    if tag != "math" && tag != "latex" && tag != "tex" {
        return None;
    }
    let content_start = if line_end < bytes.len() && bytes[line_end] == b'\n' {
        line_end + 1
    } else {
        line_end
    };
    // Find closing ```
    let rest = &input[content_start..];
    let close_rel = rest.find("\n```")?;
    let inner_end = content_start + close_rel;
    let full_end = inner_end + 1 + 3; // \n```
    if full_end > bytes.len() {
        return None;
    }
    if ranges_overlap_span(start, full_end, protected) {
        return None;
    }
    // Trim trailing newline from inner if present
    let mut inner_end_trim = inner_end;
    if inner_end_trim > content_start && bytes[inner_end_trim - 1] == b'\r' {
        inner_end_trim -= 1;
    }
    Some(MathSpan {
        full_range: ByteRangeV1 {
            start,
            end: full_end,
        },
        inner_range: ByteRangeV1 {
            start: content_start,
            end: inner_end_trim,
        },
        mode: DisplayMode::Block,
        delimiter: DelimiterKind::Fence,
    })
}

/// Currency and prose between dollars must not become math.
/// Examples: `$5.00`, `$12`, `$5.00 today and $` (paired across prices).
fn looks_like_currency(bytes: &[u8], open: usize, close: usize) -> bool {
    let inner = &bytes[open + 1..close];
    if inner.is_empty() {
        return false;
    }
    // Pure money amount
    let all_money = inner
        .iter()
        .all(|b| b.is_ascii_digit() || *b == b',' || *b == b'.' || *b == b' ');
    if all_money {
        return true;
    }
    // Opens like a price: $ then digit
    if inner[0].is_ascii_digit() {
        // Allow real math that starts with a number only if TeX markers present
        let has_tex = inner.contains(&b'\\')
            || inner.contains(&b'^')
            || inner.contains(&b'_')
            || inner.contains(&b'{');
        if !has_tex {
            return true;
        }
    }
    // Prose between dollars without TeX markers
    let has_space = inner.contains(&b' ');
    let has_tex = inner.contains(&b'\\')
        || inner.contains(&b'^')
        || inner.contains(&b'_')
        || inner.contains(&b'{')
        || inner.iter().any(|b| b.is_ascii_alphabetic());
    // "5.00 today and " style: spaces + no backslash commands
    if has_space && !inner.contains(&b'\\') && !inner.contains(&b'^') && !inner.contains(&b'_') {
        // If mostly digits/spaces/punctuation, currency/prose
        let mathish = inner.iter().any(|b| matches!(b, b'+' | b'=' | b'*' | b'/' | b'<' | b'>'));
        if !mathish {
            return true;
        }
    }
    let _ = has_tex;
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::limits::LimitsV1;

    #[test]
    fn discovers_inline_and_display() {
        let input = "a $x^2$ and $$y$$ done";
        let (spans, _) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].mode, DisplayMode::Inline);
        assert_eq!(&input[spans[0].inner_range.start..spans[0].inner_range.end], "x^2");
        assert_eq!(spans[1].mode, DisplayMode::Block);
    }

    #[test]
    fn paren_and_bracket() {
        let input = r"inline \(a+b\) display \[c\]";
        let (spans, _) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
        assert_eq!(spans.len(), 2);
        assert_eq!(&input[spans[0].inner_range.start..spans[0].inner_range.end], "a+b");
        assert_eq!(&input[spans[1].inner_range.start..spans[1].inner_range.end], "c");
    }

    #[test]
    fn currency_not_selected() {
        let input = "price $5.00 today";
        let (spans, _) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
        assert!(spans.is_empty());
    }

    #[test]
    fn protected_code_skipped() {
        let input = "code `$x$` and real $y$";
        let first_tick = input.find('`').unwrap();
        let second_tick = input[first_tick + 1..].find('`').unwrap() + first_tick + 1 + 1;
        let protected = [ProtectedRange {
            start: first_tick,
            end: second_tick,
        }];
        let (spans, _) = discover_strict(input, &protected, &LimitsV1::default()).unwrap();
        assert_eq!(spans.len(), 1);
        assert_eq!(&input[spans[0].inner_range.start..spans[0].inner_range.end], "y");
    }

    #[test]
    fn unclosed_stays_text() {
        let input = "oops $x and more";
        let (spans, diags) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
        assert!(spans.is_empty());
        assert!(diags.iter().any(|d| d.code == DiagnosticCode::UnclosedDelimiter));
    }

    #[test]
    fn fence_math() {
        let input = "before\n```math\nx+1\n```\nafter";
        let (spans, _) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
        assert_eq!(spans.len(), 1);
        assert_eq!(&input[spans[0].inner_range.start..spans[0].inner_range.end], "x+1");
    }

    #[test]
    fn negative_undelimited() {
        let input = "sqrt(x^2+1) and x^2 + y^2 = z^2 without dollars";
        let (spans, _) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
        assert!(spans.is_empty());
    }
}
