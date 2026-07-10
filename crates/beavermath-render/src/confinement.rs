//! Threat-envelope helpers: hostile TeX must not trigger external I/O in our adapter path.

use crate::ratex_adapter::{render_svg, SupportedTeX};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostileCheck {
    pub tex: String,
    pub rejected: bool,
    pub command: Option<String>,
}

/// Assert closed-command rejection for the hostile corpus entries we ship.
pub fn assert_no_hostile_side_effects(tex: &str) -> HostileCheck {
    let cmd = SupportedTeX::is_rejected(tex).map(|s| s.to_string());
    let rejected = cmd.is_some();
    // Attempting render must fail closed without panic
    if rejected {
        let _ = render_svg(tex);
    }
    HostileCheck {
        tex: tex.to_string(),
        rejected,
        command: cmd,
    }
}

pub const HOSTILE_CORPUS: &[&str] = &[
    "\\input{/etc/passwd}",
    "\\write18{id}",
    "\\usepackage{tikz}",
    "\\href{http://evil}{x}",
    "\\includegraphics{x.png}",
    "\\catcode`\\$=12",
    "\\def\\foo{}",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_hostile_rejected() {
        for t in HOSTILE_CORPUS {
            let c = assert_no_hostile_side_effects(t);
            assert!(c.rejected, "should reject {t}");
            let err = render_svg(t).unwrap_err();
            assert!(err.contains("unsupported"), "{t}: {err}");
        }
    }
}
