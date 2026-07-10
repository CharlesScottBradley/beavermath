//! Machine-safe hook process protocol: JSON request in → AdapterEventV1 out.

use crate::events::{AdapterEventV1, EventKind};
use crate::lifecycle::LifecycleStore;
use beavermath_core::diagnostics::ExitStatus;
use beavermath_core::hash::sha256_hex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HookRequestV1 {
    #[serde(default = "default_schema")]
    pub schema_version: String,
    pub kind: String,
    pub message_id: String,
    #[serde(default)]
    pub revision: u64,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub write_preview: bool,
    #[serde(default)]
    pub preview_dir: Option<String>,
}

fn default_schema() -> String {
    beavermath_core::SCHEMA_VERSION.to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct HookResponseV1 {
    #[serde(flatten)]
    pub event: AdapterEventV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_path: Option<String>,
}

/// Process one hook request against a store. Pure machine path.
pub fn process_hook_request(
    store: &mut LifecycleStore,
    req: &HookRequestV1,
) -> Result<HookResponseV1, (ExitStatus, String)> {
    let kind = parse_kind(&req.kind).ok_or_else(|| {
        (
            ExitStatus::Usage,
            format!("unknown kind: {}", req.kind),
        )
    })?;

    let mut event = match kind {
        EventKind::MessageEnd | EventKind::TurnEnd => {
            let content = req.content.as_deref().ok_or_else(|| {
                (
                    ExitStatus::Usage,
                    "content required for message_end".into(),
                )
            })?;
            store.on_message_end(&req.message_id, req.revision, content)
        }
        EventKind::Replace => {
            let content = req.content.as_deref().ok_or_else(|| {
                (ExitStatus::Usage, "content required for replace".into())
            })?;
            store.on_replace(&req.message_id, req.revision, content)
        }
        EventKind::Delete => store.on_delete(&req.message_id),
        EventKind::Cancel => store.on_cancel(&req.message_id),
        EventKind::Resume => store.on_resume(&req.message_id).ok_or_else(|| {
            (
                ExitStatus::InvalidInput,
                format!("no entry for resume: {}", req.message_id),
            )
        })?,
        EventKind::MessageStart | EventKind::MessageUpdate => {
            // No render while streaming
            AdapterEventV1::new(
                kind,
                &req.message_id,
                req.revision,
                sha256_hex(req.content.as_deref().unwrap_or("").as_bytes()),
            )
        }
        other => AdapterEventV1::new(
            other,
            &req.message_id,
            req.revision,
            sha256_hex(b""),
        ),
    };

    // Align kind for turn_end alias
    if matches!(kind, EventKind::TurnEnd) {
        event.kind = EventKind::MessageEnd;
    }

    let exact_source = store.exact_source(&req.message_id).map(|s| s.to_string());

    let mut preview_path = None;
    if req.write_preview {
        if let Some(preview) = event.semantic_preview.as_ref() {
            let dir = resolve_preview_dir(req.preview_dir.as_deref(), req.session_id.as_deref());
            match write_sidecar_preview(&dir, &req.message_id, req.revision, preview, exact_source.as_deref())
            {
                Ok(p) => preview_path = Some(p.display().to_string()),
                Err(e) => {
                    // Fail-open for preview write: still return event
                    let _ = e;
                }
            }
        }
    }

    Ok(HookResponseV1 {
        event,
        exact_source,
        preview_path,
    })
}

pub fn process_hook_json(
    store: &mut LifecycleStore,
    json: &str,
) -> Result<String, (ExitStatus, String)> {
    let req: HookRequestV1 = serde_json::from_str(json).map_err(|e| {
        (ExitStatus::Usage, format!("invalid JSON: {e}"))
    })?;
    let resp = process_hook_request(store, &req)?;
    let out = serde_json::to_string(&resp).map_err(|e| {
        (ExitStatus::Internal, format!("serialize: {e}"))
    })?;
    // Machine purity
    if out.as_bytes().contains(&0x1b) {
        return Err((
            ExitStatus::Internal,
            "refused ESC in machine JSON".into(),
        ));
    }
    Ok(out)
}

fn parse_kind(s: &str) -> Option<EventKind> {
    let t = s.to_ascii_lowercase().replace('-', "_");
    Some(match t.as_str() {
        "message_start" => EventKind::MessageStart,
        "message_update" => EventKind::MessageUpdate,
        "message_end" | "stop" | "agent_stop" => EventKind::MessageEnd,
        "turn_end" => EventKind::TurnEnd,
        "branch" => EventKind::Branch,
        "resume" => EventKind::Resume,
        "session_switch" => EventKind::SessionSwitch,
        "shutdown" => EventKind::Shutdown,
        "cancel" => EventKind::Cancel,
        "replace" => EventKind::Replace,
        "delete" => EventKind::Delete,
        _ => return None,
    })
}

fn resolve_preview_dir(explicit: Option<&str>, session_id: Option<&str>) -> PathBuf {
    let base = if let Some(p) = explicit {
        PathBuf::from(p)
    } else if let Some(home) = std::env::var_os("HOME") {
        PathBuf::from(home).join(".beavermath/previews")
    } else {
        PathBuf::from(".beavermath/previews")
    };
    if let Some(sid) = session_id {
        base.join(sid)
    } else {
        base.join("_default")
    }
}

fn write_sidecar_preview(
    dir: &Path,
    message_id: &str,
    revision: u64,
    semantic: &str,
    exact: Option<&str>,
) -> std::io::Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700));
    }
    let safe_id: String = message_id
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    let path = dir.join(format!("{safe_id}-r{revision}.md"));
    let mut body = String::new();
    body.push_str("<!-- beavermath preview: presentation only; source is authoritative -->\n\n");
    body.push_str("## Semantic preview\n\n");
    body.push_str(semantic);
    body.push_str("\n\n");
    if let Some(src) = exact {
        body.push_str("## Exact source (canonical)\n\n```\n");
        body.push_str(src);
        body.push_str("\n```\n");
    }
    std::fs::write(&path, body)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use beavermath_terminal::escape::scan_machine_channel;

    #[test]
    fn message_end_json_round_trip_pure() {
        let mut store = LifecycleStore::new();
        let req = r#"{
            "kind": "message_end",
            "message_id": "m1",
            "revision": 1,
            "content": "The answer is $x^2+y^2=z^2$."
        }"#;
        let out = process_hook_json(&mut store, req).unwrap();
        assert!(scan_machine_channel(out.as_bytes()).clean);
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["message_id"], "m1");
        assert_eq!(v["content_sha256"].as_str().unwrap().len(), 64);
        assert!(v["semantic_preview"].as_str().unwrap().contains("power"));
        assert_eq!(
            v["exact_source"].as_str().unwrap(),
            "The answer is $x^2+y^2=z^2$."
        );
    }

    #[test]
    fn dual_run_stable_digest() {
        let mut s1 = LifecycleStore::new();
        let mut s2 = LifecycleStore::new();
        let req = r#"{"kind":"message_end","message_id":"m","revision":1,"content":"see $a$"}"#;
        let a = process_hook_json(&mut s1, req).unwrap();
        let b = process_hook_json(&mut s2, req).unwrap();
        let va: serde_json::Value = serde_json::from_str(&a).unwrap();
        let vb: serde_json::Value = serde_json::from_str(&b).unwrap();
        assert_eq!(va["content_sha256"], vb["content_sha256"]);
        assert_eq!(va["semantic_preview"], vb["semantic_preview"]);
    }

    #[test]
    fn url_and_code_not_selected() {
        let mut store = LifecycleStore::new();
        let content = "see https://x.com/$user$/p and code `$x$` and $y$";
        let req = serde_json::json!({
            "kind": "message_end",
            "message_id": "m",
            "revision": 1,
            "content": content
        });
        let out = process_hook_json(&mut store, &req.to_string()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let arts = v["artifacts"].as_array().unwrap();
        assert_eq!(arts.len(), 1);
        assert_eq!(arts[0]["expression_exact_utf8"], "$y$");
        let preview = v["semantic_preview"].as_str().unwrap();
        assert!(preview.contains("https://x.com/$user$/p"));
        assert!(preview.contains("`$x$`"));
    }

    #[test]
    fn write_preview_sidecar() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = LifecycleStore::new();
        let req = serde_json::json!({
            "kind": "message_end",
            "message_id": "m1",
            "revision": 1,
            "content": "hi $z$",
            "write_preview": true,
            "preview_dir": dir.path().join("p").to_string_lossy(),
            "session_id": "sess1"
        });
        let out = process_hook_json(&mut store, &req.to_string()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        let path = v["preview_path"].as_str().unwrap();
        let body = std::fs::read_to_string(path).unwrap();
        assert!(body.contains("Exact source"));
        assert!(body.contains("hi $z$"));
        assert!(body.contains("Semantic preview"));
    }
}
