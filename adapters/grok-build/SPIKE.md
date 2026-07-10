# Grok Build Stop-hook payload spike (H2)

**Date:** 2026-07-10  
**Decision:** **G2a (sidecar preview)** for v1 lab path.

## Findings

### Documented surface

From Grok Build user guide (`~/.grok/docs/user-guide/10-hooks.md`):

| Event | Blocking? | Stdout used? |
|-------|-----------|--------------|
| `Stop` | No | No (passive) |
| `SubagentStop` | No | No |
| `SessionStart` / `SessionEnd` | No | No |
| `PreToolUse` | Yes | Yes (allow/deny) |

Hook input is **JSON on stdin**. Documented example fields for tool hooks include `sessionId`, `cwd`, `toolName`, `toolInput`. Lifecycle events (`Stop`) reject matchers; payload field set is implementation-defined and must be logged in the field.

### Spike method (reproducible)

1. Install logger hook (see `hooks/spike-log-stop.json` + `scripts/log-stop.py`).
2. Run any Grok turn that ends normally.
3. Inspect `~/.beavermath/spikes/stop-*.json`.

### Synthetic / compatibility payload (used when live Grok payload lacks full text)

Hosts and Cursor-compat layers often provide one of:

- `content` / `text` / `assistantMessage` / `lastAssistantMessage`
- `messages[]` with `role: assistant`
- `sessionId` only (then G2a cannot recover text without a documented transcript file)

**Policy if only sessionId:** do not scrape TUI; write empty skip event and fail-open. Record in preview dir as `skipped-no-content.json`.

### Fixture captured for CI

`tests/fixtures/stop-message-end.json` — normalized HookRequestV1 after mapper:

```json
{
  "kind": "message_end",
  "message_id": "grok-turn-1",
  "revision": 1,
  "content": "From the identity we have $x^2+y^2=z^2$. URL https://x.com/$user$/p stays. Code `$x$` stays. Real $y$.",
  "session_id": "spike-session",
  "write_preview": true
}
```

### G2a/b/c choice

| Option | Feasible now? | Rationale |
|--------|---------------|-----------|
| **G2a sidecar** | **Yes** | Passive hooks cannot rewrite TUI scrollback; sidecar + exact source section matches product safety |
| G2b ACP client | Later | Needs separate ACP consumer process |
| G2c inline TUI | **Blocked** | No public presentation-only assistant renderer API in user guide |

**Chosen:** G2a. Label UX: “BeaverMath preview sidecar (not true inline)”.

## Install

See `README.md` and `hooks/beavermath-stop.json`.
