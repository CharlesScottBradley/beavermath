# Changelog

All notable changes to BeaverMath are documented here.  
Format inspired by [Keep a Changelog](https://keepachangelog.com/).  
Versioning follows [SemVer](https://semver.org/) once public crates are enabled.

## [0.1.0] — 2026-07-10

**Status:** public GitHub source + macOS release binaries; crates.io not published (`publish = false`).  
**Tag:** `v0.1.0`

### Added

- Rust workspace: `beavermath-core`, `beavermath-render`, `beavermath-terminal`, `beavermath-adapter`, `beavermath-cli`
- Strict explicit-LaTeX discovery, lossless partition, SHA-256 digests, semantic text
- RaTeX-backed PNG/SVG behind internal adapter; hostile-command rejection
- CLI: `render`, `filter`, `doctor`, `cache`, `repair`, `hook`, `present`, `grok-session`
- Terminal policy: semantic-first; Kitty/iTerm2 typed frames only with trusted capability (zero queries)
- Harness scaffolds: OMP extension (O2 partial), OpenCode plugin (C1), Grok session VIEWPORT, oneshot wrappers
- Schemas, corpus fixtures, CI workflow skeleton, dual MIT/Apache licenses

### Known limitations (not v1.0.0 claims)

- Stock Grok Build TUI does not paint math in-scrollback (closed binary)
- OMP: no true assistant-body content renderer (custom preview only)
- OpenCode: no safe in-bubble rewrite without source poison risk
- Claude Code / Codex: oneshot-oriented; not full interactive in-chat
- Multi-arch release archives, SBOM automation, signatures/attestations: not yet produced
- Live hardware matrix for every emulator/a11y tool: not fully claimed

### Security

- Core offline: no telemetry or hidden network from present/hook paths
- Machine channels refuse ESC/CSI/OSC graphics protocol
- Persistent cache opt-in; no raw source in cache keys by design

## [Unreleased]

- Phase 12 residual: crates.io, Linux/Windows archives, signatures (optional; separate from Phase 13)
- Phase 13 scoped: post-v1 probe / Sixel / SSH-tmux graphics / repair locales ([docs/PHASE13_SCOPE.md](docs/PHASE13_SCOPE.md)); no v1 claim change
- Harness product bar (Claude IP3, Codex IP4, Grok P1, OMP O3): Track H, not Phase 13
