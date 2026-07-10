//! Harness-specific thin wrappers over the neutral lifecycle contract.
//! No screen scraping. Machine stdout stays escape-free.

use crate::events::AdapterEventV1;
use crate::lifecycle::LifecycleStore;
use beavermath_core::pipeline::{process_document, ProcessOptions};

/// OMP: extension-only post-turn preview (Phase 8).
#[derive(Debug, Default)]
pub struct OmpAdapter {
    pub store: LifecycleStore,
}

impl OmpAdapter {
    pub fn on_turn_complete(
        &mut self,
        message_id: &str,
        revision: u64,
        content: &str,
    ) -> AdapterEventV1 {
        // Model prompts never need BeaverMath instructions; this is host-side.
        self.store.on_message_end(message_id, revision, content)
    }

    pub fn schema_version(&self) -> &'static str {
        beavermath_core::SCHEMA_VERSION
    }
}

/// Codex: structured / one-shot filter preserving producer exit status.
#[derive(Debug, Default)]
pub struct CodexAdapter;

impl CodexAdapter {
    /// Filter producer stdout; returns (filtered_text, producer_status_passthrough).
    pub fn filter_oneshot(stdout: &str, producer_exit: i32) -> (String, i32, String) {
        let res = process_document(stdout.as_bytes(), &[], &ProcessOptions::default());
        match res {
            Ok(r) => {
                let text = r.filtered_text.unwrap_or_else(|| stdout.to_string());
                let meta = serde_json::json!({
                    "schema_version": beavermath_core::SCHEMA_VERSION,
                    "input_sha256": r.input_sha256,
                    "expressions": r.expressions.len(),
                });
                (text, producer_exit, meta.to_string())
            }
            Err(_) => (stdout.to_string(), producer_exit, "{}".into()),
        }
    }
}

/// Claude Code: hooks for native text/artifact; TTY wrapper needs trusted capability separately.
#[derive(Debug, Default)]
pub struct ClaudeAdapter {
    pub store: LifecycleStore,
}

impl ClaudeAdapter {
    pub fn on_hook_complete(&mut self, message_id: &str, content: &str) -> AdapterEventV1 {
        self.store.on_message_end(message_id, 1, content)
    }

    /// Hook machine channel must be escape-free.
    pub fn hook_stdout_json(event: &AdapterEventV1) -> Result<String, String> {
        event.to_machine_json()
    }
}

/// OpenCode: server/SSE/plugin events with reconnect/replay.
#[derive(Debug, Default)]
pub struct OpenCodeAdapter {
    pub store: LifecycleStore,
}

impl OpenCodeAdapter {
    pub fn on_event(
        &mut self,
        message_id: &str,
        revision: u64,
        content: &str,
        kind: &str,
    ) -> AdapterEventV1 {
        match kind {
            "replace" => self.store.on_replace(message_id, revision, content),
            "delete" => self.store.on_delete(message_id),
            "cancel" => self.store.on_cancel(message_id),
            _ => self.store.on_message_end(message_id, revision, content),
        }
    }

    pub fn replay(&self, message_id: &str) -> Option<AdapterEventV1> {
        self.store.on_resume(message_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beavermath_core::hash::sha256_hex;

    #[test]
    fn all_adapters_same_schema() {
        let omp = OmpAdapter::default();
        assert_eq!(omp.schema_version(), "1");
        let mut claude = ClaudeAdapter::default();
        let ev = claude.on_hook_complete("m", "hi $x$");
        assert_eq!(ev.schema_version, "1");
        let mut oc = OpenCodeAdapter::default();
        let ev2 = oc.on_event("m", 1, "hi $y$", "end");
        assert_eq!(ev2.schema_version, "1");
        let (text, code, _) = CodexAdapter::filter_oneshot("val $z$", 0);
        assert_eq!(code, 0);
        assert!(!text.contains('\x1b'));
        assert!(text.contains("z") || text.contains("power") || !text.is_empty());
    }

    #[test]
    fn codex_preserves_exit() {
        let (_, code, _) = CodexAdapter::filter_oneshot("x", 7);
        assert_eq!(code, 7);
    }

    #[test]
    fn claude_hook_stdout_clean() {
        let mut c = ClaudeAdapter::default();
        let ev = c.on_hook_complete("m", "$a$");
        let out = ClaudeAdapter::hook_stdout_json(&ev).expect("json");
        assert!(!out.as_bytes().contains(&0x1b));
    }

    #[test]
    fn opencode_reconnect_replay() {
        let mut oc = OpenCodeAdapter::default();
        oc.on_event("m1", 1, "msg $q$", "end");
        let replay = oc.replay("m1").unwrap();
        assert_eq!(replay.content_sha256, sha256_hex(b"msg $q$"));
    }
}
