# BeaverMath

Terminal math for agent harnesses. Strict LaTeX in, readable text out, source unchanged.

## Status

**Private 0.1.1 candidate.** Repository visibility is still private until the public flip. Crates are not on crates.io (`publish = false`). No Sixel, active probing, or SSH/tmux graphics as supported features.

## What it does

Models write `$...$` / `$$...$$` / `\(...\)` / `\[...\]`. BeaverMath turns completed text into:

- **semantic** linear text (default)
- optional **unicode** / **ascii**
- optional **PNG/SVG** when you ask for artifacts
- optional **Kitty / iTerm2** pixels only with trusted capability (no terminal probing)

Exact source bytes and SHA-256 digests stay authoritative. Bare ASCII is not auto-math. Repair is opt-in.

## Install

```bash
git clone https://github.com/CharlesScottBradley/beavermath.git
cd beavermath
cargo install --path crates/beavermath-cli --locked
beavermath --version
```

Requires Rust matching `rust-toolchain.toml` (1.91.1).

macOS binaries (when a release is published): download from GitHub Releases and verify SHA-256 in `docs/RELEASE_DIGESTS.md`.

## Quick use

```bash
beavermath filter 'The answer is $x^2+y^2=z^2$.'
beavermath present 'From Maxwell, $E=mc^2$.'
beavermath doctor
```

Machine JSON (`present` / `hook`) contains no terminal escape sequences.

## Works today (0.1.x)

| Surface | Status |
|---------|--------|
| CLI `filter` / `present` / `render` / `hook` / `doctor` | live |
| URL + inline-code protection | live |
| OMP extension | partial (preview/widget; not assistant-body rewrite) |
| OpenCode plugin | partial / oneshot |
| Grok Build | session VIEWPORT / sidecar; stock TUI paint blocked |
| Claude Code / Codex | oneshot-oriented |

## Not claimed in 0.1.x

- Sixel, active terminal probing, SSH/tmux graphics
- Full in-chat body paint on every harness
- crates.io packages
- Cross-emulator pixel byte identity without pinned CI goldens
- Repair as authority over source

## Safety defaults

- Offline core (no telemetry, no hidden network)
- Hostile TeX commands rejected (`\input`, `\write18`, ...)
- Machine stdout stays free of graphics protocol bytes
- Graphics require trusted capability and a direct TTY

## Docs

- [CHANGELOG](CHANGELOG.md)
- [SECURITY](SECURITY.md)
- [SUPPORT](SUPPORT.md)
- [THIRD_PARTY](THIRD_PARTY.md)
- [Release digests](docs/RELEASE_DIGESTS.md)
- [Phase gates](docs/PHASE_GATES.md)
- [Best public path](docs/BEST_PUBLIC_PATH.md)
- [Go-public plan](docs/GO_PUBLIC_PLAN.md)

## License

`MIT OR Apache-2.0`: [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
