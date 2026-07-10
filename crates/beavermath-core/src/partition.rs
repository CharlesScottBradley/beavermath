use crate::scanner::MathSpan;
use crate::types::ByteRangeV1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpanKind {
    Text,
    Math,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSpanV1 {
    pub kind: SpanKind,
    pub range: ByteRangeV1,
    /// Index into expressions list when kind is Math.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentPartitionV1 {
    pub spans: Vec<TextSpanV1>,
}

/// Build ordered non-overlapping complete partition from math spans.
pub fn build_partition(input_len: usize, math: &[MathSpan]) -> DocumentPartitionV1 {
    let mut spans = Vec::new();
    let mut cursor = 0usize;
    for (idx, m) in math.iter().enumerate() {
        let s = m.full_range.start;
        let e = m.full_range.end;
        if s > cursor {
            spans.push(TextSpanV1 {
                kind: SpanKind::Text,
                range: ByteRangeV1 {
                    start: cursor,
                    end: s,
                },
                expression_index: None,
            });
        }
        spans.push(TextSpanV1 {
            kind: SpanKind::Math,
            range: ByteRangeV1 { start: s, end: e },
            expression_index: Some(idx),
        });
        cursor = e;
    }
    if cursor < input_len {
        spans.push(TextSpanV1 {
            kind: SpanKind::Text,
            range: ByteRangeV1 {
                start: cursor,
                end: input_len,
            },
            expression_index: None,
        });
    }
    if spans.is_empty() && input_len == 0 {
        // empty document: single empty text span optional; leave empty
    }
    DocumentPartitionV1 { spans }
}

/// Concatenate exact slices; must equal original bytes.
pub fn reconstruct<'a>(input: &'a str, partition: &DocumentPartitionV1) -> String {
    let mut out = String::with_capacity(input.len());
    for s in &partition.spans {
        out.push_str(&input[s.range.start..s.range.end]);
    }
    out
}

pub fn partition_is_lossless(input: &str, partition: &DocumentPartitionV1) -> bool {
    reconstruct(input, partition) == input
        && no_overlap_or_gap(input.len(), partition)
}

fn no_overlap_or_gap(len: usize, partition: &DocumentPartitionV1) -> bool {
    let mut cursor = 0usize;
    for s in &partition.spans {
        if s.range.start != cursor || s.range.end < s.range.start || s.range.end > len {
            return false;
        }
        cursor = s.range.end;
    }
    cursor == len || (len == 0 && partition.spans.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::limits::LimitsV1;
    use crate::scanner::discover_strict;

    #[test]
    fn lossless_round_trip() {
        let input = "Hello $x$ world $$y=1$$ end\n";
        let (math, _) = discover_strict(input, &[], &LimitsV1::default()).unwrap();
        let p = build_partition(input.len(), &math);
        assert!(partition_is_lossless(input, &p));
        assert_eq!(reconstruct(input, &p), input);
    }

    #[test]
    fn no_math_full_text_span() {
        let input = "no math here";
        let p = build_partition(input.len(), &[]);
        assert!(partition_is_lossless(input, &p));
        assert_eq!(p.spans.len(), 1);
        assert_eq!(p.spans[0].kind, SpanKind::Text);
    }
}
