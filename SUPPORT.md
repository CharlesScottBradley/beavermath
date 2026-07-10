# Support matrix (private 0.1.0 candidate)

## Platforms (logical artifact / CLI)

| Platform | Build tested | Notes |
|----------|--------------|-------|
| macOS aarch64 | yes (dev machine) | Primary lab host |
| macOS x86_64 | not claimed | |
| Linux aarch64/x86_64 | CI workflow defined; not fully evidenced here | |
| Windows | not claimed for terminal pixels | |

## Terminal presentation

| Destination | v0.1 claim |
|-------------|------------|
| Semantic / filter / present / JSON | Supported |
| Direct Kitty / iTerm2 pixels | Only with trusted capability + direct TTY; zero queries |
| Apple Terminal / unknown | Text only |
| SSH / tmux / screen | Text / structured only |
| Pipe / CI / hook stdout | No graphics protocol |

## Harness integration

| Host | Status |
|------|--------|
| CLI oneshot / filter / present | Supported |
| OMP | Extension O2: custom preview + widget (not body-inline) |
| OpenCode | Plugin C1: presentation files; bubble rewrite blocked |
| Grok Build | Session VIEWPORT + hooks; stock TUI paint blocked |
| Claude Code / Codex | Oneshot-oriented scripts |

## Installation (private)

```bash
git clone git@github.com:CharlesScottBradley/beavermath.git
cd beavermath
cargo install --path crates/beavermath-cli --locked
beavermath present --mode semantic <<< 'The answer is $x^2$.'
```

Requires Rust toolchain matching `rust-toolchain.toml` (1.91.1 channel pin).
