# Phase 12: Public 0.1.0 publication

**Status:** **executing / completed for GitHub public 0.1.0** (see checklist below)

## Approved cut

| Field | Value |
|-------|--------|
| Version | **0.1.0** (public pre-approved) |
| Scope | Source + macOS binaries + SBOM; not full multi-OS 1.0.0 matrix |
| crates.io | Deferred (`publish = false`) |
| Public flip | **Approved** by maintainer 2026-07-10 |

## Digests

See [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md).

## Sequence

1. Freeze digests + SBOM + deny.toml in git  
2. Tag `v0.1.0` on freeze commit  
3. Push tag to origin  
4. Set repository visibility **public**  
5. Create GitHub Release `v0.1.0` attaching **exact** prebuilt tar.gz + SHA256SUMS + SBOM (no rebuild)  
6. Post-public smoke: clone + install from cargo path / download binary  

## Known limitations (public README-aligned)

- Not full Phase 10 multi-arch RC evidence (Linux/Windows binaries not in this release)  
- Grok stock TUI / OpenCode bubble / OMP body-inline incomplete  
- Transitive `ttf-parser` unmaintained (RUSTSEC-2026-0192) via RaTeX; denied ignore documented  

## Blockers closed for this cut

- [x] Candidate docs + CHANGELOG/SECURITY/SUPPORT  
- [x] Dual macOS release archives + SHA-256  
- [x] SBOM (CycloneDX JSON for CLI package)  
- [x] cargo-deny config (licenses/sources; advisory ignore documented)  
- [x] Maintainer approve public 0.1.0  

## Follow-ups

- Linux binary via CI; Windows if claimed  
- crates.io when ready  
- Signatures/attestations (cosign / GH attestations)  
- Phase 13 post-v1 protocols  
