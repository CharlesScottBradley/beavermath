# Phase 13 scope — post-v1 terminal and semantic expansion

**Status:** scoped (not started)  
**Date:** 2026-07-10  
**Depends on:** public `v0.1.0` (Phase 12 GitHub cut complete)  
**Canonical phase text:** [ROADMAP.md](../ROADMAP.md) § Phase 13  
**PRD:** BM-013 (safe probe), BM-016 (Sixel), FR-036 / FR-040 / FR-062, open post-v1 decisions  
**Does not delay or redefine:** supported `0.1.x` / future `1.0.0` claims

---

## 1. What Phase 13 is

Phase 13 adds **optional, experimental terminal and semantic surfaces** that were deliberately excluded from v1:

| Track | Name | Outcome |
|-------|------|---------|
| **13A** | Experimental safe probing | `probe` only in an **experimental** build; exclusive lease; hard deadlines; termios restore |
| **13B** | Sixel | Bounded monochrome append-only encoder + hostile DCS parser; real-terminal evidence |
| **13C** | SSH / tmux graphics | End-terminal confirmation + direct Kitty + **exact one-layer** tmux wrapper; fail closed otherwise |
| **13D** | Repair locales / a11y | Extra host-owned repair providers; frozen locale profiles; MathML navigation where hosts expose trees |

**Hard rule from ROADMAP:** this phase carries **no v1 compatibility claim**. Shipping any of 13A–D must not change default `0.1.x` behavior, default CLI modes, or public “supported” matrix rows without a new version + claim review.

---

## 2. What Phase 13 is not

Keep these on **separate tracks**. Do not bury them under “Phase 13” or they will dilute protocol work and re-open publication scope.

| Track | Content | Why separate |
|-------|---------|--------------|
| **12R** Phase 12 residual | crates.io when ready; Linux/Windows archives; signatures/attestations; fuller multi-arch matrix | Supply-chain / release, not protocols |
| **H** Harness product bar | Grok P1 paint, OMP O3 content renderer, Claude IP3, Codex IP4, OpenCode C2 | Host APIs; see [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md) |
| **11R** Adapter honesty | Claude/Codex oneshot already live; document P1/P2/P3 spikes without claiming interactive | Phase 11 residual / IP track |

**Claude / Codex recommendation:** run as **Track H (IP3 / IP4)** in parallel with early 13A design if desired. They are **not** 13A–D deliverables. Product bar for Claude/Codex remains host-display override or honest oneshot-only labels ([adapters/claude-code/INCHAT.md](../adapters/claude-code/INCHAT.md), [adapters/codex/INCHAT.md](../adapters/codex/INCHAT.md)).

---

## 3. Entry criteria (must be true before claiming Phase 13 start)

1. **Public `v0.1.0` is live** and digests still match [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md).
2. **Default build still:** zero queries, no Sixel, SSH/tmux/screen **text fallback only**, no `probe` in the supported binary.
3. **Threat model re-read** for terminal frames (typed frames, machine-channel purity, no raw model strings in controls). Document any delta in this file or a short `docs/THREAT_PHASE13.md` if the model grows.
4. **Feature flags / cargo features agreed** so experimental code cannot leak into default release claims by accident (proposed below).
5. **Maintainer decide sequencing:** protocol-first (13A→13B→13C→13D) vs harness-first (Track H) vs dual track with explicit owners.

---

## 4. Packaging and claim discipline

### 4.1 Proposed build split

| Artifact | Features | Claims |
|----------|----------|--------|
| Default `beavermath` (0.1.x line) | no `probe`, no Sixel, no mux graphics | Same as public 0.1.0 terminal policy |
| Experimental build e.g. `beavermath-exp` or feature `experimental-protocols` | 13A–C behind flags | Explicitly experimental; README “not supported v1” |
| CI job | default + experimental matrix rows labeled | Fail if experimental frames appear in default machine-channel tests |

### 4.2 CLI surface (reserved names)

| Command / flag | Phase | Default build |
|----------------|-------|---------------|
| `beavermath probe` | 13A | **Absent** or hard-error “experimental build only” |
| `--probe=safe` | 13A | Experimental only; requires allowlisted candidate + lease |
| Sixel encoder path | 13B | Off unless trusted capability + experimental feature |
| Mux / SSH pixel path | 13C | Off unless end-terminal confirm + one-layer tmux proven |

### 4.3 Invariants that still block ship (all tracks)

From ROADMAP §6.2 and [PHASE_GATES.md](PHASE_GATES.md): exact source identity; no ESC on machine channels; no undelimited auto-math; visual failure must not reorder semantic output; adapters never rewrite canonical content.

---

