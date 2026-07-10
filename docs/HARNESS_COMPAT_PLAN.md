# BeaverMath Harness Compatibility Plan

**Status:** active track (scaffolds shipped; **product bar elevated**)  
**Scope:** live harness integrations that make model LaTeX readable **in the chat window**  
**Product bar (authoritative):** [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md)  
**Sources of truth:** [PRD](../tasks/prd-beavermath.md), [ROADMAP](../ROADMAP.md), this document, INCHAT_PRESENTATION.md  
**Lab priority:** Grok Build is a **first-class** target alongside OMP, Codex, Claude Code, and OpenCode

---

## 1. Outcome

When a developer derives with an LLM **inside** a supported CLI harness chat, the model keeps writing ordinary explicit LaTeX (`$...$`, `$$...$$`, `\(...\)`, `\[...\]`, fenced math). The **host display layer** presents completed assistant content as readable math **in the same viewport**. Exact UTF-8 source remains canonical for model context, session history, copy-as-source, resume, branch, and audit.

No model tool call. No prompt instruction. No screen scraping. No mutation of canonical transcript bytes.

**Sidecar files, supplemental bubbles, and oneshot pipes are scaffolds only.** They do not meet the product bar.

### Success picture (one turn)

```text
Model streams:  "From Maxwell, $E=mc^2$ and $x^2+y^2=z^2$."
Stored/source:  identical LaTeX bytes + digests
User display:   "From Maxwell, E equals (mc) to the power of (2) and
                 (x) to the power of (2) plus (y) to the power of (2)
                 equals (z) to the power of (2)."
Optional TTY:   same text, then typed Kitty/iTerm2 PNG if authorized
```

---

## 2. What already exists vs what this track adds

| Layer | Status today | This track |
|-------|--------------|------------|
| `beavermath-core` strict discovery, digests, semantic, auto-protect | shipped | consume only; fix only if harness gaps force it |
| `beavermath-render` RaTeX PNG/SVG | shipped | consume for optional pixels / artifact refs |
| `beavermath-terminal` policy + Kitty/iTerm2 typed frames | shipped | consume when adapter owns TTY |
| `beavermath-cli` render/filter/doctor | shipped | oneshot path + hook helper binaries |
| `beavermath-adapter` LifecycleStore + fixture harness wrappers | **fixture-level** | harden contract; become shared spine |
| Live OMP / Codex / Claude / OpenCode / Grok | **not live** | implement for real |
| Grok Build in original PRD list | absent | **add as first-class lab target** |

**Honest gate language:** ROADMAP Phases 8 and 11 were marked fixture-pass. This plan replaces that with **live** exit gates. Fixture tests remain necessary but not sufficient.

---

## 3. Architecture

### 3.1 Layers (do not collapse)

```text
┌─────────────────────────────────────────────────────────────┐
│  Host harness (Grok / OMP / Codex / Claude / OpenCode)      │
│  owns session, streaming UI, model context, TTY or not      │
└───────────────────────────┬─────────────────────────────────┘
                            │ documented events / hooks / pipe
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  adapters/<host>/                                           │
│  map host events → AdapterEventV1 + exact content           │
│  NEVER emit terminal protocol bytes on machine channels     │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  beavermath-adapter (shared)                                │
│  LifecycleStore: message_id, revision, content_sha256,      │
│  exact_source, artifacts, preview_active                    │
└───────────────────────────┬─────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              ▼                           ▼
     beavermath-core                 beavermath-render
     (discovery/semantic)            (optional PNG/SVG)
              │                           │
              └─────────────┬─────────────┘
                            ▼
              beavermath-terminal (only if adapter owns TTY
              + trusted capability; after semantic exists)
```

### 3.2 Two output channels (hard split)

| Channel | Contents | ESC / graphics |
|---------|----------|----------------|
| **Machine** (hooks stdout policy, RPC, JSONL, adapter events, CI) | `AdapterEventV1`, digests, semantic strings, artifact refs | **Forbidden** |
| **TTY presentation** (only when this process owns a direct TTY) | semantic text already shown, then optional typed Kitty/iTerm2 frames | Allowed only via private frame types |

