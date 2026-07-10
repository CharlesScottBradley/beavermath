# Release digests - v0.1.1

## Source freeze

Verify with `git rev-list -n1 v0.1.1` (must equal tag target).


| Artifact | Value |
|----------|--------|
| Git tag | `v0.1.1` |
| Git commit (tag target) | set by tag `v0.1.1`; run `git rev-list -n1 v0.1.1` |
| `Cargo.lock` SHA-256 | `918e6a8d46f29e79630ec99be4f183c29dffac1b25a54f2b7233d303d9ef268f` |
| Workspace version | `0.1.1` |
| Toolchain | `1.91.1` |

## Prebuilt CLI binaries (path-remapped + stripped)

| Artifact | SHA-256 |
|----------|---------|
| `beavermath-0.1.1-aarch64-apple-darwin.tar.gz` | `a123193b15427b8c44c7d15962a2c77b099b1546d9289869133723dc91e3e18f` |
| `beavermath-0.1.1-x86_64-apple-darwin.tar.gz` | `01e94f1830ef0dd62baba89d75246cb4252a7d2eb63f8185f18d1ecfff95dbe9` |
| Unpacked binary aarch64-apple-darwin | `6ff3d020e3a2aa60cea92918015d34e8505540e08c3c8c5c579a39f887647d63` |
| Unpacked binary x86_64-apple-darwin | `e167f083f720b125a155023a7377961fbc1659ae800106a4b0b9b7594d6a5d24` |

Built with path remapping and strip. strings scan clean of host home paths.

Linux official binary: not in this cut. Source install on Linux supported.

## SBOM

| Artifact | SHA-256 |
|----------|---------|
| `docs/release/sbom-beavermath-cli.cdx.json` | `c8de49cdb45569ee0af21c884a529f670ce348e9dab63e81ed0e09d5bbec9a65` |

## crates.io

`publish = false`
