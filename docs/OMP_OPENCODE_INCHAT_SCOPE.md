# Scope: OMP + OpenCode automatic math presentation

**Status:** next implementation track (after Grok paint blocked)  
**Date:** 2026-07-10  
**Product bar:** [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md)  
**Shared API (already shipped):** `beavermath present` / `present_message_v1`  
**Hosts on this machine:** `omp` v16.3.15 (`~/.bun/bin/omp`), `opencode` (`~/.opencode/bin/opencode`)

---

## 1. Outcome (what “done” means)

When you derive in **OMP** or **OpenCode** interactive chat, and the assistant finishes a turn that contains explicit math:

```text
Stored / model context:  "… $x^2+y^2=z^2$ …"
Viewport / same message region: readable presentation
  e.g. "(x) to the power of (2) plus …"
```

| Must | Must not |
|------|----------|
| Automatic after message complete | Model tool call or prompt instruction |
| Exact source immutable for model/session | Rewrite `chat_history` / provider content to semantic |
| URL / code auto-protected | Screen-scrape TUI |
| Machine channels ESC-free | Claim sidecar-only as product bar |
| Fail-open (present error → show source) | Raise limits or network from core |

### “ASCII math” vs LaTeX (scope clarity)

| Input style | Default behavior | In this track |
|-------------|------------------|---------------|
| Explicit LaTeX `$...$`, `$$...$$`, `\(...\)`, `\[...\]`, fenced math | Auto present | **Primary path** |
| Bare ASCII `x^2+y^2=z^2` (no delimiters) | Strict mode leaves as text | **Opt-in only** (`repair` + consent / host policy); not silent auto |
| Currency / code / URL `$` | Not math | Keep auto_protect |

If the lab later wants silent bare-ASCII auto-math, that is a **separate policy decision** (false-positive risk). This track defaults to **explicit delimiters auto**, bare ASCII via **opt-in repair presentation**.

---

## 2. Shared spine (do not reimplement)

```text
message complete (host event)
        │
        ▼
  beavermath present   (or present_message_v1 in-process)
        │
        ├─ exact_source, content_sha256  → store / model (unchanged)
        └─ display_text, spans           → host paint only
```

Dependencies already green: IP0 present API, filter match tests, machine purity.

Optional later: NAPI/WASM bindings so OMP/OpenCode TS does not shell out every turn (perf). **v1 may shell out** to `beavermath present` for isolation.

---

## 3. OMP track (host is installed)

### 3.1 Documented extension surfaces (OMP / oh-my-pi)

From upstream extension guide:

| API | Use for BeaverMath |
|-----|-------------------|
| `pi.on("message_end" \| …)` | Trigger present on completed assistant content |
| `pi.on("message_start" \| "message_update")` | Source while streaming (no present swap mid-stream) |
| `registerMessageRenderer(type, …)` | Render **custom** message types only |
| `registerAssistantThinkingRenderer` | Display-only under **thinking** blocks (not main assistant body) |
| `setWidget` / `setHookWidget` | Supplemental UI above/below editor (not in-message) |
| `sendMessage` / `appendEntry` | Supplemental custom message (interim, not product bar alone) |

**Gap (product bar):** no documented `registerAssistantContentRenderer` for normal assistant text spans. True inline still needs upstream or a verified display-override API.

### 3.2 Delivery ladder (OMP)

| Step | Mechanism | Meets product bar? |
|------|-----------|--------------------|
| **O0** | Spike: real `message_end` payload fields (content, id, revision) on local `omp` | Scaffold |
| **O1** | Extension package: on `message_end` → `beavermath present` → store presentation by message id | Scaffold |
| **O2** | **Best current UX:** present + `registerMessageRenderer` for a bound custom type **or** widget that is clearly tied to the turn; document limitation | **Partial** (acceptable lab interim if labeled) |
| **O3** | True assistant-content renderer (patch/PR OMP or new API) | **Yes** |
| **O4** | Optional: `--mode unicode` / pixel refs when TTY capability known | Enhancement |

### 3.3 OMP package layout (target)

```text
adapters/omp/
  package/                 # installable OMP extension
    package.json / omp manifest
    src/index.ts           # pi.on + present subprocess
    src/present.ts         # spawn beavermath present, parse JSON
  tests/
    fixtures/message_end.json
    present-integration.test.ts
  SPIKE.md                 # payload findings
  INCHAT.md                # status (update as steps complete)
  extension-preview.md     # keep as history
```

### 3.4 OMP install path (lab)

```bash
# Extension discovery: OMP profiles / extension dirs (confirm in O0 spike)
# Typical pattern: package under ~/.omp/... or project extension path
omp  # interactive with extension loaded
```

### 3.5 OMP exit gates

| Gate | Pass criterion |
|------|----------------|
| O0 | SPIKE.md lists real event field names + sample redacted payload |
| O1 | Fixture `message_end` → present JSON; digests stable; source ≠ display when math present |
| O2 | Live interactive OMP turn with `$x^2+y^2=z^2$` shows presentation **in-session** (custom message/widget acceptable only if user does not need a second manual file open); source in session export still exact LaTeX |
| O3 | Same turn paints **inside** normal assistant content region; disabling extension restores source paint |
| Safety | URL/`$user$`, `` `$x$` `` protected; fail-open; no ESC on machine channels |

---