Adapters that run as **hooks** almost never own the harness TTY. They produce machine artifacts and/or write a sidecar preview file. Pixels require either:

- a **rich wrapper** that owns the TTY, or  
- a **host presentation hook** that asks BeaverMath for PNG and paints itself, or  
- user runs interactive `beavermath render --format auto --trusted-capability …` outside the hook.

### 3.3 Presentation modes (per message)

| Mode | When | User sees |
|------|------|-----------|
| `source_while_streaming` | chunks still arriving | exact LaTeX (host default) |
| `semantic_complete` | turn/message complete | filtered semantic text overlay or sidecar |
| `artifact_ref` | host supports attachments/widgets | link/ref to PNG/SVG + semantic |
| `pixel_inline` | direct TTY + trusted capability | semantic first, then protocol frames |
| `unchanged` | no accepted math | byte-identical input |

Default for v1 harness path: **semantic_complete**. Pixels are opt-in and host-capability gated.

---

## 4. Shared contract (freeze before host work)

### 4.1 Types (already largely present; harden)

| Type | Role |
|------|------|
| `RenderRequestV1` | input, protected ranges, output context, flags |
| `MathArtifactV1` | per-expression universal artifact |
| `DocumentResultV1` | whole-message partition result |
| `AdapterEventV1` | lifecycle envelope for hosts |
| `OutputContextV1` | TTY/pipe/capability/accessibility |
| `DiagnosticV1` / `ExitStatus` | stable codes |

### 4.2 Lifecycle events adapters must implement

| Event | Trigger | Store action |
|-------|---------|--------------|
| `MessageStart` | first chunk / message open | optional; no render |
| `MessageUpdate` | streaming chunk | track incomplete spans; **do not** replace display with partial parse |
| `MessageEnd` | completed assistant text | `process_document` → artifacts + semantic_preview |
| `Replace` | edit/regenerate | recompute; bump revision |
| `Delete` | message removed | drop entry; no stale preview |
| `Resume` | session reload | rebuild preview from exact_source only |
| `Cancel` | turn cancelled | clear in-flight; publish no partial artifact |
| `Branch` / `SessionSwitch` | host-specific | invalidate or re-key previews |

**Idempotency key:** `(message_id, revision, content_sha256)`.

### 4.3 Auto-protect (already in core)

`ProcessOptions.auto_protect = true` merges URL + inline code + non-math fence ranges. Hosts may pass **additional** protected ranges (e.g. tool JSON blobs). Hosts must not disable auto-protect for chat paths without a documented reason.

### 4.4 New shared surface to add (H0)

| Deliverable | Purpose |
|-------------|---------|
| `adapters/common/` docs + re-exports | one import path for hosts |
| `beavermath hook-process` or `beavermath-adapter` JSON-stdin binary | `AdapterEventV1` in ← `AdapterEventV1` / preview out |
| Golden lifecycle fixture suite | resume/replace/delete/cancel/duplicate/out-of-order |
| Schema fixtures for `AdapterEventV1` | FR-072 completeness |
| `docs/ADAPTER_CONTRACT.md` | field-level, non-negotiable rules |

**Hook-process I/O sketch (machine-safe):**

```text
stdin:  JSON { "kind": "message_end", "message_id", "revision", "content": "<exact utf-8>" }
stdout: JSON AdapterEventV1  (no ESC)
stderr: human diagnostics only
exit:   ExitStatus codes
```

Optional second mode:

```text
stdin:  raw document text
stdout: filtered semantic document (filter mode)
```

---

## 5. Dependency graph

