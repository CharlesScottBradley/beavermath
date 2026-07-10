# Grok Build in-chat presentation (IP1)

**Product bar:** [docs/INCHAT_PRESENTATION.md](../../docs/INCHAT_PRESENTATION.md)  
**Status:** stock TUI paint still **blocked** (closed binary `xai-grok-shell`); **session display projection shipped** as interim viewport  
**Scaffold:** G2a sidecar ([SPIKE.md](SPIKE.md))  
**Shared surface:** `beavermath present` / `present_message_v1`  
**Session path:** `beavermath grok-session` → `beavermath_display/VIEWPORT.md` (source in `chat_history.jsonl` untouched)  
**Patch when source available:** [PATCH_PROPOSAL.md](PATCH_PROPOSAL.md)

## 1. Display contract (fixture-driven)

Host must maintain two fields per assistant message:

| Field | Role | Mutated by BeaverMath? |
|-------|------|------------------------|
| `content` / source | Exact UTF-8 for model + resume | **Never** |
| `display` / presentation | Viewport paint | Yes, derived only |

```text
source  = "The answer is $x^2+y^2=z^2$."
present = beavermath present(source)   # machine JSON
viewport paints present.display_text
session stores source only (or source + content_sha256)
on resume: display = present(source).display_text again
```

### Fixture: `tests/fixtures/inchat-display-contract.json`

```json
{
  "message_id": "grok-inchat-1",
  "revision": 1,
  "source": "The answer is $x^2+y^2=z^2$. URL https://x.com/$user$/p. Code `$x$`. Real $y$.",
  "expected_source_contains": ["$x^2+y^2=z^2$", "https://x.com/$user$/p", "`$x$`", "$y$"],
  "expected_display_rules": {
    "must_not_equal_source_when_math": true,
    "must_contain_url_literal": "https://x.com/$user$/p",
    "must_contain_code_literal": "`$x$`",
    "must_not_select_code_as_only_math": true
  }
}
```

Verified by `crates/beavermath-cli/tests/hook_and_filter.rs::present_matches_filter_dual` and adapter `present` unit tests: `display_text == filter(source)`, digests stable, zero ESC.

## 2. Paint-path design (P1)

### Target insertion point (when Grok Build source is available)

1. Find the widget/paint function that renders **assistant message body** in the TUI scrollback (markdown/text view for `agent_message` / assistant role).
2. Before paint, if message is **complete** (not streaming):

```text
let source = message.content; // immutable
let p = present_message_v1(PresentRequestV1 { source, mode: Semantic, .. })?;
// paint p.display_text (or span-aware paint later)
// never write p.display_text back into message.content
```

3. While streaming: paint `source` only (product rule: source while streaming).
4. Cache: `PresentCache` keyed by `content_sha256` + config digest.
5. On session load / compact restore: recompute display from source; do not persist display as authority.

### Integration options

| Option | Feasibility without xAI source tree |
|--------|-------------------------------------|
| A. Upstream PR: `registerAssistantPresentation` | Needs Grok Build repo access |
| B. Lab private patch of paint path | Needs local checkout of TUI crate |
| C. ACP client that paints its own panel | Not the stock Grok Build window |
| D. Sidecar G2a | Shipped; **fails product bar** |

**This environment:** Grok Build is installed as a binary/TUI under `~/.grok` without a modifiable agent-message paint source in-repo. Therefore **IP1 interactive viewport is blocked**; IP0 + this design + fixture contract are the maximum shippable slice.

## 3. Blocker record

See `BLOCKER.txt` (and goal scratch `grok-inchat-blocker.txt`):

- No open, patchable Grok Build assistant-message render source in the beavermath workspace or a discovered lab checkout.
- Documented hooks are passive on `Stop` (cannot rewrite pager).
- Claiming “math converts in this Grok chat window” would be false until a paint-path patch or official presentation API lands.

## 4. Host integration checklist (when unblocked)

- [ ] Clone/patch Grok message paint  
- [ ] Call `beavermath present` (subprocess) or link `beavermath_adapter::present_message_v1`  
- [ ] Unit test: stored session JSON still contains `$x^2$`  
- [ ] Manual: one derivation turn shows semantic in viewport  
- [ ] Fail-open: if present fails, paint source  

## 5. Proof of present readiness (shipped)

```bash
printf '%s' 'The answer is $x^2+y^2=z^2$.' | beavermath present --mode semantic | python3 -c \
  'import json,sys; p=json.load(sys.stdin); assert p["exact_source"]!=p["display_text"]; assert "power" in p["display_text"]'
```

Hosts only need to bind this JSON to their display field.
