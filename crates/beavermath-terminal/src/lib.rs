//! Terminal-safe text and direct Kitty / iTerm2 typed frame encoders.
//!
//! Only privately constructed frames may emit protocol bytes. V1 performs zero queries.

pub mod decision;
pub mod escape;
pub mod iterm2;
pub mod kitty;
pub mod present;

pub use decision::{decide_presentation, graphics_preference_allows, PresentationDecision};
pub use escape::{escape_for_terminal_text, scan_machine_channel, MachineScan};
pub use iterm2::Iterm2Frame;
pub use kitty::KittyFrame;
pub use present::{encode_graphics_for_png, GraphicsEmit};
