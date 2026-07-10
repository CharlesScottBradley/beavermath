# Release digests — v0.1.0 (path-clean)

## Source freeze

| Artifact | Value |
|----------|--------|
| Git tag | `v0.1.0` |
| Git commit (tag target) | `7b022e4d52be317c4a0afb59bce3d28d7e2255ba` |
| `Cargo.lock` SHA-256 | `491077fd3424d63c658c0a728df520dcba9c895c96c2ecb014a603868467efe8` |
| Workspace version | `0.1.0` |
| Toolchain | `1.91.1` |

## Prebuilt CLI binaries (path-remapped + stripped)

| Artifact | SHA-256 |
|----------|---------|
| `beavermath-0.1.0-aarch64-apple-darwin.tar.gz` | `a6dc221a0d8de5183ebc91c92addcbe9d841bdf8f9abb83ec7a042067efc0e91` |
| `beavermath-0.1.0-x86_64-apple-darwin.tar.gz` | `f1993438aee9ed72c10a8cf5aa54ce1ab2ee51d65d6501300b6965fffc1edb33` |
| Unpacked binary aarch64-apple-darwin | `1091d0f74588d7b10eda14355a6826b28de8ce5f100c748ea614a2b1082e4568` |
| Unpacked binary x86_64-apple-darwin | `1d8111970f382ded08d7cc040383dbac1c7d7643ac34bda5d86b22b8997d1288` |

Built with `--remap-path-prefix` (repo → `/src/beavermath`, cargo home → `/cargo`) and strip. `strings` scan: no `/Users/` and no lab Desktop roots.

## SBOM

| Artifact | SHA-256 |
|----------|---------|
| `docs/release/sbom-beavermath-cli.cdx.json` | `c8de49cdb45569ee0af21c884a529f670ce348e9dab63e81ed0e09d5bbec9a65` |

Host absolute paths scrubbed from package refs.

## Path hygiene

- `origin/main` history: zero lab absolute paths
- Release assets (binaries + SBOM): verified by download + strings/rg
- Local backup ref `backup/pre-clean-history` only (not pushed)

## crates.io

`publish = false`
