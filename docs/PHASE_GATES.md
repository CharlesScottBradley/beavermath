# Phase exit-gate ledger

## Publication

| Item | Status | Evidence |
|------|--------|----------|
| Path-clean `main` history | **live** | orphan clean tree; lab-path scan 0 |
| CI fmt/clippy/test/deny | **live** | green on freeze tip `fe4603d` |
| Freeze tag | **`v0.1.1`** | `git rev-list -n1 v0.1.1` equals HEAD at freeze |
| Release assets | **live (public)** | macOS aarch64+x86_64 remapped; SBOM scrubbed |
| Public visibility | **public** | flipped 2026-07-10 |
| crates.io | **deferred** | all crates `publish = false` |

## Wave F pre-flip governance

| Control | Status | Action |
|---------|--------|--------|
| Branch protection + required CI | **live** | required checks on `main` |
| Secret scanning / push protection | check Settings | enable if not already |
| Dependabot config | **in tree** | confirm alerts enabled |
| Security Advisories | prefer enabled | Settings → Code security |
| CONTRIBUTING + issue templates | **in tree** | live |

## OMP + OpenCode

| Step | Status | Evidence |
|------|--------|----------|
| Present spine (IP0) | **live** | `beavermath present` |
| O0-O2 OMP | **live (partial)** | extension preview/widget; not body-inline |
| O3 content renderer | **blocked** | no OMP API |
| OpenCode C0-C1 | **live** | plugin + oneshot |
| OpenCode C2 bubble | **API-blocked** | oneshot retained |

## Other hosts

| Item | Status |
|------|--------|
| Grok stock TUI | blocked; VIEWPORT interim |
| Claude / Codex | oneshot-oriented |
| Phase 13 | scoped, not started |

## Critical invariants

1. Exact source never rewritten by present.
2. auto_protect on URL/code.
3. Bare ASCII not auto-math.
4. Machine JSON zero ESC.
5. Fail-open host extensions.
