# Release digests — v0.1.0 (path-clean rebuild)

## Source freeze (clean history)

| Artifact | Value |
|----------|--------|
| Git tag | `v0.1.0` (moved to clean-history tip after orphan) |
| Git commit (tag target) | b4a269f15f06d4a93843ba3f9bafcc16d5e44467 |
| `Cargo.lock` SHA-256 | `491077fd3424d63c658c0a728df520dcba9c895c96c2ecb014a603868467efe8` |
| Workspace version | `0.1.0` |
| Toolchain | `1.91.1` (`rust-toolchain.toml`) |

## Prebuilt CLI binaries (macOS; digests unchanged from prior build)

| Artifact | SHA-256 |
|----------|---------|
| `beavermath-0.1.0-aarch64-apple-darwin.tar.gz` | `c3e91374153718b5f13944ebfc04121330d3e7d2e1a5470fa85ed0a520706c29` |
| `beavermath-0.1.0-x86_64-apple-darwin.tar.gz` | `87c2c4986b8039c9bd2523aff09e9472156ab063752a1086310269f188ebfe9d` |
| Unpacked binary aarch64-apple-darwin | `51b6ff99bac2c25cb46f79d4dacd14ac843dca4374423b49a3de309607cf2252` |
| Unpacked binary x86_64-apple-darwin | `c5c5c15a0b7da6a411da0466bc61c95d35b14bb4efae953e307c68f680a3ec63` |

Archive layout: single executable named `beavermath` at archive root.

## SBOM (path-scrubbed)

| Artifact | SHA-256 |
|----------|---------|
| `docs/release/sbom-beavermath-cli.cdx.json` | `c8de49cdb45569ee0af21c884a529f670ce348e9dab63e81ed0e09d5bbec9a65` |

SBOM package refs use generic `pkg:cargo/...` style identifiers; absolute host paths are not embedded.

## Path hygiene

- Tip and all commits on the public-bound branch must contain **zero** lab absolute paths (`/Users/...` install roots).
- Dirty pre-clean history retained only on private ref `backup/pre-clean-history` (not pushed for public consumption).

## Linux / Windows

Not produced in this cut.

## crates.io

Workspace crates remain `publish = false` for this cut.
