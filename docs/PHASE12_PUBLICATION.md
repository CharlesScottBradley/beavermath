# Phase 12: publication checklist

**Status:** private freeze **`v0.1.1` complete**; public flip is a separate Wave F step.  
**Canonical plans:** [BEST_PUBLIC_PATH.md](BEST_PUBLIC_PATH.md), [GO_PUBLIC_PLAN.md](GO_PUBLIC_PLAN.md).

## Freeze facts

| Field | Value |
|-------|--------|
| Tag | `v0.1.1` |
| Verify | `git rev-list -n1 v0.1.1` equals freeze tip |
| Workspace version | `0.1.1` |
| Visibility at freeze | PRIVATE |
| crates.io | Deferred (`publish = false`) |
| Digests | [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md) |
| Assets | macOS aarch64 + x86_64 remapped/stripped; SBOM path-scrubbed |

## Done for freeze

- [x] Simple README + honest SUPPORT/CHANGELOG/SECURITY/PHASE_GATES
- [x] Path-clean history
- [x] CI green on freeze tip
- [x] Remapped binaries + digests + private GitHub Release
- [x] Dependabot + CONTRIBUTING + issue templates
- [x] release-prep manual + path hygiene

## Wave F (public flip) remaining

1. Explicit maintainer approve (received when executing flip)
2. Set visibility public
3. Enable Security Advisories + secret scanning as available
4. Branch protection + required CI (available when public)
5. README status line to public 0.1.1
6. Clean-machine smoke recorded

## Known limitations (claims)

- Not full multi-OS 1.0 matrix
- OMP O2 partial; O3 blocked
- OpenCode / Grok / Claude / Codex not full in-chat body paint
- crates.io deferred
- `RUSTSEC-2026-0192` ttf-parser via RaTeX documented ignore
