//! Neutral lifecycle/event contract for harness adapters.

pub mod events;
pub mod grok_session;
pub mod harness;
pub mod hook;
pub mod lifecycle;
pub mod present;

pub use events::{AdapterEventV1, EventKind};
pub use harness::{ClaudeAdapter, CodexAdapter, OmpAdapter, OpenCodeAdapter};
pub use hook::{process_hook_json, process_hook_request, HookRequestV1, HookResponseV1};
pub use lifecycle::LifecycleStore;
pub use present::{
    parse_present_request, present_message_v1, present_to_machine_json, PresentCache,
    PresentRequestV1, PresentationMode, PresentationSpanV1, PresentationV1,
};
