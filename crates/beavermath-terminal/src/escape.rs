//! Visibly escape C0/C1 controls for plain text sinks.

/// Escape untrusted controls; writer-owned LF is preserved.
pub fn escape_for_terminal_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\n' => out.push('\n'),
            '\t' => out.push('\t'),
            '\u{0000}'..='\u{001F}' | '\u{007F}'..='\u{009F}' => {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineScan {
    pub clean: bool,
    pub esc_count: usize,
    pub csi_like: usize,
    pub osc_like: usize,
    pub apc_like: usize,
}

/// Scan content for terminal protocol / control contamination.
pub fn scan_machine_channel(bytes: &[u8]) -> MachineScan {
    let mut esc_count = 0usize;
    let mut csi_like = 0usize;
    let mut osc_like = 0usize;
    let mut apc_like = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == 0x1b {
            esc_count += 1;
            if i + 1 < bytes.len() {
                match bytes[i + 1] {
                    b'[' => csi_like += 1,
                    b']' => osc_like += 1,
                    b'_' => apc_like += 1, // APC / Kitty often uses ESC _ or ESC G variants
                    b'P' => apc_like += 1, // DCS
                    _ => {}
                }
            }
        }
        // C1 CSI 0x9B, OSC 0x9D, etc.
        if b == 0x9b {
            csi_like += 1;
        }
        if b == 0x9d {
            osc_like += 1;
        }
        if b == 0x9f || b == 0x90 {
            apc_like += 1;
        }
        i += 1;
    }
    MachineScan {
        clean: esc_count == 0 && csi_like == 0 && osc_like == 0 && apc_like == 0,
        esc_count,
        csi_like,
        osc_like,
        apc_like,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_esc() {
        let s = escape_for_terminal_text("x\x1b[31my");
        assert!(!s.as_bytes().contains(&0x1b));
    }

    #[test]
    fn machine_scan_flags_osc() {
        let payload = b"hello\x1b]1337;File=x\x07";
        let scan = scan_machine_channel(payload);
        assert!(!scan.clean);
        assert!(scan.esc_count > 0);
    }

    #[test]
    fn clean_semantic() {
        let s = b"The answer is (x) to the power of (2).";
        assert!(scan_machine_channel(s).clean);
    }
}
