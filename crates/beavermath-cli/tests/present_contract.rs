//! Display contract fixture for in-chat hosts (IP1).

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn bin() -> PathBuf {
    option_env!("CARGO_BIN_EXE_beavermath")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/beavermath")
        })
}

#[test]
fn inchat_display_contract_fixture() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../adapters/grok-build/tests/fixtures/inchat-display-contract.json");
    let raw = std::fs::read_to_string(&fixture).expect("contract fixture");
    let contract: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let source = contract["source"].as_str().unwrap();

    let mut child = Command::new(bin())
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
    assert!(out.status.success());
    let p: serde_json::Value =
        serde_json::from_str(&String::from_utf8(out.stdout).unwrap()).unwrap();

    assert_eq!(p["exact_source"], source);
    for needle in contract["expected_source_contains"].as_array().unwrap() {
        assert!(
            source.contains(needle.as_str().unwrap()),
            "source missing {}",
            needle
        );
    }
    let display = p["display_text"].as_str().unwrap();
    if contract["expected_display_rules"]["must_not_equal_source_when_math"]
        .as_bool()
        .unwrap_or(false)
    {
        assert_ne!(display, source);
    }
    let url = contract["expected_display_rules"]["must_contain_url_literal"]
        .as_str()
        .unwrap();
    assert!(display.contains(url));
    let code = contract["expected_display_rules"]["must_contain_code_literal"]
        .as_str()
        .unwrap();
    assert!(display.contains(code));

    let mut child = Command::new(bin())
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
    assert_eq!(display, filtered.trim_end());
}
