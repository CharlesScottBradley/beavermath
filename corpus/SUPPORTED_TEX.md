# Supported TeX (v1 freeze)

**Renderer:** RaTeX 0.1.13 (`ratex-parser` / `ratex-layout` / `ratex-svg` / `ratex-render`)  
**Config id:** `ratex-0.1.13|font=embed-katex|svg=standalone`  
**Fonts:** `ratex-katex-fonts` 0.1.13 (embedded; MIT)

## Closed rejection set

These fail closed before layout when present in the expression:

- `\input`, `\include`, `\write18`, `\openin`, `\openout`
- `\usepackage`, `\href`, `\url`, `\includegraphics`
- `\catcode`, `\def`, `\gdef`, `\edef`, `\let`, `\special`

## Supported (initial)

Common KaTeX-compatible math: atoms, superscripts/subscripts, `\frac`, `\sqrt`,
sums/integrals/products, matrices/cases as accepted by RaTeX parse, Greek letters,
standard relations and operators.

## Limits (FR-046)

| Limit | Value |
|-------|------:|
| Input | 1 MiB |
| Expression | 64 KiB |
| Expressions / message | 128 |
| Nesting depth | 64 |
| Axis px | 4096 |
| Pixels | 4_194_304 |
| Encoded artifact | 8 MiB |
| Formula wall time | 500 ms (best-effort; worker enforcement evolving) |

## Decision gate (Phase 2)

**Continue with RaTeX:** yes for v1 private RC. Closed command rejection is
BeaverMath-owned; assets are embed-fonts; SVG/PNG paths are pure Rust with no
network. If future confinement needs OS-level sandboxing, the adapter boundary
allows replacement without public contract changes.
