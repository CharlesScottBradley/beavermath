# Security policy

## Supported versions

| Version | Support |
|---------|---------|
| `0.1.x` (private candidate) | Active development; report issues privately |
| Public `1.x` | Opens when Phase 12 publication completes |

## Reporting a vulnerability

**Do not open a public GitHub issue for security problems while the repository is private or before a public security contact is designated.**

Preferred:

1. Contact the repository owner (`CharlesScottBradley` on GitHub) via a **private** channel (GitHub Security Advisories once enabled, or direct private message agreed with the maintainer).
2. Include: affected commit SHA, component (`core` / `cli` / adapter), reproduction, and impact assessment.
3. Allow a reasonable window for a fix before any public disclosure.

## Scope of security design (v0.1 candidate)

- Untrusted TeX closed-command rejection for known dangerous primitives
- No hidden network from core present/render paths
- Typed terminal graphics frames only; no raw model strings as protocol control
- Redirected/machine stdout free of terminal graphics protocol bytes
- Exact source digests for integrity of input association

## Out of scope (for this document)

- Compromised host or malicious `beavermath` binary on PATH
- Host harness vulnerabilities (OMP, OpenCode, Grok, Claude, Codex)
- Upstream RaTeX bugs beyond our adapter confinement envelope
