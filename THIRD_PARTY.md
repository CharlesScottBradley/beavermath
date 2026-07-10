# Third-party notices (private 0.1.0 candidate)

BeaverMath is dual-licensed MIT OR Apache-2.0. Dependencies retain their own licenses.

## Primary renderer stack

| Crate | Version (lock) | License (upstream) | Role |
|-------|----------------|--------------------|------|
| ratex-parser | 0.1.13 | MIT | TeX parse |
| ratex-layout | 0.1.13 | MIT | Layout |
| ratex-svg | 0.1.13 | MIT | SVG export |
| ratex-render | 0.1.13 | MIT | PNG export |
| ratex-katex-fonts | 0.1.13 | MIT | Embedded fonts |
| ratex-types / font / lexer / … | 0.1.13 | MIT | RaTeX stack |

See [RaTeX](https://github.com/erweixin/RaTeX).

## Other notable dependencies

| Crate | Role |
|-------|------|
| serde / serde_json | Serialization |
| sha2 / hex | Digests |
| clap | CLI |
| thiserror | Errors |
| base64 | Terminal frame encoding |
| tiny-skia (transitive via ratex-render) | Raster |

Exact versions: committed `Cargo.lock`.  
Before public crates.io: generate machine-readable SBOM (e.g. `cargo cyclonedx`) and re-run license/advisory audit.

## Notices generation

```bash
# When tools are installed:
cargo install cargo-cyclonedx
cargo cyclonedx -f json -o sbom.cdx.json
cargo install cargo-deny
cargo deny check
```
