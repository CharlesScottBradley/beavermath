//! Corpus-driven strict discovery gates (positive / negative / hostile selection).

use beavermath_core::limits::LimitsV1;
use beavermath_core::scanner::discover_strict;
use beavermath_core::{process_document, ProcessOptions};
use std::path::PathBuf;

fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../corpus")
}

#[test]
fn positive_inline_pythagoras() {
    let input = "The answer is $x^2+y^2=z^2$.";
    let (spans, _) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
    assert_eq!(spans.len(), 1);
    let res = process_document(input.as_bytes(), &[], &ProcessOptions::default()).unwrap();
    assert_eq!(res.expressions.len(), 1);
    assert!(!res.expressions[0].semantic.text.is_empty());
    assert!(!res.expressions[0].alt.text.is_empty());
    assert_eq!(res.input_exact_utf8, input);
}

#[test]
fn negative_currency_file() {
    let path = corpus_root().join("negative/currency.txt");
    let input = std::fs::read_to_string(&path).expect("currency corpus");
    let (spans, _) = discover_strict(&input, &[], &LimitsV1::default()).unwrap();
    assert!(
        spans.is_empty(),
        "currency must not select math spans: {spans:?}"
    );
}

#[test]
fn negative_undelimited_file() {
    let path = corpus_root().join("negative/undelimited.txt");
    let input = std::fs::read_to_string(&path).expect("undelimited corpus");
    let (spans, _) = discover_strict(&input, &[], &LimitsV1::default()).unwrap();
    assert!(spans.is_empty(), "undelimited prose must not select: {spans:?}");
}

#[test]
fn positive_fence_file() {
    let path = corpus_root().join("positive/fence_math.txt");
    let input = std::fs::read_to_string(&path).expect("fence corpus");
    let (spans, _) = discover_strict(&input, &[], &LimitsV1::default()).unwrap();
    assert_eq!(spans.len(), 1);
    let inner = &input[spans[0].inner_range.start..spans[0].inner_range.end];
    assert!(inner.contains("x+1"));
}

#[test]
fn lossless_partition_on_mixed() {
    let input = "A $a$ B $$b$$ C \\(c\\) D \\[d\\] E";
    assert!(beavermath_core::pipeline::assert_partition_roundtrip(input));
}

#[test]
fn negative_url_dollar_pairs_auto_protect() {
    let path = corpus_root().join("negative/urls.txt");
    let input = std::fs::read_to_string(&path).expect("urls corpus");
    // Shipped path: process_document with empty host ranges auto-protects URLs.
    let res = process_document(input.as_bytes(), &[], &ProcessOptions::default()).unwrap();
    assert!(
        res.expressions.is_empty(),
        "URL $pairs$ must not become math: {:?}",
        res.expressions
            .iter()
            .map(|e| e.expression_exact_utf8.clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(res.input_exact_utf8, input);
}

#[test]
fn negative_inline_code_auto_protect() {
    let path = corpus_root().join("negative/inline_code.txt");
    let input = std::fs::read_to_string(&path).expect("inline code corpus");
    let res = process_document(input.as_bytes(), &[], &ProcessOptions::default()).unwrap();
    // Only real $y$ and $w$ outside code
    assert_eq!(res.expressions.len(), 2);
    let inners: Vec<_> = res
        .expressions
        .iter()
        .map(|e| e.expression_exact_utf8.as_str())
        .collect();
    assert!(inners.contains(&"$y$"));
    assert!(inners.contains(&"$w$"));
    assert!(!inners.iter().any(|s| *s == "$x$" || *s == "$z$"));
}

#[test]
fn url_and_real_math_together() {
    let input = "see https://x.com/$user$/profile and $a$";
    let res = process_document(input.as_bytes(), &[], &ProcessOptions::default()).unwrap();
    assert_eq!(res.expressions.len(), 1);
    assert_eq!(res.expressions[0].expression_exact_utf8, "$a$");
    // Filter must preserve URL dollars
    let filtered = res.filtered_text.unwrap();
    assert!(filtered.contains("https://x.com/$user$/profile"));
    assert!(!filtered.contains("$a$"));
}
