//! Grok Build session I/O: read exact assistant source; never mutate chat_history.
//! Presentation is derived only (IP1 unblock path without binary paint patch).

use crate::present::{present_message_v1, PresentRequestV1, PresentationV1};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Resolve Grok home (`GROK_HOME` or `~/.grok`).
pub fn grok_home() -> PathBuf {
    if let Ok(h) = std::env::var("GROK_HOME") {
        return PathBuf::from(h);
    }
    dirs_home()
        .map(|h| h.join(".grok"))
        .unwrap_or_else(|| PathBuf::from(".grok"))
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// URL-encode cwd the way Grok groups sessions (simple percent-encoding for non-unreserved).
pub fn encode_cwd(cwd: &str) -> String {
    let mut out = String::new();
    for b in cwd.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b'/' => out.push_str("%2F"),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

pub fn sessions_root() -> PathBuf {
    grok_home().join("sessions")
}

/// List session dirs under a cwd group, newest first by summary updated_at if present.
pub fn list_sessions_for_cwd(cwd: &str) -> Vec<PathBuf> {
    let group = sessions_root().join(encode_cwd(cwd));
    if !group.is_dir() {
        return vec![];
    }
    let mut dirs: Vec<(String, PathBuf)> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&group) {
        for e in rd.flatten() {
            let p = e.path();
            if !p.is_dir() {
                continue;
            }
            let updated = std::fs::read_to_string(p.join("summary.json"))
                .ok()
                .and_then(|s| serde_json::from_str::<Value>(&s).ok())
                .and_then(|v| {
                    v.get("updated_at")
                        .or_else(|| v.get("last_active_at"))
                        .and_then(|x| x.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_default();
            dirs.push((updated, p));
        }
    }
    dirs.sort_by(|a, b| b.0.cmp(&a.0));
    dirs.into_iter().map(|(_, p)| p).collect()
}

pub fn latest_session_for_cwd(cwd: &str) -> Option<PathBuf> {
    list_sessions_for_cwd(cwd).into_iter().next()
}

/// Extract assistant message texts from chat_history.jsonl (exact source only).
pub fn extract_assistant_sources(session_dir: &Path) -> std::io::Result<Vec<String>> {
    let path = session_dir.join("chat_history.jsonl");
    let f = File::open(path)?;
    let mut out = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let Ok(v) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let ty = v.get("type").and_then(|t| t.as_str()).unwrap_or("");
        if ty != "assistant" && ty != "model" {
            continue;
        }
        if let Some(text) = content_to_text(v.get("content")) {
            if !text.trim().is_empty() {
                out.push(text);
            }
        }
    }
    Ok(out)
}

/// Last completed assistant source in the session.
pub fn last_assistant_source(session_dir: &Path) -> std::io::Result<Option<String>> {
    Ok(extract_assistant_sources(session_dir)?.into_iter().last())
}

fn content_to_text(content: Option<&Value>) -> Option<String> {
    let c = content?;
    if let Some(s) = c.as_str() {
        return Some(s.to_string());
    }
    if let Some(arr) = c.as_array() {
        let mut parts = Vec::new();
        for block in arr {
            if let Some(s) = block.as_str() {
                parts.push(s.to_string());
                continue;
            }
            if let Some(t) = block.get("text").and_then(|t| t.as_str()) {
                parts.push(t.to_string());
            } else if let Some(t) = block.get("content").and_then(|t| t.as_str()) {
                parts.push(t.to_string());
            }
        }
        if parts.is_empty() {
            return None;
        }
        return Some(parts.join(""));
    }
    None
}

/// Present every assistant message; source list unchanged.
pub fn present_session_assistants(session_dir: &Path) -> Result<Vec<PresentationV1>, String> {
    let sources =
        extract_assistant_sources(session_dir).map_err(|e| format!("read session: {e}"))?;
    let mut out = Vec::with_capacity(sources.len());
    for source in sources {
        let p = present_message_v1(&PresentRequestV1 {
            source,
            ..Default::default()
        })?;
        out.push(p);
    }
    Ok(out)
}

/// Present last assistant only.
pub fn present_last_assistant(session_dir: &Path) -> Result<Option<PresentationV1>, String> {
    let Some(source) = last_assistant_source(session_dir).map_err(|e| e.to_string())? else {
        return Ok(None);
    };
    Ok(Some(present_message_v1(&PresentRequestV1 {
        source,
        ..Default::default()
    })?))
}

/// Write display-only projections next to the session (never touch chat_history).
pub fn write_session_projections(session_dir: &Path) -> Result<PathBuf, String> {
    let presented = present_session_assistants(session_dir)?;
    let dir = session_dir.join("beavermath_display");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
    }
    let index_path = dir.join("index.jsonl");
    let mut lines = Vec::new();
    for (i, p) in presented.iter().enumerate() {
        let msg_path = dir.join(format!("assistant-{i:04}.md"));
        let body = format!(
            "<!-- display only; exact source is authoritative in chat_history.jsonl -->\n\n\
             ## Display\n\n{}\n\n## Exact source\n\n```\n{}\n```\n",
            p.display_text, p.exact_source
        );
        std::fs::write(&msg_path, body).map_err(|e| e.to_string())?;
        lines.push(
            serde_json::json!({
                "index": i,
                "content_sha256": p.content_sha256,
                "display_path": msg_path.file_name().and_then(|s| s.to_str()),
                "exact_source_len": p.exact_source.len(),
                "display_text_len": p.display_text.len(),
            })
            .to_string(),
        );
    }
    std::fs::write(&index_path, lines.join("\n") + "\n").map_err(|e| e.to_string())?;
    // Combined viewport file for quick reading
    let mut view = String::from("# BeaverMath session display (source preserved separately)\n\n");
    for (i, p) in presented.iter().enumerate() {
        view.push_str(&format!(
            "## Assistant message {i}\n\n{}\n\n---\n\n",
            p.display_text
        ));
    }
    let view_path = dir.join("VIEWPORT.md");
    std::fs::write(&view_path, view).map_err(|e| e.to_string())?;
    Ok(view_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn encode_cwd_slash() {
        assert_eq!(encode_cwd("/home/user/project"), "%2Fhome%2Fuser%2Fproject");
    }

    #[test]
    fn extract_and_present_from_fixture_session() {
        let dir = tempfile::tempdir().unwrap();
        let hist = dir.path().join("chat_history.jsonl");
        let mut f = File::create(&hist).unwrap();
        writeln!(
            f,
            r#"{{"type":"user","content":[{{"type":"text","text":"hi"}}]}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"type":"assistant","content":[{{"type":"text","text":"The answer is $x^2+y^2=z^2$."}}]}}"#
        )
        .unwrap();
        let sources = extract_assistant_sources(dir.path()).unwrap();
        assert_eq!(sources.len(), 1);
        assert!(sources[0].contains("$x^2"));
        let p = present_last_assistant(dir.path()).unwrap().unwrap();
        assert_eq!(p.exact_source, sources[0]);
        assert_ne!(p.display_text, p.exact_source);
        assert!(p.display_text.contains("power"));
        let view = write_session_projections(dir.path()).unwrap();
        let body = std::fs::read_to_string(view).unwrap();
        assert!(body.contains("power"));
        // history untouched
        let hist_body = std::fs::read_to_string(hist).unwrap();
        assert!(hist_body.contains("$x^2+y^2=z^2$"));
    }
}
