# Changelog

All notable changes to BeaverMath are documented here.  
Format inspired by [Keep a Changelog](https://keepachangelog.com/).  
Versioning follows [SemVer](https://semver.org/) once public crates are enabled.

## [Unreleased]

### Later (not 0.1.1 claims)


- crates.io, Linux/Windows archives as needed, signatures/attestations
- Phase 13: probe / Sixel / SSH-tmux graphics / repair locales ([docs/PHASE13_SCOPE.md](docs/PHASE13_SCOPE.md))
- Harness product bar: Claude IP3, Codex IP4, Grok P1, OMP O3 (Track H)

## [0.1.1] - 2026-07-10

**Status:** public GitHub source + macOS release binaries; crates.io not published.
**Tag:** `v0.1.1`

### Added

- Simple install-first README and honest status tables
- Path-clean history for public cut
- CLI JSON/newline polish; production panic cleanup
- Dependabot, CONTRIBUTING, issue templates
- Remapped macOS aarch64/x86_64 binaries + path-scrubbed SBOM + digests

### Known limitations

Same harness partials as 0.1.0; no crates.io; no Sixel/probe/SSH-tmux graphics claims.

## [0.1.0] - 2026-07-10 (private historical cut)

**Status:** private candidate tag. Not the public freeze (main moved after tag).  
**Tag:** `v0.1.0` (lags current main; use `v0.1.1` for public)

### Added

- Rust workspace: `beavermath-core`, `beavermath-render`, `beavermath-terminal`, `beavermath-adapter`, `beavermath-cli`
- Strict explicit-LaTeX discovery, lossless partition, SHA-256 digests, semantic text
- RaTeX-backed PNG/SVG behind internal adapter; hostile-command rejection
- CLI: `render`, `filter`, `doctor`, `cache`, `repair`, `hook`, `present`, `grok-session`
- Terminal policy: semantic-first; Kitty/iTerm2 typed frames only with trusted capability (zero queries)
- Harness scaffolds: OMP extension (O2 partial), OpenCode plugin (C1), Grok session VIEWPORT, oneshot wrappers
- Schemas, corpus fixtures, CI workflow, dual MIT/Apache licenses
- Path-clean history rewrite; path-scrubbed SBOM; remapped macOS binaries (private release assets)

### Known limitations (not v1.0.0 claims)

- Stock Grok Build TUI does not paint math in-scrollback (closed binary)
- OMP: no true assistant-body content renderer (custom preview only)
- OpenCode: no safe in-bubble rewrite without source poison risk
- Claude Code / Codex: oneshot-oriented; not full interactive in-chat
- crates.io not published (`publish = false`)
- Linux/Windows official binaries not required for source install
- Live hardware matrix for every emulator/a11y tool: not claimed
- Transitive `ttf-parser` unmaintained advisory ignored via RaTeX (`RUSTSEC-2026-0192`, documented in `deny.toml`)

### Security

- Core offline: no telemetry or hidden network from present/hook paths
- Machine channels refuse ESC/CSI/OSC graphics protocol
- Persistent cache opt-in; no raw source in cache keys by design