## 5. Work packages

### 13A — Experimental safe probing

**Problem:** v1 refuses all queries. Some post-v1 paths need confirmed capability without stealing keystrokes or leaving termios dirty.

**Scope in**

- Explicit `--probe=safe` (or `beavermath probe --mode safe`) **only** in experimental builds.
- Named **allowlisted** probe candidates (fixed request/response grammars; no open-ended “send anything”).
- Preconditions: foreground controlling TTY; same input and output terminal; exclusive input lease.
- Deadlines: **250 ms** direct; **1 s** SSH/tmux-class; **2 s** absolute max.
- Restore exact termios (and related TTY state) on success, timeout, malformed response, signal, panic, cancel.
- Unrelated input must be preserved (or session fails closed without consuming foreign bytes into application logic).
- A named graphics **preference** alone never forces pixels.

**Scope out**

- Probe in default release binary as a supported feature.
- Probing from redirected/non-TTY or headless CI as a success path.
- Using probe results to upgrade claim matrix without real-terminal rows.

**Exit gates**

- [ ] Adversarial PTY tests: short writes, mid-probe SIGINT, concurrent paste, wrong-tty, timeout.
- [ ] Property: after any outcome, termios snapshot equals pre-probe snapshot (or documented safe restore).
- [ ] Default build: no probe binary path / no probe sequences in doctor or present.
- [ ] BM-013 checklist items pass.

**Risk:** probe becomes a footgun for “just enable pixels.” Mitigation: experimental package name + claim ban in README/CHANGELOG.

**Rough effort:** design + PTY harness 3–5 days; allowlist + restore edge cases 2–4 days. **Total ~1–2 weeks** focused.

---

### 13B — Sixel

**Problem:** Sixel expands attack surface (DCS, repeat, palette, cursor). Excluded from v1 for that reason (FR-036).

**Scope in**

- Bounded **response parser** (reject DA1 false positives, palette mutation, unbounded repeat, oversized dims, unterminated DCS).
- **Monochrome append-only** encoder first (no deletion protocol dependence in v1.x experimental).
- Preserve cursor/rendition state around the sequence.
- Real-terminal corpus on named Sixel emulators (ROADMAP: current real Sixel terminals).
- Capability still requires trusted attestation or 13A probe result; preference alone insufficient.

**Scope out**

- Color Sixel, ReGIS, arbitrary DCS payloads.
- Claiming cross-emulator pixel identity.
- Default-build “Sixel supported” marketing.

**Exit gates**

- [ ] Hostile DCS / repeat / size fuzz corpus green.
- [ ] PTY adversarial: partial write leaves terminal usable; semantic already complete before pixels.
- [ ] At least one real Sixel terminal row documented with date, version, pass/fail.
- [ ] BM-016 checklist items pass.
- [ ] Default build still rejects Sixel frames in machine channels.

**Risk:** false “works on my xterm” without parser hostility. Mitigation: parser tests before encoder polish.

**Rough effort:** parser + corpus 1–2 weeks; encoder + real terminal 1 week. **Total ~2–3 weeks**.

**Dependency:** 13A optional for auto-detect; **not required** if capability is host-attested only.

---

### 13C — SSH and tmux graphics

**Problem:** remote and multiplexed paths strip or mis-handle graphics; nested mux and GNU screen are untrusted (FR-040).

**Scope in**

- End-terminal confirmation (who actually paints).
- Direct Kitty transfer when confirmed.
- **Exact one-layer** tmux passthrough wrapper only (documented version + config assumptions).
- Fail to text on: hidden panes, stripped passthrough, reattachment, stale identity, nested multiplexer, timeout.

**Scope out**

- GNU screen graphics.
- Nested tmux / tmux-in-ssh-in-tmux “best effort.”
- Claiming SSH without end-terminal confirmation.

**Exit gates**

- [ ] Matrix rows: direct local Kitty (baseline); SSH+PTY text; SSH+confirmed Kitty if in scope; tmux one-layer pass; nested fail-closed; screen text-only.
- [ ] No graphics bytes when fallback is selected (byte scan).
- [ ] Identity/stale session fixtures (reattach, rename, pane hide).

**Risk:** flaky CI without real mux fixtures. Mitigation: scripted tmux in CI where available; manual signed matrix for the rest.

**Rough effort:** design + fail-closed policy 3–5 days; Kitty-through-one-layer tmux 1–2 weeks; SSH confirm path 1 week. **Total ~3–4 weeks**.

**Dependency:** strong on typed Kitty frames already in v1; 13A useful for remote deadlines.

---

### 13D — Repair providers, locales, MathML navigation

**Problem:** repair is opt-in and host-owned; a11y locales and structured navigation are incomplete.

