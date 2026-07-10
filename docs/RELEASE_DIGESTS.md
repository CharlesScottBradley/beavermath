# Release digests - v0.1.1

## Source freeze

| Artifact | Value |
|----------|--------|
| Git tag | `v0.1.1` |
| Git commit (tag target) | `dfb90ad9412eeb525fa7f3188a59910157264ed6` |
| Binary build source | same tree as tag (crates 0.1.1 + freeze docs) |
| `Cargo.lock` SHA-256 | `918e6a8d46f29e79630ec99be4f183c29dffac1b25a54f2b7233d303d9ef268f` |
| Workspace version | `0.1.1` |
| Toolchain | `1.91.1` |

## Prebuilt CLI binaries (path-remapped + stripped)

| Artifact | SHA-256 |
|----------|---------|
| `beavermath-0.1.1-aarch64-apple-darwin.tar.gz` | `65efc9133c4a18cabdb4db8e3d648a4b5b60cd8bd78b8858b62721aca51d91a6` |
| `beavermath-0.1.1-x86_64-apple-darwin.tar.gz` | `96598bd03d478cda91eb7413ceaf5af951a4b6f9461444e701a1b7ad8d53992f` |
| Unpacked binary aarch64-apple-darwin | `6ff3d020e3a2aa60cea92918015d34e8505540e08c3c8c5c579a39f887647d63` |
| Unpacked binary x86_64-apple-darwin | `e167f083f720b125a155023a7377961fbc1659ae800106a4b0b9b7594d6a5d24` |

Built with `--remap-path-prefix` (repo to `/src/beavermath`, cargo home to `/cargo`) and strip. `strings` scan: no `/Users/` and no lab Desktop roots.

Linux official binary: not in this cut (host lacks Linux linker). Source install on Linux remains supported.

## SBOM

| Artifact | SHA-256 |
|----------|---------|
| `docs/release/sbom-beavermath-cli.cdx.json` | `c8de49cdb45569ee0af21c884a529f670ce348e9dab63e81ed0e09d5bbec9a65` |

Host absolute paths scrubbed from package refs.

## Path hygiene

- `main` history: zero lab absolute paths on public-bound commits
- Release assets verified by local `strings` before tag
- Do not push dirty backup refs

## crates.io

`publish = false`
