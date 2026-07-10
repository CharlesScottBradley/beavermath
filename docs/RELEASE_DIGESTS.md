# Release digests - v0.1.1

## Source freeze

| Artifact | Value |
|----------|--------|
| Git tag | `v0.1.1` |
| Git commit (tag target) | `_PENDING_COMMIT_` |
| `Cargo.lock` SHA-256 | `491077fd3424d63c658c0a728df520dcba9c895c96c2ecb014a603868467efe8` |
| Workspace version | `0.1.0` (package) / release series 0.1.1 |
| Toolchain | `1.91.1` |

## Prebuilt CLI binaries (path-remapped + stripped)

| Artifact | SHA-256 |
|----------|---------|
| `beavermath-0.1.1-aarch64-apple-darwin.tar.gz` | `4bcce8c325d7713a06bbbb0ef908b146fdbc561b8447893f34e26c1598987f92` |
| `beavermath-0.1.1-x86_64-apple-darwin.tar.gz` | `1a4492b1bc2e2c4af9ac0d5b6333c2ff491c44bc9291f0244e2be9863b24cadd` |
| Unpacked binary aarch64-apple-darwin | `4d804c4e8a577240c12856dbbc8d07cc56e0abe5f23dbf77711befeeb60142ce` |
| Unpacked binary x86_64-apple-darwin | `5044c016a71ae5cce6bf8a97073b637304ffc504430982ddcad601097d7c185b` |

Built with `--remap-path-prefix` (repo to `/src/beavermath`, cargo home to `/cargo`) and strip. `strings` scan: no `/Users/` and no lab Desktop roots.

Linux official binary: not in this cut (cross-link failed on macOS host without Linux linker). Source install on Linux remains supported.

## SBOM

| Artifact | SHA-256 |
|----------|---------|
| `docs/release/sbom-beavermath-cli.cdx.json` | `c8de49cdb45569ee0af21c884a529f670ce348e9dab63e81ed0e09d5bbec9a65` |

Host absolute paths scrubbed from package refs.

## Path hygiene

- `main` history: zero lab absolute paths
- Release assets verified by local `strings` / `rg` before tag
- Local backup refs only if present; do not push dirty backups

## crates.io

`publish = false`
