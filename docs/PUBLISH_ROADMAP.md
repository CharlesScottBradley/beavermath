# BeaverMath publish roadmap

**Purpose:** ordered plan to take the private candidate to a defensible public release.  
**Status:** living plan (2026-07-10). Does **not** flip visibility or publish crates by itself.  
**Canonical freeze / flip checklist:** [PHASE12_PUBLICATION.md](PHASE12_PUBLICATION.md)  
**Digest ledger:** [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md)  
**Phase ledger:** [PHASE_GATES.md](PHASE_GATES.md)  
**Contract sources:** [ROADMAP.md](../ROADMAP.md) Phase 10-12, [PRD](../tasks/prd-beavermath.md) BM-024 / FR-098-100 / NFR-010-011 / NFR-019

---

## 0. How to read this plan

### Three different “releases”

Do not collapse these into one button. Each has its own gates.

| Release | What the world gets | Hard gates |
|---------|---------------------|------------|
| **R1 - GitHub source public** | Readable repo, issues, docs, install-from-source | Full-history path+secret scan clean (or clean-tree import); tip CI green; honest claims |
| **R2 - Binary / archive release** | Tagged GitHub Release + multi-arch tarballs + checksums + SBOM + attestations | Reproducible builds, clean-machine smoke, FR-098/099 |
| **R3 - crates.io (optional first wave)** | `publish = true` packages | Ownership, API freeze, no accidental private deps, intentional order |

Harness adapters (OMP / OpenCode / Grok / Claude / Codex) are **not** a fourth registry release. They ship as **in-tree recipes** under R1/R2 with honest partial status. Full in-chat body paint remains host-API blocked where already documented.

### Release rule (non-negotiable)

From ROADMAP / PRD:

> Public publication promotes the **exact approved private candidate** without rebuilding after approval.

Implication:

1. Freeze digests → approve → build archives **once** → attach those bytes → open visibility / tag.  
2. Any fix after approval = **new candidate**, new freeze, new digests.  
3. Never “fix forward on main after tag” and re-upload different binaries under the same version.

### Version naming

| Tag | When allowed |
|-----|----------------|
| **`v0.1.0`** | **Recommended first public.** Multi-arch/SBOM/a11y matrix incomplete; in-chat partial. Honest beta/RC. |
| **`v1.0.0`** | Only after Phase 10 multi-arch + SBOM + signatures + compatibility matrix rows that ROADMAP marks required for public v1. |

ROADMAP’s “public v1.0.0” is product intent. This plan’s first cut is **`0.1.0` public source + optional archives**, unless you explicitly accept 1.0 with a long known-limitations list.

---

## 1. Current state (ground truth)

Recorded from workspace audit + Phase 12 freeze docs (re-check before acting).

| Fact | Value | Why it matters |
|------|--------|----------------|
| Repo | `CharlesScottBradley/beavermath` | Private until explicit flip |
| Visibility | **PRIVATE** | R1 not done |
| Workspace version | `0.1.0`, all crates `publish = false` | R3 blocked by design |
| Toolchain | `rust-toolchain.toml` → `1.91.1` | Must match CI and release builds |
| Freeze doc candidate | `5d86818…` ([PHASE12_PUBLICATION.md](PHASE12_PUBLICATION.md)) | Digest ledger points here |
| Freeze-doc commit itself | later commit on `main` that *records* that freeze | **HEAD may be ahead of freeze SHA** - re-freeze before flip (§2.0) |
| Tests | `cargo test --locked --workspace` green on freeze host | Necessary, not sufficient |
| Clippy / fmt | **red** under CI’s `-D warnings` / `fmt --check` | R1 CI credibility blocker |
| Absolute path leaks | still present in adapters install/hooks/fixtures | R1 privacy/brand blocker |
| Multi-arch archives | **not produced** | R2 open |
| SBOM / deny / sign | **not automated** | R2 open |
| In-chat bar | OMP O2 partial; others oneshot / blocked | Claim discipline, not ship-stop for 0.1.0 |

