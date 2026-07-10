# Security policy

## Supported versions

| Version | Support |
|---------|---------|
| Public `0.1.1` / `0.1.x` | Active; use GitHub Security Advisories |

## Reporting a vulnerability

**Do not open a public issue for security problems.**

1. Prefer **GitHub Security Advisories** / private vulnerability reporting on this repository.
2. Include: commit SHA, component (`core` / `cli` / adapter), reproduction, impact.
3. If advisories are unavailable, contact the owner (`CharlesScottBradley`) privately.

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
