# Support matrix (public 0.1.1)

## Platforms

| Platform | Claim | Notes |
|----------|--------|-------|
| macOS aarch64 | source + binary (when release assets ship) | Primary build host |
| macOS x86_64 | source + binary (when release assets ship) | Cross-built with path remapping |
| Linux x86_64 | source; CI test job | Binary preferred for best cut if strings-clean |
| Linux aarch64 | source | Not a primary release target yet |
| Windows | source only | No pixel claims |

## Terminal presentation

| Destination | 0.1.x claim |
|-------------|-------------|
| Semantic / filter / present / JSON | Supported |
| Direct Kitty / iTerm2 pixels | Only with trusted capability + direct TTY; zero queries |
| Apple Terminal / unknown | Text only |
| SSH / tmux / screen | Text / structured only |
| Pipe / CI / hook stdout | No graphics protocol |

## Harness integration

| Host | Status |
|------|--------|
| CLI oneshot / filter / present | Supported |
| OMP | O2: custom preview + widget (not body-inline); O3 blocked |
| OpenCode | Plugin C1 / oneshot; bubble rewrite blocked |
| Grok Build | Session VIEWPORT + hooks; stock TUI paint blocked |
| Claude Code / Codex | Oneshot-oriented scripts |

## Installation

```bash
git clone https://github.com/CharlesScottBradley/beavermath.git
cd beavermath
cargo install --path crates/beavermath-cli --locked
beavermath present --mode semantic <<< 'The answer is $x^2$.'
```

Requires Rust matching `rust-toolchain.toml` (1.91.1).

Binary install (when published): download the archive for your arch from GitHub Releases, verify SHA-256 against `docs/RELEASE_DIGESTS.md`, extract, run `./beavermath --version`.

## Reporting

- Security: [SECURITY.md](SECURITY.md)
- Bugs and claim disputes: GitHub issues (use claim template for overstated docs)
