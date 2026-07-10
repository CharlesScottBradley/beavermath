# Phase exit-gate ledger

## Publication

| Item | Status | Evidence |
|------|--------|----------|
| Path-clean `main` history | **live** | orphan clean tree; full-history lab-path scan 0 |
| CI fmt/clippy/test/deny | **live** | `.github/workflows/ci.yml` green on recent main |
| Public visibility | **not flipped** | repo PRIVATE |
| Next public freeze | **`v0.1.1` planned** | [BEST_PUBLIC_PATH.md](BEST_PUBLIC_PATH.md), [GO_PUBLIC_PLAN.md](GO_PUBLIC_PLAN.md) |
| Historical private tag `v0.1.0` | **lags main** | not the public freeze; do not open on this tag alone |
| crates.io | **deferred** | all crates `publish = false` |

## OMP + OpenCode

| Step | Status | Evidence |
|------|--------|----------|
| Present spine (IP0) | **live** | `beavermath present`; dual-run + filter match |
| **O0** OMP SPIKE | **live** | `adapters/omp/SPIKE.md` |
| **O1** OMP extension present | **live** | `adapters/omp/extension/` + fixture test |
| **O2** OMP in-session UX | **live (partial)** | custom preview + widget; not assistant-body inline |
| **O3** true content renderer | **blocked** | no OMP API |
| **C0** OpenCode SPIKE | **live** | `adapters/opencode/SPIKE.md` |
| **C1** OpenCode plugin present | **live** | `adapters/opencode/plugin/` + fixture test |
| **C2** OpenCode bubble paint | **API-blocked** | would require mutating stored text; oneshot retained |
| Oneshot OpenCode | **live** | `oneshot.sh` |

## In-chat product bar (global)

See [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md). Sidecar/oneshot alone is not the full bar.  
OMP O2 is automatic in-session preview (honest partial). OpenCode interactive paint blocked.

## Other hosts

| Item | Status |
|------|--------|
| Grok stock TUI | blocked (closed binary); `grok-session` VIEWPORT interim |
| Claude Code / Codex | oneshot-oriented |
| Core ROADMAP 0-7 | pass (implementation present) |
| Phase 13 | **scoped, not started** ([PHASE13_SCOPE.md](PHASE13_SCOPE.md)); Track H harness bar is separate |

## Critical invariants

1. Exact source never rewritten by present.  
2. auto_protect on URL/code.  
3. Bare ASCII not auto-math.  
4. Machine JSON zero ESC.  
5. Fail-open host extensions.

## Wave F pre-flip governance (E7 residual)

Private free accounts cannot set branch protection (403). Do these **before** or **immediately when** flipping public:

| Control | Status now | Action at flip |
|---------|------------|----------------|
| Branch protection + required CI | blocked private free | enable on public `main` |
| Secret scanning / push protection | check Settings | enable |
| Dependabot alerts | config present | confirm enabled |
| Security Advisories | not yet | enable + update SECURITY.md |
| Tag protection `v*` | optional | enable if available |

Files already in tree: `.github/dependabot.yml`, `CONTRIBUTING.md`, issue templates.

