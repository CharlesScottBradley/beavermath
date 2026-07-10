# OMP in-chat presentation

**Status:** **O0-O2 live (installable)** · **O3 upstream-blocked**  
**Product bar:** partial in-session (custom preview message), not true assistant-body inline

## Ladder

| Step | Status |
|------|--------|
| O0 SPIKE | [SPIKE.md](SPIKE.md) - `message_end` + ExtensionAPI |
| O1 present on complete | [extension/index.js](extension/index.js) |
| O2 in-session UX | Custom type `beavermath.math-preview` with `display: true` + `content: display_text` + Component renderer |
| O3 true content renderer | **Blocked** - no `registerAssistantContentRenderer` |

## Install

```bash
# PATH must include beavermath (cargo install --path crates/beavermath-cli)
export PATH="$HOME/.cargo/bin:$PATH"
REPO_ROOT="$(git rev-parse --show-toplevel)"

# One-shot session with extension
omp --extension="$REPO_ROOT/adapters/omp/extension" -p 'Explain $x^2+y^2=z^2$ briefly'

# Or add adapters/omp/extension to settings.extensions permanently (OMP settings UI / config)
```

## Behavior

1. On `message_end` for assistant text with math delimiters → `beavermath present`.  
2. Original assistant message **unchanged** (exact LaTeX for model).  
3. Extension sends a **custom** follow message: `display: true` (boolean show), `content: display_text` (readable math).  
4. `registerMessageRenderer` returns a pi-tui `Component` with `render(width) → string[]`.  
5. Immediate UX: `setWidget` on `message_end` (string[] below editor).  
6. Fail-open on present errors.

## Label (honest)

This is **automatic in-session** presentation bound to the turn via a BeaverMath preview message / widget.  
It is **not** true rewrite of the assistant content bubble (O3).

## Tests

```bash
node adapters/omp/tests/present-integration.mjs
```
