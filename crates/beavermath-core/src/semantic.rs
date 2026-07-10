//! Deterministic English semantic text for supported TeX fragments.
//!
//! This is a BeaverMath-owned linearization. It does not invent unsupported meaning.
//! When structure cannot be established, falls back to literal source with status.

use crate::types::DisplayMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticResult {
    pub text: String,
    pub locale: String,
    pub status: String,
    pub alt_text: String,
    pub alt_status: String,
    pub ascii: Option<String>,
    pub unicode: Option<String>,
}

/// Visibly escape terminal controls and bidi overrides for display; exact source unchanged.
pub fn source_display(exact: &str) -> String {
    let mut out = String::with_capacity(exact.len());
    for ch in exact.chars() {
        match ch {
            '\u{0000}'..='\u{0008}'
            | '\u{000B}'
            | '\u{000C}'
            | '\u{000E}'..='\u{001F}'
            | '\u{007F}'..='\u{009F}' => {
                out.push_str(&format!("\\u{{{:04X}}}", ch as u32));
            }
            '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' => {
                out.push_str(&format!("\\u{{{:04X}}}", ch as u32));
            }
            c => out.push(c),
        }
    }
    out
}

/// Produce semantic/alt/visual text for inner TeX (no delimiters).
pub fn semantic_for_tex(inner: &str, mode: DisplayMode) -> SemanticResult {
    let linearized = linearize(inner.trim());
    let status = if linearized.confident {
        "ok"
    } else {
        "literal_fallback"
    };
    let text = if linearized.confident {
        linearized.text.clone()
    } else {
        format!("math: {}", inner.trim())
    };
    let alt = text.clone();
    let ascii = Some(to_ascii_approx(inner.trim()));
    let unicode = Some(to_unicode_approx(inner.trim()));
    let _ = mode;
    SemanticResult {
        text,
        locale: "en".into(),
        status: status.into(),
        alt_text: alt,
        alt_status: status.into(),
        ascii,
        unicode,
    }
}

struct Lin {
    text: String,
    confident: bool,
}

fn linearize(tex: &str) -> Lin {
    if tex.is_empty() {
        return Lin {
            text: "empty math".into(),
            confident: true,
        };
    }

    // Split top-level equals / relations for reading order
    if let Some(parts) = split_top_level(tex, '=') {
        if parts.len() == 2 {
            let l = linearize(parts[0].trim());
            let r = linearize(parts[1].trim());
            return Lin {
                text: format!("{} equals {}", l.text, r.text),
                confident: l.confident && r.confident,
            };
        }
    }
    if let Some(parts) = split_top_level(tex, '+') {
        if parts.len() > 1 {
            let mut confident = true;
            let texts: Vec<String> = parts
                .iter()
                .map(|p| {
                    let l = linearize(p.trim());
                    confident &= l.confident;
                    l.text
                })
                .collect();
            return Lin {
                text: texts.join(" plus "),
                confident,
            };
        }
    }

    // Fraction \frac{a}{b}
    if let Some(rest) = tex.strip_prefix("\\frac") {
        if let Some((num, den, rem)) = parse_two_braced(rest) {
            if rem.trim().is_empty() {
                let n = linearize(num);
                let d = linearize(den);
                return Lin {
                    text: format!("({}) over ({})", n.text, d.text),
                    confident: n.confident && d.confident,
                };
            }
        }
    }

    // sqrt
    if let Some(rest) = tex.strip_prefix("\\sqrt") {
        if let Some((body, rem)) = parse_one_braced(rest.trim_start()) {
            if rem.trim().is_empty() {
                let b = linearize(body);
                return Lin {
                    text: format!("square root of ({})", b.text),
                    confident: b.confident,
                };
            }
        }
    }

    // Simple superscript x^2 or x^{10}
    if let Some(idx) = find_top_level_caret(tex) {
        let base = &tex[..idx];
        let rest = &tex[idx + 1..];
        let (exp, rem) = if rest.starts_with('{') {
            match parse_one_braced(rest) {
                Some((e, r)) => (e.to_string(), r),
                None => {
                    return Lin {
                        text: tex.to_string(),
                        confident: false,
                    }
                }
            }
        } else if let Some(ch) = rest.chars().next() {
            (ch.to_string(), &rest[ch.len_utf8()..])
        } else {
            return Lin {
                text: tex.to_string(),
                confident: false,
            };
        };
        if rem.trim().is_empty() && !base.is_empty() {
            let b = linearize(base);
            let e = linearize(&exp);
            return Lin {
                text: format!("({}) to the power of ({})", b.text, e.text),
                confident: b.confident && e.confident,
            };
        }
    }

    // Subscript
    if let Some(idx) = find_top_level_underscore(tex) {
        let base = &tex[..idx];
        let rest = &tex[idx + 1..];
        let (sub, rem) = if rest.starts_with('{') {
            match parse_one_braced(rest) {
                Some((e, r)) => (e.to_string(), r),
                None => {
                    return Lin {
                        text: tex.to_string(),
                        confident: false,
                    }
                }
            }
        } else if let Some(ch) = rest.chars().next() {
            (ch.to_string(), &rest[ch.len_utf8()..])
        } else {
            return Lin {
                text: tex.to_string(),
                confident: false,
            };
        };
        if rem.trim().is_empty() && !base.is_empty() {
            let b = linearize(base);
            let s = linearize(&sub);
            return Lin {
                text: format!("({}) sub ({})", b.text, s.text),
                confident: b.confident && s.confident,
            };
        }
    }

    // Sum/int with optional bounds left as structure markers
    for (cmd, word) in [
        ("\\sum", "sum"),
        ("\\int", "integral"),
        ("\\prod", "product"),
    ] {
        if tex.starts_with(cmd) {
            return Lin {
                text: format!("{word} expression {tex}"),
                confident: true,
            };
        }
    }

    // Greek / common commands
    let replaced = replace_common_commands(tex);
    let confident = !replaced.contains('\\');
    Lin {
        text: replaced,
        confident,
    }
}

