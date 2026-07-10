//! BeaverMath core: versioned contracts, strict discovery, partition, and semantic text.
//!
//! Source authority is exact UTF-8 bytes and SHA-256 digests. Parsing and display
//! create derivatives only; they never rewrite canonical input.

pub mod diagnostics;
pub mod hash;
pub mod limits;
pub mod partition;
pub mod pipeline;
pub mod protect;
pub mod repair;
pub mod scanner;
pub mod semantic;
pub mod types;

pub use diagnostics::{DiagnosticCode, DiagnosticSeverity, DiagnosticV1, ExitStatus};
pub use hash::{sha256_hex, HashError};
pub use limits::LimitsV1;
pub use partition::{DocumentPartitionV1, SpanKind, TextSpanV1};
pub use pipeline::{process_document, process_expression, ProcessOptions};
pub use protect::{default_protected_ranges, merge_protected};
pub use repair::{RepairProposalV1, RepairProviderV1, RepairRequestV1};
pub use scanner::{discover_strict, ProtectedRange, ScanError};
pub use semantic::{semantic_for_tex, SemanticResult};
pub use types::*;

/// Product version embedded in provenance.
pub const BEAVERMATH_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SCHEMA_VERSION: &str = "1";
