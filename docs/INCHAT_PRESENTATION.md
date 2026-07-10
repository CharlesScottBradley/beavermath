# Product bar: in-chat presentation (all harnesses)

**Status:** required product outcome  
**Date:** 2026-07-10 (IP0 shipped; IP1–IP5 host paint/API pending)  
**Parent plan:** [HARNESS_COMPAT_PLAN.md](HARNESS_COMPAT_PLAN.md)  
**Shared API:** `beavermath_adapter::present_message_v1` / `beavermath present`

## 1. What “done” actually means

When you are **inside** a harness chat window (Grok Build, OMP, Claude Code, Codex TUI/app, OpenCode UI), and the model finishes a turn containing explicit math:

```text
Stored / model context:  "From Maxwell, $E=mc^2$."
What you see in the chat: "From Maxwell, E equals (mc) to the power of (2)."
                           (or Unicode/pixels if capability allows)
```

| Requirement | Rule |
|-------------|------|
| **In the same window** | Presentation appears in the transcript/viewport the user is already reading |
| **Automatic** | No `beavermath` CLI, no tool call, no prompt instruction |
| **Source preserved** | Canonical message bytes stay exact UTF-8 LaTeX for resume, copy-as-source, model context |
| **All planned harnesses** | Grok Build, OMP, Codex, Claude Code, OpenCode (plus oneshot for any pipe) |
| **Not sufficient** | Sidecar files, supplemental “preview” messages below the turn, oneshot pipes only |

**Sidecar G2a and oneshot remain engineering scaffolds.** They do not satisfy the product bar.

## 2. Why G2a is not enough

| Path | User sees math where? | Product bar? |
|------|------------------------|--------------|
| G2a sidecar `~/.beavermath/previews/` | Another file | **No** |
| Supplemental custom message | Extra bubble under the turn | **Partial** (acceptable only as interim) |
| Oneshot `\| beavermath filter` | Outside interactive chat | **No** for interactive use |
| **Presentation overlay / rewrite of display layer** | **Same message region** | **Yes** |

Grok Build’s current hooks cannot paint the pager. Meeting the bar for Grok **requires a host presentation API or a fork** of the TUI render path. The same class of requirement applies to every harness: a place where **display** is derived from **immutable source**.

## 3. Universal mechanism (all hosts)

```text
┌──────────────────────────────────────────┐
│  Host message store (immutable source)   │
│  exact UTF-8 + message_id + revision     │
└──────────────────┬───────────────────────┘
                   │ on complete (and on resume)
                   ▼
┌──────────────────────────────────────────┐
│  beavermath hook / library               │
│  → MathArtifactV1[] + semantic_preview   │
│  → optional PNG digest (not protocol)    │
└──────────────────┬───────────────────────┘
                   │ presentation only
                   ▼
┌──────────────────────────────────────────┐
│  Host display layer                      │
│  renders semantic / unicode / image      │
│  for that message_id in the chat viewport│
└──────────────────────────────────────────┘
```

**Hard split (unchanged):**

- Model, history export for replay, and “copy source” → exact LaTeX  
- Viewport paint → presentation  
- Machine channels (RPC, hooks that are not the TUI) → JSON only, no ESC  

## 4. Per-harness path to the product bar

Each host needs **one** of these integration classes. Prefer the highest available.

| Class | Name | In-chat? | Notes |
|-------|------|----------|-------|
| **P1** | **Native content renderer** | Yes | Host calls adapter; paints overlay for assistant spans |
| **P2** | **Display-buffer transform** | Yes | After message complete, host replaces *display* string only |
| **P3** | **TTY-owning wrapper** | Yes* | Wrapper owns terminal, re-paints turn (fragile; last resort) |
| **S** | Sidecar / oneshot | No | Scaffold only |

### 4.1 Grok Build (lab first-class)

| Class | Mechanism | Work |
|-------|-----------|------|
| **P1 target** | Add presentation API, e.g. `registerAssistantPresentation(fn)` or post-render pass in the agent-message paint path: input immutable text → output display spans | **Grok Build code change** (or private patch) |
| P2 interim | If session store is readable and TUI re-reads a `display` field distinct from `content`, write display via documented API only | Spike required |
| S today | G2a Stop → sidecar | Shipped; **does not meet bar** |

**Spike for Grok P1 (concrete):**

