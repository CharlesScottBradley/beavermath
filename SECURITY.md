# Security policy

## Supported versions

| Version | Support |
|---------|---------|
| `0.1.x` private candidate | Active development; report privately |
| Public `0.1.x` / later | Opens when GitHub visibility is public and Security Advisories are enabled |

## Reporting a vulnerability

**Do not open a public issue for security problems.**

While the repository is private:

1. Contact the repository owner (`CharlesScottBradley` on GitHub) via a private channel.
2. Include: commit SHA, component (`core` / `cli` / adapter), reproduction, impact.

After the public flip:

1. Prefer GitHub Security Advisories / private vulnerability reporting on the repository.
2. Keep the same reproduction details.

Allow a reasonable window for a fix before any public disclosure.

## Design scope (0.1.x)

- Untrusted TeX closed-command rejection for known dangerous primitives
- No hidden network from core present/render paths
- Typed terminal graphics frames only; no raw model strings as protocol control
- Redirected/machine stdout free of terminal graphics protocol bytes
- Exact source digests for input association integrity
- Documented advisory ignore for transitive `ttf-parser` via RaTeX (`RUSTSEC-2026-0192` in `deny.toml`)

## Out of scope (for this document)

- Compromised host or malicious `beavermath` binary on PATH
- Full guarantee of every third-party harness TUI paint path
