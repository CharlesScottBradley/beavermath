# Contributing

## Build and test

```bash
cargo fmt --all -- --check
cargo clippy --locked --workspace -- -D warnings
cargo test --locked --workspace
cargo deny check
```

Toolchain: `rust-toolchain.toml` (1.91.1).

## Claim discipline

Public docs must not overclaim. Before changing README or SUPPORT:

- Partial harness integrations stay labeled partial.
- No Sixel, active probe, or SSH/tmux graphics as supported 0.1.x features.
- Repair never becomes authority over exact source.
- Bare ASCII is not auto-math unless an explicit opt-in path is documented.

## Pull requests

- Keep changes focused.
- Include tests for behavior changes.
- CI must pass.
- Do not commit lab absolute paths (`/Users/...` install roots) or secrets.

## Security

See [SECURITY.md](SECURITY.md). Do not file public issues for vulnerabilities.