```text
H0  Shared contract + hook-process + golden lifecycle fixtures
 │
 ├── H1  Universal oneshot / pipe path (filter + JSONL polish)
 │         no host SDK required
 │
 ├── H2  Grok Build live adapter (lab first-class)  ──┐
 │         depends on H0; benefits from H1           │
 │                                                   │
 ├── H3  OMP live extension preview                   │ parallel after H0
 │         depends on H0; pixels only if extension   │
 │         owns a surface                            │
 │                                                   │
 ├── H4  Codex structured + oneshot                   │
 ├── H5  Claude Code hooks + optional rich wrapper    │
 └── H6  OpenCode server/plugin + oneshot            │
                                                     │
H7  True inline presentation hooks (OMP core / Grok pager)
      only if host exposes presentation-only API
      does NOT block H2-H6 semantic success
```

**Ordering rules**

1. **H0 before any live host.** No host-specific schema forks.  
2. **H1 early** so any CLI that can pipe text works during adapter work.  
3. **H2 (Grok) and H3 (OMP) may parallelize** after H0; Grok is lab-priority, OMP is PRD-priority. If forced to serialize: **H2 then H3** for this lab.  
4. **H4-H6 after H0**; prefer after at least one of H2/H3 is green so lifecycle fixtures are battle-tested.  
5. **H7 never blocks** “readable math in chat” claims that only need semantic overlay/sidecar.

**Does not depend on:** Phase 12 public publish, Phase 13 probing/Sixel, MCP server (unless a host forces it).

**Core dependency:** current private tree at/after `bc9887f` (auto-protect, CLI Auto graphics path). Re-run `cargo test --locked --workspace` as H0 entry gate.

---

## 6. Mechanism designs (specific where it counts)

### 6.1 Grok Build (first-class lab target)

#### Problem

Grok Build renders markdown in the TUI. It does **not** call BeaverMath on assistant text. Hooks exist but are mostly **passive** (stdout ignored except `PreToolUse` deny). There is **no** documented “replace assistant message display” API in the public user guide.

#### Mechanism ladder (implement in order; stop at first viable product bar)

**G2a - Turn-end hook + sidecar preview (minimum viable product)**

| Item | Detail |
|------|--------|
| Trigger | `Stop` and/or `SubagentStop` hook (`~/.grok/hooks/` or project `.grok/hooks/`) |
| Input | Hook JSON on stdin; **probe actual payload** in H2 spike (document fields: `sessionId`, any transcript path, last message text if present) |
| Process | `beavermath-hook` binary: if full assistant text present → `LifecycleStore::on_message_end`; else fall back to session file read if path is documented/stable |
| Output | Write machine-safe JSON + human preview under `~/.beavermath/previews/<sessionId>/<messageKey>.md` (or project `.beavermath/`) |
| User UX | Preview file + optional notification; **not** yet inline in pager |
| Failure | Fail-open; never block Stop |

**Exit for G2a:** completing a math-heavy turn produces a preview file whose semantic text matches `beavermath filter` on the same source; exact source file retained.

**G2b - ACP / headless consumer (structured client)**

| Item | Detail |
|------|--------|
| Trigger | ACP `session/update` with `agent_message_chunk`; buffer until turn end |
| Process | Same LifecycleStore |
| Output | Client-side presentation panel or filtered scrollback in a **BeaverMath-aware ACP client** |
| Use | Editors / custom clients; not the default TUI unless Grok embeds it |

**G2c - Presentation-only TUI integration (true chat UX) - PRODUCT BAR**

| Item | Detail |
|------|--------|
| Requires | Grok Build change or official extension: e.g. `registerAssistantPresentation` / post-render transform that receives immutable message text and returns display-only markup or overlay regions |
| Constraint | Transform must not rewrite stored transcript or model replay buffer |
| Pixels | Host passes `OutputContextV1` with trusted capability if the host owns the terminal |
| **Status** | **Required for Grok product completion.** G2a does not satisfy “converts in this window.” See [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md). |

**H2 spike (first week of H2, before committing to G2a/G2b/G2c):**

