use crate::events::{AdapterEventV1, EventKind};
use beavermath_core::hash::sha256_hex;
use beavermath_core::pipeline::{process_document, ProcessOptions};
use beavermath_core::types::MathArtifactV1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct LifecycleStore {
    entries: HashMap<String, Entry>,
    persist_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entry {
    revision: u64,
    content_sha256: String,
    exact_source: String,
    artifacts: Vec<MathArtifactV1>,
    preview_active: bool,
    semantic_preview: Option<String>,
}

impl LifecycleStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Optional file-backed resume across process boundaries (preview metadata only).
    pub fn with_persist_dir(dir: impl Into<PathBuf>) -> std::io::Result<Self> {
        let dir = dir.into();
        std::fs::create_dir_all(&dir)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
        }
        let mut store = Self {
            entries: HashMap::new(),
            persist_dir: Some(dir.clone()),
        };
        store.load_all(&dir)?;
        Ok(store)
    }

    fn load_all(&mut self, dir: &Path) -> std::io::Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        for ent in std::fs::read_dir(dir)?.flatten() {
            let p = ent.path();
            if p.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(bytes) = std::fs::read(&p) {
                if let Ok(entry) = serde_json::from_slice::<Entry>(&bytes) {
                    if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                        self.entries.insert(stem.to_string(), entry);
                    }
                }
            }
        }
        Ok(())
    }

    fn persist_entry(&self, message_id: &str, entry: &Entry) {
        let Some(dir) = &self.persist_dir else {
            return;
        };
        let safe: String = message_id
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let path = dir.join(format!("{safe}.json"));
        if let Ok(bytes) = serde_json::to_vec_pretty(entry) {
            let _ = std::fs::write(path, bytes);
        }
    }

    fn remove_persisted(&self, message_id: &str) {
        let Some(dir) = &self.persist_dir else {
            return;
        };
        let safe: String = message_id
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let _ = std::fs::remove_file(dir.join(format!("{safe}.json")));
    }

    /// Process completed assistant content into artifacts. Source remains immutable.
    pub fn on_message_end(
        &mut self,
        message_id: &str,
        revision: u64,
        exact_content: &str,
    ) -> AdapterEventV1 {
        let digest = sha256_hex(exact_content.as_bytes());
        // Idempotent: same key returns stored event shape
        if let Some(e) = self.entries.get(message_id) {
            if e.revision == revision && e.content_sha256 == digest && e.preview_active {
                let mut ev =
                    AdapterEventV1::new(EventKind::MessageEnd, message_id, revision, digest);
                ev.artifacts = e.artifacts.clone();
                ev.semantic_preview = e.semantic_preview.clone();
                return ev;
            }
        }

        let result =
            process_document(exact_content.as_bytes(), &[], &ProcessOptions::default()).ok();
        let artifacts = result
            .as_ref()
            .map(|r| r.expressions.clone())
            .unwrap_or_default();
        let preview = result.and_then(|r| r.filtered_text);

        let entry = Entry {
            revision,
            content_sha256: digest.clone(),
            exact_source: exact_content.to_string(),
            artifacts: artifacts.clone(),
            preview_active: true,
            semantic_preview: preview.clone(),
        };
        self.persist_entry(message_id, &entry);
        self.entries.insert(message_id.to_string(), entry);

        let mut ev = AdapterEventV1::new(EventKind::MessageEnd, message_id, revision, digest);
        ev.artifacts = artifacts;
        ev.semantic_preview = preview;
        ev
    }

    pub fn on_replace(
        &mut self,
        message_id: &str,
        revision: u64,
        exact_content: &str,
    ) -> AdapterEventV1 {
        // Force recompute even if content matches
        self.entries.remove(message_id);
        let mut ev = self.on_message_end(message_id, revision, exact_content);
        ev.kind = EventKind::Replace;
        ev
    }

    pub fn on_delete(&mut self, message_id: &str) -> AdapterEventV1 {
        let digest = self
            .entries
            .remove(message_id)
            .map(|e| e.content_sha256)
            .unwrap_or_else(|| sha256_hex(b""));
        self.remove_persisted(message_id);
        AdapterEventV1::new(EventKind::Delete, message_id, 0, digest)
    }

    pub fn on_resume(&self, message_id: &str) -> Option<AdapterEventV1> {
        let e = self.entries.get(message_id)?;
        if !e.preview_active {
            return None;
        }
        let mut ev = AdapterEventV1::new(
            EventKind::Resume,
            message_id,
            e.revision,
            e.content_sha256.clone(),
        );
        ev.artifacts = e.artifacts.clone();
        ev.semantic_preview = e.semantic_preview.clone();
        debug_assert_eq!(sha256_hex(e.exact_source.as_bytes()), e.content_sha256);
        Some(ev)
    }

    pub fn exact_source(&self, message_id: &str) -> Option<&str> {
        self.entries
            .get(message_id)
            .map(|e| e.exact_source.as_str())
    }

    pub fn on_cancel(&mut self, message_id: &str) -> AdapterEventV1 {
        if let Some(e) = self.entries.get_mut(message_id) {
            e.preview_active = false;
            e.artifacts.clear();
            e.semantic_preview = None;
        }
        if let Some(e) = self.entries.get(message_id) {
            self.persist_entry(message_id, e);
        }
        AdapterEventV1::new(EventKind::Cancel, message_id, 0, sha256_hex(b""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resume_preserves_source() {
        let mut store = LifecycleStore::new();
        let content = "Answer: $a+b$.";
        store.on_message_end("m1", 1, content);
        let src = store.exact_source("m1").unwrap();
        assert_eq!(src, content);
        let ev = store.on_resume("m1").unwrap();
        assert_eq!(ev.kind, EventKind::Resume);
        assert!(!ev.artifacts.is_empty());
    }

    #[test]
    fn replace_and_delete() {
        let mut store = LifecycleStore::new();
        store.on_message_end("m1", 1, "x $y$ z");
        store.on_replace("m1", 2, "only text");
        assert_eq!(store.exact_source("m1").unwrap(), "only text");
        store.on_delete("m1");
        assert!(store.exact_source("m1").is_none());
    }

    #[test]
    fn no_stale_after_delete() {
        let mut store = LifecycleStore::new();
        store.on_message_end("m1", 1, "$x$");
        store.on_delete("m1");
        assert!(store.on_resume("m1").is_none());
    }

    #[test]
    fn duplicate_message_end_idempotent() {
        let mut store = LifecycleStore::new();
        let c = "math $q$";
        let a = store.on_message_end("m", 1, c);
        let b = store.on_message_end("m", 1, c);
        assert_eq!(a.content_sha256, b.content_sha256);
        assert_eq!(a.semantic_preview, b.semantic_preview);
    }

    #[test]
    fn cancel_clears_preview() {
        let mut store = LifecycleStore::new();
        store.on_message_end("m", 1, "$x$");
        store.on_cancel("m");
        assert!(store.on_resume("m").is_none());
        assert_eq!(store.exact_source("m").unwrap().contains("$x$"), true);
    }

    #[test]
    fn file_backed_resume() {
        let dir = tempfile::tempdir().unwrap();
        {
            let mut store = LifecycleStore::with_persist_dir(dir.path()).unwrap();
            store.on_message_end("m9", 3, "persist $k$");
        }
        let store2 = LifecycleStore::with_persist_dir(dir.path()).unwrap();
        let ev = store2.on_resume("m9").unwrap();
        assert_eq!(ev.revision, 3);
        assert_eq!(store2.exact_source("m9").unwrap(), "persist $k$");
    }

    #[test]
    fn out_of_order_replace_wins() {
        let mut store = LifecycleStore::new();
        store.on_message_end("m", 1, "old $a$");
        store.on_replace("m", 5, "new $b$");
        assert_eq!(store.exact_source("m").unwrap(), "new $b$");
        let ev = store.on_resume("m").unwrap();
        assert_eq!(ev.revision, 5);
    }
}