1. Locate agent message render in Grok Build source (scrollback / markdown widget).  
2. Insert a pure function call: `present = beavermath_present(source)` with cache by `content_sha256`.  
3. Paint `present` in the widget; keep `source` in session JSON.  
4. On resume/compact, recompute from source only.  
5. Tests: source round-trip; no ESC in stored session; viewport contains semantic.

Until that lands, **Grok in-chat conversion will not happen.**

### 4.2 OMP

| Class | Mechanism |
|-------|-----------|
| **P1 target** | Upstream or patch `registerAssistantContentRenderer` (ROADMAP Phase 9 / BM-022): presentation-only, async, composable, failure falls back to source |
| P2 interim | Extension post-turn that injects a **replacement widget bound to the same message id** if OMP allows display override without mutating provider content |
| S | Supplemental custom message (current recipe) |

### 4.3 Claude Code

| Class | Mechanism |
|-------|-----------|
| **P1/P2** | Documented hooks that can attach a rich rendering surface, or Claude’s text transform on assistant output **if** it is display-only |
| **P3** | Optional rich wrapper that owns the TTY and reprints completed turns with filtered text (must not break headless; source on disk unchanged) |
| S | `hook-stop.sh` → sidecar |

### 4.4 Codex

| Class | Mechanism |
|-------|-----------|
| **P1/P2** | App-server / TUI structured events → presentation panel for completed items |
| Oneshot | Print mode stays filter (meets bar for **print**, not interactive TUI) |
| Interactive TUI | Requires Codex surface that separates display vs transcript |

### 4.5 OpenCode

| Class | Mechanism |
|-------|-----------|
| **P1/P2** | Plugin/server message complete → UI re-render from artifacts |
| Oneshot | Filter wrapper for CLI print |

## 5. Shared library surface (implement once)

All hosts should call the **same** presentation function:

```text
present_message_v1(source: &str, ctx: OutputContextV1) -> PresentationV1
  PresentationV1 {
    exact_source,           // echo
    content_sha256,
    display_text,           // semantic filter of whole message (default chat UX)
    spans: [{ range, semantic, alt, png_digest? }],
    mode: semantic | unicode | pixel_ref
  }
```

- CLI: `beavermath present` (JSON in/out) for hosts that shell out.  
- Rust crate: `beavermath_adapter::present_message` for native embeds.  
- Cache key: `content_sha256` + config digest.  
- Streaming: paint source until complete; then swap display (no partial math flash if host can double-buffer).

## 6. Delivery plan (reframed)

| Phase | Goal | Product bar? |
|-------|------|--------------|
| H0–H1 | Contract + oneshot | Scaffold |
| H2 G2a | Grok sidecar | **No** (done as scaffold) |
| **IP0** | `present_message` API + tests + `beavermath present` | Shared |
| **IP1 Grok P1** | Patch/PR Grok Build message paint path | **Yes for Grok** |
| **IP2 OMP P1** | Live content renderer or true display override | **Yes for OMP** |
| **IP3 Claude P1/P2/P3** | Best available in-chat path | **Yes for Claude** |
| **IP4 Codex** | Interactive path if API exists; else honest oneshot-only | Conditional |
| **IP5 OpenCode** | Plugin UI re-render | **Yes for OpenCode** |

**Track success criterion (new):**  
At least **Grok Build** and **one other** interactive harness show semantic math **in the chat viewport** for a completed turn containing `$x^2+y^2=z^2$`, with exact source still in the session store. Remaining hosts either meet the bar or are labeled **oneshot-only / blocked on host API** (never “live” if only sidecar).

## 7. Explicit non-goals (still)

- Mutating stored transcript to semantic permanently  
- Screen scraping another process’s TUI  
- Requiring the model to call a render tool  
- Claiming in-chat success from sidecar or `beavermath filter` alone  

## 8. Immediate next work

1. Implement **IP0** (`present_message` + `beavermath present`) so every host has one call.  
2. **Grok IP1:** find paint path; design minimal presentation hook; lab patch first, upstream second.  
3. **OMP IP2:** content renderer proposal + extension that uses it when available.  
4. Re-label PHASE_GATES: G2a = scaffold; product bar = in-chat only.

---

*This document overrides any earlier “H7 optional / G2a sufficient for H2” language for product completion.*
