//! Binary-level tests for hook + filter (shipped entry points).

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn bin() -> PathBuf {
    // Prefer cargo-built binary path from CARGO_BIN_EXE
    option_env!("CARGO_BIN_EXE_beavermath")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/beavermath")
        })
}

#[test]
fn hook_message_end_dual_stable() {
    let b = bin();
    assert!(b.exists(), "missing binary at {}", b.display());
    let req = r#"{"kind":"message_end","message_id":"m1","revision":1,"content":"The answer is $x^2+y^2=z^2$."}"#;
    let run = |req: &str| {
        let mut child = Command::new(&b)
            .arg("hook")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn");
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(req.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(
            out.status.success(),
            "stderr={}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(!out.stdout.contains(&0x1b));
        String::from_utf8(out.stdout).unwrap()
    };
    let a = run(req);
    let b_out = run(req);
    let va: serde_json::Value = serde_json::from_str(&a).unwrap();
    let vb: serde_json::Value = serde_json::from_str(&b_out).unwrap();
    assert_eq!(va["content_sha256"], vb["content_sha256"]);
    assert_eq!(va["exact_source"], "The answer is $x^2+y^2=z^2$.");
    assert!(va["semantic_preview"].as_str().unwrap().contains("power"));
}

#[test]
fn filter_adversarial_and_math() {
    let b = bin();
    let input = b"evil\x1b]1337;File=x\x07 and $x^2$ ok";
    let mut child = Command::new(&b)
        .arg("filter")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(input).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert!(!out.stdout.contains(&0x1b), "raw ESC leaked");
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("power") || s.contains("x"));
    assert!(s.contains("001B") || s.contains("\\u{001B}"));
}

#[test]
fn filter_dual_stable() {
    let b = bin();
    let doc = "The answer is $x^2+y^2=z^2$.";
    let run = || {
        let mut child = Command::new(&b)
            .arg("filter")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(doc.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success());
        out.stdout
    };
    assert_eq!(run(), run());
}

#[test]
fn present_matches_filter_dual() {
    let b = bin();
    let source = "The answer is $x^2+y^2=z^2$. URL https://x.com/$user$/p. Code `$x$`. Real $y$.";
    let run_present = || {
        let mut child = Command::new(&b)
            .args(["present", "--mode", "semantic"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(source.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(!out.stdout.contains(&0x1b));
        String::from_utf8(out.stdout).unwrap()
    };
    let a = run_present();
    let b_out = run_present();
    let va: serde_json::Value = serde_json::from_str(&a).unwrap();
    let vb: serde_json::Value = serde_json::from_str(&b_out).unwrap();
    assert_eq!(va["content_sha256"], vb["content_sha256"]);
    assert_eq!(va["exact_source"], source);
    assert_eq!(va["display_text"], vb["display_text"]);

    let mut child = Command::new(&b)
        .arg("filter")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(source.as_bytes())
        .unwrap();
    let fout = child.wait_with_output().unwrap();
    let filtered = String::from_utf8(fout.stdout).unwrap();
    assert_eq!(va["display_text"].as_str().unwrap(), filtered.trim_end());
    assert!(va["display_text"]
        .as_str()
        .unwrap()
        .contains("https://x.com/$user$/p"));
    assert!(va["display_text"].as_str().unwrap().contains("`$x$`"));
}

#[test]
fn grok_fixture_matches_filter_source() {
    let b = bin();
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../adapters/grok-build/tests/fixtures/stop-message-end.json");
    let req = std::fs::read_to_string(&fixture).expect("fixture");
    let mut child = Command::new(&b)
        .arg("hook")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(req.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8(out.stdout).unwrap()).unwrap();
    let source = v["exact_source"].as_str().unwrap();
    // Same source through filter
    let mut child = Command::new(&b)
        .arg("filter")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(source.as_bytes())
        .unwrap();
    let fout = child.wait_with_output().unwrap();
    let filtered = String::from_utf8(fout.stdout).unwrap();
    let preview = v["semantic_preview"].as_str().unwrap();
    assert_eq!(preview, filtered.trim_end());
    // Protections
    assert!(source.contains("https://x.com/$user$/p"));
    assert!(source.contains("`$x$`"));
    assert!(preview.contains("https://x.com/$user$/p"));
    assert!(preview.contains("`$x$`"));
    let arts = v["artifacts"].as_array().unwrap();
    assert!(arts.iter().any(|a| a["expression_exact_utf8"] == "$y$"));
    assert!(arts.iter().any(|a| a["expression_exact_utf8"]
        .as_str()
        .unwrap()
        .contains("x^2+y^2=z^2")));
}