**Assumption:** another agent may land polish while this plan is executed. Always re-run §2.0 freeze verification; never assume digests in this file stay current without recompute.

---

## 2. Stage A - Make the tree publicly defensible (before any push of a “release branch” content)

**Goal:** a stranger can open the tree and not see lab machine paths, private secrets, or “wonky private WIP that pretends to be finished.”  
**Does not:** open the repo or publish crates.

### A0. Re-freeze if HEAD ≠ freeze SHA

**Reasoning:** `PHASE12_PUBLICATION.md` freezes `5d86818`. If `git rev-parse HEAD` differs, the freeze is historical, not current tip. Shipping tip without re-freeze violates “promote exact candidate.”

**Steps:**

1. `git rev-parse HEAD` and compare to freeze table.  
2. If mismatch: after A1-A4 land, recompute:
   - commit SHA  
   - `shasum -a 256 Cargo.lock`  
   - tracked tree content hash (same method used for current freeze)  
3. Update [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md) and §1 of [PHASE12_PUBLICATION.md](PHASE12_PUBLICATION.md).  
4. Single freeze commit; no drive-by features.

### A1. Path and identity sanitization (P0)

**Why:** absolute `<REPO_ROOT>` paths fail public review and leak machine layout. Plug-and-play means repo-relative or env-based install only.

**Replace everywhere (tracked files):**

| Pattern | Replace with |
|---------|----------------|
| `<REPO_ROOT>/...` | `$(pwd)/...` or `$BEAVERMATH_HOME/...` or `cargo install --path crates/beavermath-cli` |
| `file://<HOME>/...` | Install-time absolute URI only: generate `file://$PWD/adapters/opencode/plugin` (or host-native relative path if documented). Never commit a lab absolute `file://<HOME>/...`. Do not use invalid `file://./...`. |
| fixture `"cwd": "<HOME>/..."` | synthetic `/tmp/project` or `"/workspace/demo"` |
| test `encode_cwd("<HOME>")` | `encode_cwd("/home/user/project")` (behavior-only) |

**Files known dirty (re-scan after fix):**

- `adapters/grok-build/README.md`  
- `adapters/grok-build/hooks/beavermath-stop.json`  
- `adapters/omp/INSTALL.md`, `INCHAT.md`  
- `adapters/opencode/plugin.example.json`, `INCHAT.md`  
- `adapters/grok-build/tests/fixtures/host-stop-sample.json`  
- `crates/beavermath-adapter/src/grok_session.rs` (test only)

**Gate:**

```bash
rg -n '<HOME>|/Users/[^/]+/Desktop/beavermath' --glob '!.git/**' .
# must be empty (or only intentional synthetic fixtures with a code comment)
```

**History rule (hard blocker for R1):** fixing the tip does **not** erase history. Requirement: **no lab file paths released**. Absolute paths in *any* commit that becomes public are a **publication blocker**, not cosmetic.

Before visibility changes, do **both**:

1. **Full-history path scan** (not tip-only):
   ```bash
   git grep -n '<HOME>\|/Users/.*/Desktop/beavermath\|file://<HOME>/' $(git rev-list --all) || true
   # must return no hits on the history that will become public
   ```
2. **Remediation (required if scan is dirty):**
   - **Default when dirty:** **S3 clean public tree** - squash or orphan a sanitized tree on the branch that will be made public (or import into a new public repo), while still private; re-freeze digests on that tree.
   - **Only if scan is already clean:** keep linear history.
   - **Do not** open the repo with known `<HOME>/...` commits still on `main` “to fix later.”

Never rewrite history that is already public. While still private, rewrite/squash is allowed with maintainer consent and no external clones depending on SHAs.

### A2. Code quality “vocab firewall” (P0 for reviewer trust)

**Why:** public reviewers open CI and `clippy` first. Green tests + red lint reads as unfinished lab code.