1. Install a `Stop` hook that logs full stdin JSON to a file for 5 real turns.  
2. Catalog whether last assistant text is present, truncated, or only session-id.  
3. Decision record in `adapters/grok-build/SPIKE.md`: choose G2a, G2b, G2c, or hybrid.  
4. If text is missing from Stop payload, define the **only** allowed recovery: documented session transcript API or file format; never scrape the TUI.

#### Grok-specific invariants

- Do not put BeaverMath instructions in system prompt.  
- Do not require a render tool.  
- Hooks fail-open.  
- Compaction (`PreCompact`/`PostCompact`): previews re-keyed or dropped; source of truth remains session transcript.  
- Subagents: `SubagentStop` treated like a message end with distinct `message_id` namespace `subagent:<id>:` 

### 6.2 OMP (PRD-first public path)

| Path | Mechanism | UX |
|------|-----------|-----|
| **O8 live** | Extension `pi.on(message_end | turn events)` → LifecycleStore → `sendMessage` / custom message / widget as **supplemental** preview | Post-turn derivative; limitation documented |
| **O9 later** | Propose generic `registerAssistantContentRenderer` upstream | True inline overlays |

Streaming: show source until complete. Resume from session entries without storing protocol frames.

### 6.3 Codex

| Path | Mechanism |
|------|-----------|
| Structured | Consume documented noninteractive / app-server events; map to MessageEnd |
| Oneshot | `codex … \| beavermath filter` preserving exit status and stderr separation (`CodexAdapter::filter_oneshot`) |

### 6.4 Claude Code

| Path | Mechanism |
|------|-----------|
| Hooks | Map documented stop/response hooks → hook-process JSON |
| Rich wrapper | Optional binary owns TTY; runs Claude underneath **only if** supported without scraping; else document “semantic via hooks only” |

### 6.5 OpenCode

| Path | Mechanism |
|------|-----------|
| Server/SSE/plugin | Map message complete/replace/delete/reconnect to LifecycleStore |
| Oneshot | filter pipeline like Codex |

### 6.6 Universal oneshot (all hosts)

```bash
# Any model stdout
some-agent --print '...' | beavermath filter

# Structured
beavermath render --format jsonl < message.txt
```

This is the compatibility floor. Every host adapter must beat this floor for interactive UX, not replace it.

---

## 7. Phased delivery

### H0 - Shared contract freeze

**Purpose:** one spine, no host forks.

**Work**

- [ ] Document `AdapterEventV1` field table and invariants in `docs/ADAPTER_CONTRACT.md`
- [ ] Add JSON Schema for `AdapterEventV1` under `schemas/`
- [ ] Implement `beavermath-cli` subcommand `hook` (or small `beavermath-hook` bin): JSON in → JSON out
- [ ] Expand golden lifecycle tests: duplicate MessageEnd, out-of-order, cancel mid-render, replace revision bump, delete, resume after process restart (file-backed store optional)
- [ ] File-backed optional `LifecycleStore` persistence for session resume across process boundaries (preview cache only; no raw terminal frames)
- [ ] Entry gate: `cargo test --locked --workspace` green

**Exit gates**

- Schema round-trip + unknown field ignore  
- Machine stdout of hook path: zero ESC/CSI/OSC  
- Same fixtures pass for all adapter wrappers in-tree  

**PRD:** BM-020, BM-025, FR-068-072, FR-095-096

---

### H1 - Universal oneshot polish

**Purpose:** every harness can pipe.

**Work**

- [ ] Stabilize `filter` CLI: preserve trailing newlines, multi-message docs, large inputs within limits  
- [ ] `--format jsonl` one artifact per expression for tooling  
- [ ] Document recipes: Codex/Claude/OpenCode/Grok headless oneshot  
- [ ] Integration tests driving the **installed binary** path  

**Exit gates**

- Pipe adversarial ESC input → escaped or rejected; no raw protocol  
- Dual-run determinism on semantic filter  
- Exit status preserved in documented wrapper examples  

**PRD:** BM-011, BM-034, BM-038, FR-059-061