fn replace_common_commands(tex: &str) -> String {
    let mut s = tex.to_string();
    let pairs = [
        ("\\alpha", "alpha"),
        ("\\beta", "beta"),
        ("\\gamma", "gamma"),
        ("\\delta", "delta"),
        ("\\epsilon", "epsilon"),
        ("\\theta", "theta"),
        ("\\lambda", "lambda"),
        ("\\mu", "mu"),
        ("\\pi", "pi"),
        ("\\sigma", "sigma"),
        ("\\phi", "phi"),
        ("\\omega", "omega"),
        ("\\infty", "infinity"),
        ("\\pm", "plus or minus"),
        ("\\times", "times"),
        ("\\cdot", "dot"),
        ("\\leq", "less than or equal to"),
        ("\\geq", "greater than or equal to"),
        ("\\neq", "not equal to"),
        ("\\approx", "approximately"),
        ("\\rightarrow", "right arrow"),
        ("\\leftarrow", "left arrow"),
        ("\\left", ""),
        ("\\right", ""),
        ("\\,", " "),
        ("\\;", " "),
        ("\\!", ""),
        ("\\ ", " "),
    ];
    for (a, b) in pairs {
        s = s.replace(a, b);
    }
    s = s.replace('{', "").replace('}', "");
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_one_braced(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();
    if !s.starts_with('{') {
        return None;
    }
    let mut depth = 0i32;
    for (i, ch) in s.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((&s[1..i], &s[i + 1..]));
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_two_braced(s: &str) -> Option<(&str, &str, &str)> {
    let (a, rest) = parse_one_braced(s)?;
    let (b, rest2) = parse_one_braced(rest)?;
    Some((a, b, rest2))
}

fn split_top_level(tex: &str, sep: char) -> Option<Vec<&str>> {
    let mut depth = 0i32;
    let mut last = 0usize;
    let mut parts = Vec::new();
    for (i, ch) in tex.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            c if c == sep && depth == 0 => {
                parts.push(&tex[last..i]);
                last = i + ch.len_utf8();
            }
            _ => {}
        }
    }
    if parts.is_empty() {
        return None;
    }
    parts.push(&tex[last..]);
    Some(parts)
}

fn find_top_level_caret(tex: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (i, ch) in tex.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            '^' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

fn find_top_level_underscore(tex: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (i, ch) in tex.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            '_' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

fn to_ascii_approx(tex: &str) -> String {
    let s = replace_common_commands(tex)
        .replace('^', "^")
        .chars()
        .filter(|c| c.is_ascii() && !c.is_control())
        .collect::<String>();
    s
}

fn to_unicode_approx(tex: &str) -> String {
    let mut s = tex.to_string();
    for (a, b) in [
        ("\\alpha", "α"),
        ("\\beta", "β"),
        ("\\gamma", "γ"),
        ("\\pi", "π"),
        ("\\infty", "∞"),
        ("\\times", "×"),
        ("\\cdot", "·"),
        ("\\leq", "≤"),
        ("\\geq", "≥"),
        ("\\neq", "≠"),
        ("\\pm", "±"),
        ("\\rightarrow", "→"),
    ] {
        s = s.replace(a, b);
    }
    s.replace('{', "").replace('}', "").replace("\\", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frac_semantic() {
        let r = semantic_for_tex("\\frac{a}{b}", DisplayMode::Inline);
        assert!(r.text.contains("over"));
        assert_eq!(r.status, "ok");
        assert!(!r.alt_text.is_empty());
    }

    #[test]
    fn power_semantic() {
        let r = semantic_for_tex("x^2", DisplayMode::Inline);
        assert!(r.text.contains("power"));
    }

    #[test]
    fn source_display_escapes_esc() {
        let s = source_display("a\x1b[31mx");
        assert!(!s.contains('\x1b'));
        assert!(s.contains("001B"));
    }

    #[test]
    fn semantic_and_alt_never_both_absent() {
        let r = semantic_for_tex("\\unknowncmd", DisplayMode::Block);
        assert!(!r.text.is_empty());
        assert!(!r.alt_text.is_empty());
    }
}
