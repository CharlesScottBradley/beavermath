use serde::{Deserialize, Serialize};

/// Hard limits from FR-046 (adapters may lower, never raise without explicit CLI opt-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitsV1 {
    pub max_input_bytes: usize,
    pub max_expression_bytes: usize,
    pub max_expressions_per_message: usize,
    pub max_tokens: usize,
    pub max_nesting_depth: usize,
    pub max_axis_px: u32,
    pub max_pixels: u64,
    pub max_decoded_raster_bytes: usize,
    pub max_encoded_artifact_bytes: usize,
    pub max_formula_ms: u64,
    pub max_message_ms: u64,
}

impl Default for LimitsV1 {
    fn default() -> Self {
        Self {
            max_input_bytes: 1024 * 1024,
            max_expression_bytes: 64 * 1024,
            max_expressions_per_message: 128,
            max_tokens: 100_000,
            max_nesting_depth: 64,
            max_axis_px: 4096,
            max_pixels: 4_194_304,
            max_decoded_raster_bytes: 16 * 1024 * 1024,
            max_encoded_artifact_bytes: 8 * 1024 * 1024,
            max_formula_ms: 500,
            max_message_ms: 2000,
        }
    }
}