---

### H2 - Grok Build live (lab first-class)

**Purpose:** derivations in Grok Build yield readable math without manual CLI.

**Work**

- [ ] **Spike:** capture real `Stop` / related hook payloads → `adapters/grok-build/SPIKE.md`  
- [ ] Choose mechanism G2a / G2b / G2c (section 6.1) with recorded rationale  
- [ ] Implement `adapters/grok-build/` : hook scripts + Rust mapping to LifecycleStore  
- [ ] Install path: `~/.grok/hooks/beavermath-stop.json` + binary on PATH  
- [ ] Preview layout and `doctor`-style status for “last turn had N expressions”  
- [ ] Tests: fixture payloads from spike; golden semantic output  
- [ ] Manual lab checklist: interactive session with pythagoras + URL/code protection cases  

**Exit gates**

- Live Grok session: completed turn with `$x^2+y^2=z^2$` produces correct semantic preview via the chosen mechanism  
- Canonical session/model content still contains exact LaTeX (verified by re-read of session store or documented export)  
- Hook failures fail-open  
- No graphics on hook stdout  

**If G2c blocked:** G2a is still a valid H2 pass with explicit UX label “sidecar preview”; track G2c under H7.

**PRD extension:** treat as BM-023-class lab target; add story **BM-039** (proposed) in a PRD amendment when implementing.

---

### H3 - OMP live extension

**Purpose:** PRD BM-021 for real.

**Work**

- [ ] Package OMP extension calling shared hook-process or native Rust via FFI/NAPI only if needed (prefer CLI/RPC for isolation)  
- [ ] message_end → supplemental preview message/widget  
- [ ] Resume/branch fixtures against real OMP if available; else recorded event logs  

**Exit gates**

- Interactive OMP: post-turn preview without model instructions  
- Source immutability + lifecycle fixtures  
- Limitation documented if not true inline  

**PRD:** BM-021, BM-032, FR-090

---

### H4 - Codex live

**Work**

- [ ] Structured event consumer from documented API  
- [ ] Oneshot wrapper script `adapters/codex/oneshot.sh` preserving exit codes  
- [ ] Fixtures + one manual checklist  

**Exit gates:** BM-033, BM-034, FR-092

---

### H5 - Claude Code live

**Work**

- [ ] Hook JSON → beavermath hook-process  
- [ ] Optional rich wrapper design doc; implement only if TTY ownership is clean  
- [ ] Headless hook path must not emit ESC  

**Exit gates:** BM-035, BM-036, FR-093

---

### H6 - OpenCode live

**Work**

- [ ] Server/SSE/plugin event mapping  
- [ ] Reconnect/replay tests  
- [ ] Oneshot filter  

**Exit gates:** BM-037, BM-038, FR-094

---

### H7 - True inline presentation (optional, host-dependent)

**Purpose:** replace sidecar/supplemental with in-transcript display overlays.

**Work**

- [ ] OMP: upstream or patch `registerAssistantContentRenderer`-class API  
- [ ] Grok: propose/implement presentation-only transform in Grok Build if lab controls that surface  
- [ ] Same lifecycle and source rules as semantic path  
- [ ] Pixels only with OutputContextV1 attestation  

**Exit gates:** BM-022 (OMP); Grok equivalent documented  

**Non-blocking:** H2-H6 may claim “chat-readable math” on semantic/sidecar alone.

---

## 8. Repository layout (target)

```text
adapters/
  common/                 # docs + shared scripts
  oneshot/                # shell recipes
  grok-build/
    SPIKE.md
    hooks/beavermath-stop.json
    README.md
    tests/fixtures/*.json
  omp/
  codex/
  claude-code/
  opencode/
crates/beavermath-cli/    # add `hook` subcommand
crates/beavermath-adapter/
docs/
  HARNESS_COMPAT_PLAN.md  # this file
  ADAPTER_CONTRACT.md     # created in H0
  PHASE_GATES.md          # update live vs fixture
schemas/
  adapter_event_v1.json
```