## 4. OpenCode track (host is installed)

### 4.1 Surfaces on this machine

- CLI: `opencode` (TUI default), `opencode run`, `opencode serve` / `web`, `opencode plugin`, `opencode acp`
- Config: `~/.config/opencode/` with `@opencode-ai/plugin`
- Oneshot already: `adapters/opencode/oneshot.sh`

### 4.2 Delivery ladder (OpenCode)

| Step | Mechanism | Meets product bar? |
|------|-----------|--------------------|
| **C0** | Spike plugin API: message complete / session events / UI re-render hooks | Scaffold |
| **C1** | Plugin module: on assistant complete → `beavermath present` → cache by message id + hash | Scaffold |
| **C2** | Wire UI re-render: message component paints `display_text` if present cache hit | **Yes** if in message region |
| **C3** | `opencode run` / print path stays oneshot filter (already) | Print bar only |
| **C4** | Server/SSE path: same present contract for web UI | Yes for web if same paint |

If plugin API cannot re-render assistant bubbles, label **API-blocked** and keep oneshot + optional side panel (not product bar).

### 4.3 OpenCode package layout (target)

```text
adapters/opencode/
  plugin/                  # opencode plugin module
    index.ts / package.json
  oneshot.sh               # keep
  SPIKE.md
  INCHAT.md                # update
  tests/fixtures/
```

### 4.4 OpenCode exit gates

| Gate | Pass criterion |
|------|----------------|
| C0 | SPIKE.md: event names, how to get exact assistant text, whether UI can re-paint |
| C1 | Fixture complete event → present; source immutable |
| C2 | Live `opencode` TUI turn with `$E=mc^2$` shows semantic **in the assistant bubble** (or honest blocked) |
| Print | `opencode run … \|` or oneshot wrapper still clean |
| Safety | Same as OMP |

---

## 5. Dependency graph

```text
IP0 present API (done)
        │
        ├─► O0 OMP payload spike
        │      └─► O1 extension present wiring
        │             └─► O2 best in-session UX (widget/custom)
        │                    └─► O3 true content renderer (may block on upstream)
        │
        └─► C0 OpenCode plugin spike
               └─► C1 plugin present wiring
                      └─► C2 TUI/web re-render (or blocked label)
```

**Parallelism:** O0–O2 and C0–C2 may run in parallel after present is stable.  
**Serial:** O3 only after O2 and after host API research.

**Does not depend on:** Grok paint unblock, Phase 12 publish, Phase 13 protocols.

---

## 6. Implementation notes (specific)

### 6.1 Calling present from TypeScript

```ts
import { spawnSync } from "node:child_process";

export function presentSource(source: string): {
  exact_source: string;
  display_text: string;
  content_sha256: string;
} {
  const r = spawnSync("beavermath", ["present", "--mode", "semantic"], {
    input: source,
    encoding: "utf8",
    maxBuffer: 16 * 1024 * 1024,
  });
  if (r.status !== 0) throw new Error(r.stderr || "present failed");
  return JSON.parse(r.stdout);
}
```

Cache: `Map<content_sha256, PresentationV1>` process-local.

### 6.2 Streaming

- While `message_update`: show **source** only.  
- On `message_end`: compute present once; swap display.  
- On replace/delete: recompute or drop cache entry.

### 6.3 Bare ASCII (opt-in)

Host flag e.g. `BEAVERMATH_ASCII_REPAIR=1` or extension setting:

1. Run present on source (strict).  
2. If no expressions and consent policy on, call `beavermath repair --consent` on candidate spans only, then present **inferred** TeX as display with provenance footer.  
3. Exact original always stored; never replace store with inferred TeX.

Default: **off**.

### 6.4 Tests

| Test | Drives |
|------|--------|
| Unit present (existing) | Shared API |
| Fixture host events | O1/C1 mappers |
| Binary: `beavermath present` dual-run | Stability |
| Manual lab checklist | O2/C2 interactive |
| Machine purity scan | Extension logs / plugin stdout |

---

## 7. Non-goals

- Patching Grok stock TUI (separate track).  
- Silent bare-ASCII auto-math as default.  
- Mutating session source to semantic.  
- Screen scraping OMP/OpenCode TUIs.  
- Requiring true O3 if O2 delivers clear in-session readability and O3 is upstream-blocked (label O3 deferred).

---

## 8. Track success criterion

Track **complete** when:

1. OMP: O0–O1 green, and **O2 live** (in-session automatic presentation after turns with explicit LaTeX) **or** O3 live.  
2. OpenCode: C0–C1 green, and **C2 live** **or** explicit API-blocked with oneshot remaining.  
3. Both: source immutability + auto_protect + fail-open documented and tested.  
4. `PHASE_GATES.md` / per-host `INCHAT.md` updated with fixture vs live honesty.  
5. No claim that oneshot alone meets interactive product bar.

---

## 9. First sprint (suggested)

| Day | Work |
|-----|------|
| 1 | O0 + C0 spikes (payloads, plugin API) → SPIKE.md each |
| 2 | O1 extension skeleton + present wiring + fixtures |
| 3 | C1 plugin skeleton + present wiring + fixtures |
| 4 | O2 best UX in live `omp` |
| 5 | C2 live `opencode` or blocked label; PHASE_GATES; demos |

---

*End of OMP/OpenCode in-chat scope.*
