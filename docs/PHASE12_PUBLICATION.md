# Phase 12: publication checklist

**Status:** private candidate; **public flip NOT done**.  
**Next public cut:** **`v0.1.1`** per [GO_PUBLIC_PLAN.md](GO_PUBLIC_PLAN.md).  
**Do not** treat the earlier private `v0.1.0` tag as the public freeze: main moved (CI polish + honesty docs) after that tag.

## Current facts

| Field | Value |
|-------|--------|
| Visibility | **PRIVATE** |
| crates.io | Deferred (`publish = false`) |
| Path-clean `main` history | yes (orphan clean tree) |
| CI on recent main | green after fmt/clippy fix |
| Tag lag | `v0.1.0` behind HEAD; re-freeze required |

## Why 0.1.1

1. Tag must equal freeze commit must equal binary build source.  
2. HEAD includes CI and honesty fixes after `v0.1.0`.  
3. Shipping public on a lagging tag is a freeze integrity failure.

## Digests

See [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md). Rewrite that file when `v0.1.1` binaries are built.

## Controlled sequence (summary)

Full steps: [GO_PUBLIC_PLAN.md](GO_PUBLIC_PLAN.md).

1. Docs: simple README, CHANGELOG 0.1.1, SUPPORT, honest adapter status.  
2. Local + GitHub CI green on freeze SHA.  
3. Rebuild path-remapped macOS binaries; scrub SBOM; strings scan.  
4. Commit digests; tag `v0.1.1`.  
5. Explicit maintainer **approve public**.  
6. GitHub Release attach assets; re-scan.  
7. Visibility public.  
8. Security Advisories + clean-machine smoke.  

## Known limitations (public claims)

- Not full multi-OS 1.0 matrix  
- OMP O2 partial; O3 blocked  
- OpenCode / Grok / Claude / Codex not full in-chat body paint  
- crates.io deferred  
- `RUSTSEC-2026-0192` ttf-parser via RaTeX documented ignore  

## Follow-ups after public

- Linux binary via CI  
- crates.io when ready  
- Signatures/attestations  
- Phase 13 post-v1 protocols  
