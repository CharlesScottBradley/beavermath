# Patch proposal: Grok Build assistant paint path (true in-chat)

**Binary on this machine:** `~/.grok/downloads/grok-macos-aarch64` (v0.2.93)  
**Internal crate paths from strings:** `crates/codegen/xai-grok-shell/src/...`  
**Status:** closed source; cannot apply patch without xAI source or official API.

## Required behavior

```rust
// Conceptual — assistant message body paint (after stream complete)
fn paint_assistant_message(msg: &Message) {
    let source = msg.content_exact_utf8(); // immutable store
    match beavermath_present(source) {
        Ok(p) => paint_markdown_or_text(&p.display_text),
        Err(_) => paint_markdown_or_text(source), // fail-open
    }
    // NEVER: msg.content = p.display_text;
}
```

## Integration options for xAI

1. **Env-gated:** `GROK_BEAVERMATH_PRESENT=1` shells out to `beavermath present` on complete messages only.  
2. **Library:** link `beavermath_adapter::present_message_v1` if licensing allows.  
3. **Hook:** active presentation hook that returns display text (stdout JSON) for the TUI to apply (unlike passive Stop).

## Lab interim (shipped)

| Command | Role |
|---------|------|
| `beavermath present` | Shared IP0 API |
| `beavermath grok-session` | Read `chat_history.jsonl`, write `session/beavermath_display/VIEWPORT.md` |
| Stop hook | Invokes projection after turns (fail-open) |

Open `VIEWPORT.md` beside Grok (split pane) until paint lands. **Does not rewrite stock TUI.**

## Verification when unblocked

1. Session `chat_history.jsonl` still contains `$x^2+y^2=z^2$`.  
2. Viewport shows semantic power phrasing.  
3. URL/`$user$` and `` `$x$` `` unchanged in display.  
4. Resume recomputes from source.
