# Phase exit-gate ledger

## OMP + OpenCode track (this goal)

| Step | Status | Evidence |
|------|--------|----------|
| Present spine (IP0) | **live** | `beavermath present`; dual-run + filter match |
| **O0** OMP SPIKE | **live** | `adapters/omp/SPIKE.md` |
| **O1** OMP extension present | **live** | `adapters/omp/extension/` + fixture test |
| **O2** OMP in-session UX | **live (partial)** | custom `beavermath.math-preview` + display; **not** assistant-body inline |
| **O3** true content renderer | **blocked** | no OMP API |
| **C0** OpenCode SPIKE | **live** | `adapters/opencode/SPIKE.md` |
| **C1** OpenCode plugin present | **live** | `adapters/opencode/plugin/` + fixture test |
| **C2** OpenCode bubble paint | **API-blocked** | would require mutating stored text; oneshot retained |
| Oneshot OpenCode | **live** | `oneshot.sh` |

## In-chat product bar (global)

See [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md). Sidecar/oneshot alone ≠ full bar.  
OMP O2 = automatic in-session preview (honest partial). OpenCode interactive paint blocked.

## Prior notes

| Item | Status |
|------|--------|
| Grok stock TUI | blocked (closed binary); `grok-session` VIEWPORT interim |
| Core ROADMAP 0–7, 10 | pass |
| Phase 12 publish | **done (GitHub public 0.1.0)** — tag `v0.1.0`, digests [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md); crates.io still deferred |
| Phase 13 | **scoped** — [PHASE13_SCOPE.md](PHASE13_SCOPE.md); not started. Tracks 13A–D post-v1 protocols; harness bar (Claude/Codex IP3/IP4) is Track H, not 13 |

## Critical invariants

1. Exact source never rewritten by present.  
2. auto_protect on URL/code.  
3. Bare ASCII not auto-math.  
4. Machine JSON zero ESC.  
5. Fail-open host extensions.