**Steps:**

1. `cargo fmt --all`  
2. Fix `cargo clippy --locked --workspace -- -D warnings` (dead `replace('^','^')`, `&& false`, collapsible replaces, etc.)  
3. Production panic polish (small, visible):
   - CLI JSON `unwrap()` on `to_string_pretty` → map to `ExitStatus` / stderr  
   - avoid runtime `assert_eq!` on user-facing repair path  
   - `events.rs` `expect("serialize")` → `Result` or infallible typed write  
   - scanner `last().unwrap()` after push: document invariant or use `expect` with message  
4. Re-run `cargo test --locked --workspace`

**Gate:** local commands match `.github/workflows/ci.yml` (fmt, build, test, clippy `-D warnings`, license/`publish = false` grep).

### A3. Doc honesty matrix (P0)

**Why:** contradictory status destroys trust faster than missing features.

**Single source of truth table** (update once, link everywhere):

| Host | In-chat | What ships in 0.1.0 | Do not claim |
|------|---------|---------------------|--------------|
| CLI `present` / `filter` | N/A | **live** | - |
| OMP | partial | O2 custom preview / widget | full assistant-body rewrite (O3) |
| OpenCode | partial / blocked | plugin side present; oneshot | in-bubble source rewrite |
| Grok Build | blocked paint | `grok-session` VIEWPORT / sidecar | stock TUI scrollback paint |
| Claude / Codex | oneshot | filter/hook recipes | full interactive in-chat |

**Files to reconcile:** `adapters/README.md`, each `INCHAT.md`, README status, CHANGELOG known limitations (already good direction).

**Claim discipline (from PHASE12 §3):** no Sixel, no probe, no SSH/tmux pixels, no repair-as-authority, no cross-emulator pixel identity without pinned goldens.

### A4. Brand / portable install story

**Why:** first public minute is “how do I install?”

**Minimum install path for R1 (source-only is OK for 0.1.0):**

```bash
git clone https://github.com/CharlesScottBradley/beavermath.git
cd beavermath
cargo install --path crates/beavermath-cli --locked
beavermath doctor
beavermath filter 'The answer is $x^2+y^2=z^2$.'
```

**Adapter install:** document relative paths only; require `beavermath` on PATH; never hardcode lab absolute paths in committed hook JSON (generate hook JSON via a small `install-hooks` script or template with `@BEAVERMATH_ROOT@`).

### A5. Privacy / secret scan (NFR-019)

**Why:** PRD forbids private model content, secrets, cache, or unreleased private artifacts in public payloads.

**Steps:**

1. Working tree: secrets patterns, `.env`, keys, tokens, absolute lab paths.
2. **Full-history path scan** (R1 blocker): every commit that will be public must have zero `<HOME>`, lab Desktop roots, and `file://<HOME>/` install paths.
3. Full-history secret scan: API keys, private keys, tokens, `.env` bodies, model session dumps.
4. Confirm no fusion/lab research content, no private session transcripts.
5. Confirm licenses dual MIT/Apache present and third-party notes honest ([THIRD_PARTY.md](../THIRD_PARTY.md)).

**Gate:** written “privacy + path history scan pass” note (date, commands, empty result) in PHASE12 or RELEASE_DIGESTS. Dirty path history ⇒ **no public flip** until S3 clean-tree or successful private rewrite.

### A6. Stage A exit criteria

- [ ] No absolute lab paths in **tip** (A1 working tree)
- [ ] No absolute lab paths in **public-bound history** (A1 history rule / full-history scan)
- [ ] CI-equivalent green locally (A2)
- [ ] One honest adapter matrix (A3)
- [ ] Portable install docs (A4)
- [ ] Privacy + path history scan recorded pass (A5)
- [ ] Freeze digests match **intended public tip** (A0)
- [ ] Maintainer still says: **do not open repo yet** until Stage B decision / explicit approve

---

## 3. Stage B - Private release candidate engineering (R2 prep; still private)