**Scope in**

- Additional **host-owned** repair providers only after quality, privacy, reproducibility reviews (provider-specific).
- Locales via **frozen semantic profiles** + native-speaker review (no silent inventing meaning).
- Explore structured MathML navigation where the host exposes an accessibility tree.

**Scope out**

- Default model-assisted undelimited reconstruction (still deferred by design).
- Repair as authority over exact source.
- Shipping locales without frozen profiles.

**Exit gates**

- [ ] Each new provider has review note (privacy, determinism, failure mode).
- [ ] Locale packs versioned; golden semantic fixtures per locale.
- [ ] Repair remains opt-in; provenance fields required.
- [ ] MathML path documented as host-dependent, not core TTY claim.

**Risk:** quality theater without native review. Mitigation: block locale claim until review artifact exists.

**Rough effort:** highly variable; **plan 1–2 weeks** for one locale + one provider skeleton, then stop for review.

**Dependency:** Phase 6 repair contract already present; no need to wait for 13B/C.

---

## 6. Suggested sequence

### Protocol-first (matches ROADMAP literal order)

```text
13A design + experimental feature gate
  → 13B Sixel (parser before encoder)
  → 13C SSH/tmux (fail-closed matrix)
  → 13D locales/providers as capacity allows
```

**Best when:** terminal protocol completeness is the product priority after 0.1.0.

### Harness-first (user interest: Claude / Codex)

```text
Track H: IP3 Claude spike → honest P1/P2/P3 or oneshot-only freeze
        IP4 Codex app-server / TUI display split spike
        (Grok P1 / OMP O3 remain host-blocked; document only)
Then enter 13A when harness track hits a wall or weekly capacity frees.
```

**Best when:** day-to-day UX in agent TUIs matters more than Sixel/SSH pixels.

### Dual track (recommended default for this lab)

| Owner focus | Work |
|-------------|------|
| **Core/terminal** | 13A feature gate + PTY lease design; then 13B parser corpus |
| **Adapters** | Claude IP3 + Codex IP4 spikes; update INCHAT honesty; no false “live in-chat” |

No shared binary claim until both tracks pass their own gates. Release notes list tracks separately.

---

## 7. Success definition (phase complete)

Phase 13 is **complete** only when:

1. 13A–13C either **ship experimental with full gates** or are **explicitly deferred again** with dated reasons (no silent half-claims).
2. Default public builds still match v1 terminal policy (zero query; no Sixel; mux/SSH text fallback).
3. Experimental artifacts, if published, have separate version/channel naming and digests.
4. CHANGELOG / README / SUPPORT matrix rows distinguish supported vs experimental.
5. 13D either delivered for at least one reviewed locale/provider or deferred with open PRD checkboxes unchanged.

**Phase 13 does not require** crates.io, full multi-arch, or full in-chat product bar. Those are 12R / H.

---

## 8. Immediate next actions (when execution starts)

Pick one:

| Option | First concrete step |
|--------|---------------------|
| **A. Protocol** | Cargo feature `experimental-protocols`; stub `probe` that refuses outside feature; PTY lease design doc + failing tests |
| **B. Harness** | Claude Code documented hook inventory + spike for display-only transform; Codex app-server event list for completed messages |
| **C. Dual** | A day-1: feature gate + refuse-by-default probe; B day-1: Claude/Codex spike notes into INCHAT.md with pass/fail |

**Do not start:** Sixel encoder without parser hostility corpus; tmux graphics without fail-closed matrix; any default-build claim change.

---

## 9. Open decisions for maintainer

1. **Sequence:** A protocol / B harness / C dual (lab default recommendation: **C**).
2. **Experimental distribution:** separate binary name vs cargo feature on same crate.
3. **13C priority:** skip SSH/tmux graphics entirely until a real lab user path needs them?
4. **13D priority:** defer until a named locale/provider request exists?
5. **Versioning:** experimental work on `main` behind flags, or `exp/phase13` branch until 13A gates green?

---

## 10. Traceability

| Source | Link |
|--------|------|
| ROADMAP Phase 13 | [ROADMAP.md](../ROADMAP.md) |
| Deferred by design | ROADMAP §8 |
| PRD probe / Sixel | BM-013, BM-016 |
| In-chat product bar | [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md) |
| Phase gates ledger | [PHASE_GATES.md](PHASE_GATES.md) |
| Public 0.1.0 digests | [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md) |
| Phase 12 freeze | [PHASE12_PUBLICATION.md](PHASE12_PUBLICATION.md) |

---

*Scoped after public 0.1.0 flip. Execution starts only after entry criteria + sequence decision.*
