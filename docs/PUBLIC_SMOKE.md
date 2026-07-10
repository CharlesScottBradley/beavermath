# Public smoke - v0.1.1

**Date:** 2026-07-10  
**Repo:** https://github.com/CharlesScottBradley/beavermath (PUBLIC)  
**Tag:** `v0.1.1`

## Clean clone

```bash
git clone --depth 1 --branch v0.1.1 https://github.com/CharlesScottBradley/beavermath.git
cd beavermath
cargo test --locked --workspace
cargo install --path crates/beavermath-cli --locked --force
beavermath --version
beavermath filter 'The answer is $x^2+y^2=z^2$.'
beavermath present 'From Maxwell, $E=mc^2$.'
```

## Results

| Check | Result |
|-------|--------|
| Clone public HTTPS | pass |
| `cargo test --locked --workspace` | pass |
| `beavermath 0.1.1` | pass |
| filter semantic | pass |
| present JSON with exact_source | pass |
| README status Public 0.1.1 | pass |
| Branch protection required CI | configured |

## Notes

- crates.io not published
- Secret scanning API enable returned 422 from this client; confirm in GitHub Settings if needed