**Goal:** evidence required by Phase 10/12 that single-host `cargo test` does not provide.  
**Still private.** Prefer completing B before R1 if you want the first public day to include downloadable binaries. **Allowed alternative:** R1 source-only first, then B, then R2 tag (see §5 sequencing options).

### B1. Supply chain automation

| Work | Reasoning |
|------|-----------|
| `cargo deny` (or equivalent) in CI | NFR-010: no High/Critical advisories, license policy |
| SBOM generation (`cargo cyclonedx` or similar) | FR-099 |
| Pin all release actions by SHA | no floating third-party release actions |
| Font / RaTeX license inventory | already started in THIRD_PARTY; complete and CI-check |

### B2. Multi-target release build

Targets from ROADMAP Phase 10 (drop any that fail confinement/smoke):

- `aarch64-apple-darwin`  
- `x86_64-apple-darwin`  
- `x86_64-unknown-linux-gnu`  
- `aarch64-unknown-linux-gnu`  
- `x86_64-pc-windows-msvc` (only if claimed)

**Steps:**

1. Add private `release.yml` (or `dist` job) that builds `--release --locked` per target.  
2. Pack `beavermath` binary (+ licenses + checksum file).  
3. Record SHA-256 per archive in [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md).  
4. Same-target rebuild twice → matching digests (NFR-011).  

**Gate:** every advertised target has archive + checksum + smoke log.

### B3. Signatures / attestations

- GitHub artifact attestations and/or cosign.  
- Document verification commands in SUPPORT.md.  

### B4. Installer safety (NFR-020)

- User-local, non-`sudo`, version-bound, checksum-verifying.  
- No shell profile edits without consent.  
- Optional for 0.1.0 if install-from-source is the only advertised path; **required** if you ship `curl | sh` style installers.

### B5. Compatibility matrix honesty

For **0.1.0 public**, minimum evidence:

| Cell | Minimum for 0.1.0 | Full v1.0.0 |
|------|-------------------|-------------|
| macOS aarch64 CLI filter/present | required | required |
| Linux CI test job | required (already in ci.yml) | required |
| Kitty/iTerm2 pixels | document “requires trusted capability”; manual note OK | fixture + host attestation tests |
| Apple Terminal / a11y | “text fallback” claim only | VoiceOver/NVDA/Orca task set (NFR-008) |
| Machine channel zero ESC | automated tests already exist | keep |

Do not block 0.1.0 on full NFR-008 matrix if CHANGELOG lists it as known limitation (already does).

### B6. Stage B exit criteria

- [ ] deny/SBOM in CI  
- [ ] multi-arch digests recorded (or explicit “source-only 0.1.0, no binary assets”)  
- [ ] signatures plan executed or explicitly deferred in CHANGELOG  
- [ ] no overclaim vs matrix  

---

## 4. Stage C - Controlled publication (the only day visibility flips)

Execute only after maintainer **explicit approve public**. Follow PHASE12 §5; expanded here with reasoning.

### C1. Pre-flight (same day)

1. Confirm freeze SHA == intended tag tip (`git rev-parse`).  
2. Confirm digests file matches archives you will attach (if any).  
3. Confirm `publish = false` still true **unless** R3 is in scope for the same day.  
4. Confirm SECURITY.md contact path works (private report path).  
5. Final `rg` path/secret scan on tip.

### C2. Tag

```bash
git tag -a v0.1.0 -m "BeaverMath 0.1.0 first public candidate"
# do not push tag until archives built and recorded if R2 same day
```

### C3. Build archives once (if R2)

- Build from the tagged commit only.  
- Write digests → commit digest update **before** tag if digests must live in-repo, **or** attach digests only on GitHub Release notes (prefer in-repo RELEASE_DIGESTS for auditability; that implies tag on the digest-recording commit).  

Practical sequence if digests are in-repo:

1. Build archives from freeze commit F.  
2. Commit digest rows → commit D.  
3. Tag `v0.1.0` on D (or re-freeze that D is the candidate).  
4. Attach archives built from F/D consistently (document which tree produced which binary).  

**Simplest honest approach for first cut:** tag source `v0.1.0` **source-only**; ship binaries as `v0.1.1` after digest pipeline is boring. Avoids “binary not in tagged tree” confusion.

### C4. Open repository (R1)

1. GitHub: visibility **Public**.  
2. Push branch + tags.  
3. Create GitHub Release: notes from CHANGELOG; attach archives if any; paste checksums.  
4. Enable Security Advisories; private vulnerability reporting.  
5. Protect `main` (required reviews optional but recommended).

### C5. Post-public smoke (FR-098)

From a **clean machine / clean VM** (not the lab checkout):

```bash
git clone https://github.com/CharlesScottBradley/beavermath.git
cd beavermath
cargo test --locked --workspace
cargo install --path crates/beavermath-cli --locked
beavermath filter 'The answer is $x^2+y^2=z^2$.'
beavermath present 'From Maxwell, $E=mc^2$.' | tee /tmp/p.json
# assert exact_source preserved; display_text non-empty; no ESC in machine JSON
```

Record pass/fail in PHASE_GATES or a `docs/PUBLIC_SMOKE.md` log.

### C6. crates.io (R3) - separate decision

Only if intentionally in scope:

1. Flip `publish = true` on intended crates only (likely start with none, or `beavermath-core` later).  
2. Order: core → render/terminal/adapter → cli (binary crates often stay GitHub-only).  
3. `cargo publish --dry-run` each.  
4. Publish once per crate; no republish-overwrite fantasy.  
5. OIDC trusted publishing after first token bootstrap if required.

**Recommendation for first public day:** **skip R3**. Source + CLI install from git is enough for 0.1.0. Crates.io can wait for API freeze confidence.

---

## 5. Sequencing options (pick one explicitly)

### Option S1 - Source-first public (fastest defensible)

```text
A (sanitize tip + **full-history path clean / S3 if dirty** + CI green + freeze) → C4 R1 public source → B binaries later → R2 tag v0.1.1
```

**Pros:** unblocks community read/review; no binary supply-chain rush.  
**Cons:** first day is clone/build only.

### Option S2 - Full candidate then flip (ROADMAP-faithful)

```text
A → B (archives+SBOM+sign) → C tag+attach+public same day
```

**Pros:** matches Phase 12 “promote exact candidate.”  
**Cons:** longer private wait; more moving parts on flip day.

### Option S3 - Clean-tree public import (**required if history path scan is dirty**)

```text
A on sanitized tip → orphan or squash clean tree → replace private main (still private)
  OR push clean tree to a new public repo and retire path-leaking history from public view
```

**Pros:** no historical path leaks; satisfies “no file paths released.”  
**Cons:** loses granular commit history; needs maintainer consent; re-freeze digests after rewrite.

**Default recommendation:**

1. Run full-history path scan.  
2. If **dirty** (current expected state): **S3 first**, then S1 or S2 flip.  
3. If **clean**: **S1** for source-first, or **S2** for ceremonial binaries.

---

## 6. What must never ship in 0.1.0 claims

Copied and operationalized from PHASE12 / ROADMAP blockers:

1. Sixel / active probe / SSH-tmux graphics as supported.  
2. Apple Terminal images.  
3. “Full in-chat math” for Grok / OpenCode / Claude / Codex.  
4. OMP O3 true body rewrite.  
5. Repair overwriting source.  
6. Cross-emulator PNG byte identity without pinned CI renderer statement.  
7. Telemetry / update check / network from core (must remain true in code).  
8. Absolute lab filesystem paths in docs or committed configs.

---

## 7. Work breakdown (execution checklist)

### Wave 1 - Public-ready tip (R1 prerequisites)