---

## 9. Cross-cutting requirements

### 9.1 Security

- Hook/plugin code: least privilege; no network from core  
- Preview dirs: owner-only (`0700` / `0600`)  
- Never log full prompts by default; digests preferred  
- Untrusted math stays in closed TeX set  

### 9.2 Performance

- Render only on **complete** content (not every token)  
- Debounce replace events  
- Respect FR-046 limits; adapters may lower, not raise  
- Cancel in-flight work on Cancel/StopFailure  

### 9.3 Testing layers

| Layer | What |
|-------|------|
| Unit | LifecycleStore, protect, hook JSON parse |
| Golden | semantic filter for fixed messages |
| Fixture | host event JSON from spikes |
| Binary | `cargo`-built `beavermath hook` / `filter` |
| Manual lab | Grok interactive checklist; OMP if available |
| Machine purity | scan stdout for ESC on all hook paths |

### 9.4 Explicit non-goals (this track)

- Screen scraping TUIs  
- Mutating canonical session text to semantic  
- Sixel / active probing / SSH-tmux graphics (Phase 13)  
- Public crates.io publish (Phase 12)  
- Teaching the model special math tools  

---

## 10. Proposed PRD amendment (when H2 starts)

Add:

**BM-039: Grok Build first-class adapter**

- As a Grok Build user, I want completed assistant math rendered automatically without tool calls.  
- Acceptance: documented mechanism (G2a/b/c), source preserved, fail-open hooks, machine-channel purity, lab checklist pass.  

Update harness matrix table to include Grok Build.

---

## 11. Exit criteria for the whole track

Track is **complete** when:

1. H0 + H1 green.  
2. **At least H2 (Grok) live** at G2a or better, with recorded spike.  
3. **At least one of H3-H6 live** beyond fixtures (prefer H3 OMP for public PRD alignment).  
4. Remaining hosts either live or documented as “oneshot-only” with recipes.  
5. `docs/PHASE_GATES.md` distinguishes **fixture** vs **live** for each host.  
6. No v1 claim of true inline unless H7 evidence exists.  

---

## 12. First implementation sprint (concrete)

**Sprint goal:** H0 + H2 spike decision + H1 filter recipes.

| Day focus | Deliverable |
|-----------|-------------|
| 1 | `ADAPTER_CONTRACT.md`, `adapter_event_v1.json`, lifecycle golden tests |
| 2 | `beavermath hook` JSON stdin/stdout binary + tests |
| 3 | Grok `Stop` payload spike → `adapters/grok-build/SPIKE.md` |
| 4 | Implement chosen G2 path skeleton + install hook |
| 5 | Manual Grok math turn checklist; fix gaps; update PHASE_GATES |

After sprint: either G2a sidecar is usable for daily lab work, or SPIKE proves we need Grok-side presentation work (H7) and we escalate.

---

## 13. Open decisions (resolve in H0/H2, do not block writing the contract)

| ID | Question | Default if unresolved |
|----|----------|------------------------|
| D1 | Grok Stop payload contains full assistant text? | Spike; else session-file read if documented |
| D2 | Preview storage path global vs project | `~/.beavermath/previews/` default |
| D3 | Native extension vs CLI hook-process for OMP | CLI/RPC first (isolation) |
| D4 | File-backed LifecycleStore in v1 | Yes for cross-process resume of previews |
| D5 | Serialize H2 before H3? | Yes for this lab |

---

## 14. Relation to ROADMAP phases

| ROADMAP | This track |
|---------|------------|
| Phase 8 | H3 live (replaces fixture-only claim) |
| Phase 9 | H7 OMP slice |
| Phase 11 | H4-H6 live |
| New | H2 Grok (lab) + H0/H1 spine |
| Phase 7 | Unchanged consumer for TTY pixels |
| Phase 12-13 | Independent; do not wait |

---

*End of harness compatibility plan.*