| # | Task | Owner cue | Done when |
|---|------|-----------|-----------|
| 1.1 | Path sanitization A1 (tip) | code/docs | working-tree `rg` clean |
| 1.1b | Full-history path scan + S3 if dirty | maintainer | `git grep` over all public-bound commits empty |
| 1.2 | fmt + clippy A2 | rustc | CI job green |
| 1.3 | Production panic polish | rustc | no user-path unwrap/assert |
| 1.4 | Adapter status single matrix A3 | docs | no contradictions |
| 1.5 | Portable INSTALL/README A4 | docs | clone path works |
| 1.6 | Privacy + path history scan A5 | maintainer | written pass note (empty scans) |
| 1.7 | Re-freeze digests A0 | maintainer | HEAD == freeze SHA |

### Wave 2 - RC evidence (R2 prerequisites)

| # | Task | Done when |
|---|------|-----------|
| 2.1 | cargo deny + SBOM CI | job green on main |
| 2.2 | multi-arch release workflow | digests rows filled |
| 2.3 | attestations | verify command works |
| 2.4 | clean VM smoke script | script + log committed or attached |

### Wave 3 - Flip day

| # | Task | Done when |
|---|------|-----------|
| 3.1 | Explicit written “approve public” | message/commit |
| 3.2 | Tag + push | tag on GitHub |
| 3.3 | Visibility public | repo public |
| 3.4 | GitHub Release | notes + assets |
| 3.5 | Post-public smoke C5 | pass recorded |
| 3.6 | (optional) crates.io | only if pre-approved |

### Wave 4 - After public (not blockers for 0.1.0)

- OMP O3 upstream hook proposal  
- Deepen semantic layer or reword PRD tone to match heuristics  
- Full a11y matrix for 1.0.0  
- crates.io if deferred  
- Phase 13 protocols still deferred  

---

## 8. Decision log (fill as you go)

| Date | Decision | Choice | Reason |
|------|----------|--------|--------|
| 2026-07-10 | First public version | **0.1.0 recommended** | matrix/SBOM incomplete; honest beta |
| 2026-07-10 | Sequence | **TBD: S1 or S2** | maintainer pick |
| 2026-07-10 | crates.io day-one | **NO recommended** | publish=false; API still young |
| 2026-07-10 | Public flip today | **NO-GO** | §4 PHASE12 blockers + path/clippy |

---

## 9. Related documents

| Doc | Role |
|-----|------|
| [PHASE12_PUBLICATION.md](PHASE12_PUBLICATION.md) | Freeze table, blockers, flip sequence |
| [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md) | Commit / lock / archive hashes |
| [PHASE_GATES.md](PHASE_GATES.md) | Live vs blocked harness honesty |
| [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md) | Product bar definition |
| [CHANGELOG.md](../CHANGELOG.md) | Human notes + known limitations |
| [SECURITY.md](../SECURITY.md) | Reporting |
| [SUPPORT.md](../SUPPORT.md) | Install / matrix |
| [THIRD_PARTY.md](../THIRD_PARTY.md) | Licenses |
| [ROADMAP.md](../ROADMAP.md) | Phase 10-13 engineering intent |
| [PRD](../tasks/prd-beavermath.md) | Normative requirements |

---

## 10. One-page summary

```text
  fix tip paths → full-history path scan (S3 clean tree if dirty) →
  fmt/clippy green → honest docs → privacy scan → re-freeze

THEN (still private, optional before flip):
  SBOM/deny → multi-arch archives → sign → record digests

FLIP (explicit approve only):
  tag v0.1.0 → (attach archives) → public repo → clean-machine smoke

LATER:
  crates.io, full a11y, O3 hooks, v1.0.0 when matrix is real
```

**Go / no-go right now**

| Action | Status |
|--------|--------|
| Continue private polish (Wave 1) | **GO** |
| Public visibility flip | **NO-GO** until Wave 1 + explicit approve |
| crates.io | **NO-GO** |
| Claim “v1.0.0 complete in-chat” | **NO-GO** forever-until-evidence |
