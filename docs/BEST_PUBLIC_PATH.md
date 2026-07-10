# BeaverMath: best path to public

**Mode:** comprehensive quality path (not the fastest cut).  
**Status:** plan only (2026-07-10). Repo stays **PRIVATE** until Wave F.  
**Version target:** public **`0.1.1`**.  
**Companion:** shorter checklist in [GO_PUBLIC_PLAN.md](GO_PUBLIC_PLAN.md). This file is the full order of work, evidence, and deferred tracks.

Writing bar: direct, concrete, short. No overclaims. Commas and colons, not em dashes.

---

## 0. What “best” means here

Best is not “most features before open.” Best is:

1. **Defensible history and assets** (no lab paths, freeze integrity).  
2. **Honest product surface** (README a cold engineer trusts).  
3. **CI and dependency policy green on the freeze SHA**.  
4. **Evidence for every public claim**.  
5. **Clean cutover** (tag = commit = binary source).  
6. **Deferred work parked on named tracks** so it cannot pollute 0.1 claims.

crates.io, full harness body paint, Sixel/probe, and WCAG-complete math a11y are **not** required for a best 0.1.1 public cut. They are later tracks with their own gates.

---

## 1. Ground truth (re-check every wave start)

| Fact | Current (2026-07-10) |
|------|----------------------|
| Visibility | PRIVATE |
| HEAD | see `git rev-parse HEAD` |
| CI | green on recent main (fmt/clippy/test/deny) |
| Path hits on `main` history | 0 for lab `/Users/beav` |
| Tag `v0.1.0` | **lags** main; not public freeze |
| crates | `publish = false` |
| Stale docs | CHANGELOG/PHASE_GATES still imply public 0.1.0 in places |

**Rule:** never flip public while tag, digests, binaries, and HEAD disagree.

---

## 2. End state of the best 0.1.1 public cut

### 2.1 Must be true

- GitHub repo **public**
- `main` history path-clean
- Tag **`v0.1.1`** equals freeze commit equals remapped binary build source
- README: simple, install-first, status tables, honest limitations
- CHANGELOG, SUPPORT, SECURITY, PHASE_GATES consistent with reality
- CI green on freeze SHA (ubuntu + macos at minimum)
- `cargo deny` green (documented RaTeX advisory ignore allowed)
- SBOM attached, path-scrubbed, re-scanned
- macOS aarch64 + x86_64 release binaries path-clean by `strings`
- Digests in `docs/RELEASE_DIGESTS.md` match attached files
- Post-public clean-machine smoke recorded
- Security reporting path live
- Asset re-download scan after GitHub Release upload
- Branch protection on `main` with required CI status checks
- Secret scanning + dependency alerts enabled (Dependabot or equivalent)
- `SECURITY.md` + GitHub Security Advisories path ready at flip

### 2.2 Strongly preferred (best path does these unless blocked)

- Linux `x86_64-unknown-linux-gnu` release binary, strings-clean, in digests
- GitHub attestation or cosign on archives (same day if easy; else first post-public tag)
- Second clean-dir smoke before flip (private) plus post-public smoke
- CI job log linked from freeze notes
- Tag protection rules for `v*` (prevent force-move of release tags after public)
- `CONTRIBUTING.md` + issue templates (bug / wrong-math-claim)

### 2.3 Not required (and not the goal of this cut)

- crates.io publish
- Windows official binary without a real Windows evidence pass
- OMP O3, Grok stock TUI paint, OpenCode bubble rewrite
- Phase 13: probe, Sixel, SSH/tmux graphics
- Deep semantic or MathML a11y rewrite

Best public 0.1 means integrity, honesty, and evidence for what you ship. It does **not** mean finishing host-API blockers or Phase 13.

### 2.4 Claims allowed after public

Strict LaTeX discovery; source digests; semantic filter/present; machine-safe JSON; URL/code protection; Kitty/iTerm2 only with trusted capability; partial harness adapters as labeled; offline core.

### 2.5 Claims forbidden

Sixel; active probe; SSH/tmux graphics; full in-chat on all hosts; repair as source authority; cross-emulator pixel identity; complete a11y; wording that says public while the repo is still private during prep.

---

## 3. Wave map (comprehensive order)

```text
A  Doc ledger repair (kill stale “already public” lies)
B  Public surface (README + SUPPORT + CHANGELOG + SECURITY)
C  Code quality bar (panic polish, newline, small invariants)
D  Evidence bar (tests, deny, multi-arch assets, path/asset scans, CI on freeze SHA)
E  Private freeze v0.1.1 (binaries, SBOM, digests, tag, repo governance)
F  Flip (approve only)
G  Post-public hardening
H  Deferred tracks (H harness, 12R residual, 13 protocols)
```

Do not skip A/B for E. Best path is surface + evidence + freeze, then flip.

---

## 4. Wave A: Doc ledger repair

**Goal:** internal status files stop contradicting reality.

| # | Task | Done when |
|---|------|-----------|
| A1 | `docs/PHASE_GATES.md`: Phase 12 = **private candidate / public flip pending**; link GO_PUBLIC + BEST_PUBLIC_PATH | no “done public 0.1.0” while private |
| A2 | `CHANGELOG.md`: 0.1.0 as private historical cut; add Unreleased → 0.1.1 prep notes; remove false “SBOM not produced” if assets exist | cold reader not misled |
| A3 | `docs/PHASE13_SCOPE.md`: depends on **actual public 0.1.x**, not false “public v0.1.0 complete” | dependency honest |
| A4 | `docs/PHASE12_PUBLICATION.md`: already corrected toward private 0.1.1; keep in sync with freeze | matches HEAD plan |
| A5 | Grep docs for “public 0.1.0” / “visibility public” claims | only future-tense or post-flip |

**Exit:** `rg -n 'public 0.1.0|GitHub public 0.1' README.md CHANGELOG.md docs/SUPPORT.md docs/PHASE_GATES.md docs/PHASE12_PUBLICATION.md` shows no false present-tense public claim.

---

## 5. Wave B: Public surface (best README)

**Goal:** a stranger can install and trust status without reading the PRD.

### B1. Replace README with the simple draft

Source text: [GO_PUBLIC_PLAN.md](GO_PUBLIC_PLAN.md) §3.1, with status still:

> private **0.1.1 candidate** (not public until flip)

Structure:

1. One-line what it is  
2. Install  
3. Quick use  
4. Works today table  
5. Not claimed table  
6. Safety defaults  
7. Short docs list  
8. License  

Drop from front page: mermaid architecture, long PRD-style paragraphs, internal freeze archaeology.

### B2. SUPPORT.md

| Platform | Claim for 0.1.1 |
|----------|-----------------|
| macOS aarch64 | source + binary (if assets ship) |
| macOS x86_64 | binary if built and strings-clean; else “source” |
| Linux | source; CI test job; binary optional stretch |
| Windows | source only unless evidenced |

Harness table: same honesty as adapters README (OMP O2 partial, etc.).

Install: HTTPS clone example for public future; keep SSH as alternative.

### B3. CHANGELOG 0.1.1 section (written at freeze, drafted in B)

Include:

- CI green (fmt/clippy)  
- path-clean history rewrite note (no path detail that re-leaks)  
- honesty doc fixes  
- known limitations (harness partials, no crates.io, RaTeX advisory ignore)  

### B4. SECURITY.md

- While private: owner private report path  
- At flip: enable GitHub Security Advisories; update SECURITY.md same day  

### B5. adapters/README.md

Single status table only (O2 partial already). No second conflicting story in SPIKE titles required for install.

**Exit:** someone who only reads README + SUPPORT can install from source and state correctly what is partial.

---

## 6. Wave C: Code quality bar (best, still small)

**Goal:** no obvious “wonky CLI” nits on first public day.

| # | Task | Done when |
|---|------|-----------|
| C1 | Replace CLI `to_string_pretty(...).unwrap()` with error mapping | doctor/cache/repair cannot panic on serialize |
| C2 | `events.rs` serialize: `Result` or documented infallible path | no bare expect in library surface if avoidable |
| C3 | Filter/present stdout newline policy (always end with `\n` for text modes) | `filter` output stable for shell |
| C4 | Keep scanner `last().unwrap()` only if invariant comment + test | documented |
| C5 | Full local gate | fmt, clippy `-D warnings`, test, deny |

**Not in Wave C:** semantic engine rewrite, new harness paint, Phase 13.

**Exit:** local gate green; optional `cargo tarpaulin` / coverage not required.

---

## 7. Wave D: Evidence bar

**Goal:** every public sentence has a check.

| # | Evidence | Command / artifact |
|---|----------|--------------------|
| D1 | Unit + integration | `cargo test --locked --workspace` |
| D2 | Lint | `cargo clippy --locked --workspace -- -D warnings` |
| D3 | Format | `cargo fmt --all -- --check` |
| D4 | Advisories/licenses | `cargo deny check` |
| D5 | Functional smoke | filter, present, hook, doctor, repair consent, currency, bare ASCII, URL protect |
| D6 | Machine channel | present/hook stdout: no `0x1b` |
| D7 | Path hygiene | `git grep` over `$(git rev-list main)` empty for lab roots |
| D8 | CI | Actions success on freeze parent SHA (ubuntu + macos) |
| D9 | (Stretch) Linux release binary | `release-prep` or manual `x86_64-unknown-linux-gnu` build + strings scan |
| D10 | (Stretch) Second clean-dir smoke | clone into `/tmp`, install, run D5 |

**Exit:** checklist D1–D8 recorded (comment in PHASE_GATES or PUBLIC_SMOKE draft). D9–D10 recommended for “best,” not optional-sloppy: do D10 at least once private; D9 if time.

---

## 8. Wave E: Private freeze `v0.1.1`

**Goal:** single immutable candidate.

### E1. Freeze commit contents

Must include:

- Wave A + B docs  
- Wave C code if done  
- Updated `docs/RELEASE_DIGESTS.md` for **0.1.1**  
- Path-scrubbed SBOM under `docs/release/`  

### E2. Binary build (required for best cut with assets)

```bash
export CARGO_TARGET_DIR=/tmp/beavermath-public-build
export RUSTFLAGS="--remap-path-prefix=$(pwd)=/src/beavermath --remap-path-prefix=$HOME/.cargo=/cargo -C strip=symbols -C debuginfo=0"
cargo build --locked --release -p beavermath-cli
cargo build --locked --release -p beavermath-cli --target x86_64-apple-darwin
# pack tar as uid/gid 0; names beavermath-0.1.1-*.tar.gz
# strings gate: no /Users/ and no Desktop/beavermath
```

If Linux stretch (D9): build `x86_64-unknown-linux-gnu` the same way; add to digests only if scan clean.

### E3. SBOM

Generate + scrub host `path+file:///...` to package ids. Scan for `/Users/`.

### E4. Digests file

Record: commit, lock hash, each archive, each unpacked binary, SBOM, toolchain, remap note.

### E5. Tag

```bash
git tag -a v0.1.1 -m "BeaverMath 0.1.1 public candidate"
```

Tag target **is** the freeze commit. Do not move tag after binary build without rebuild.

### E6. Pre-flip red team (still private)

| Check | Pass |
|-------|------|
| `git rev-list -n1 v0.1.1` == freeze SHA | yes |
| `git grep` lab paths on `main` | 0 |
| Download-or-local strings on every archive | 0 lab paths |
| CI green on freeze SHA | yes |
| README still says private candidate | yes |
| No secrets in tree | yes |

### E7. Repo governance and security gate (pre-flip, still private)

Configure **before** visibility flips so public day is not a naked repo.

| # | Control | Done when |
|---|---------|-----------|
| E7.1 | Branch protection on `main` | PR or direct-push rules set; **required status check** = `ci` / `test` job from `.github/workflows/ci.yml` |
| E7.2 | Tag hygiene | Document: do not force-move `v0.1.1` after public; enable tag protection if plan allows |
| E7.3 | Secret scanning | GitHub secret scanning on (and push protection if available on the account/plan) |
| E7.4 | Dependency alerts | Dependabot alerts on; add `.github/dependabot.yml` for `cargo` + `github-actions` (weekly) |
| E7.5 | Security advisories | Private vulnerability reporting enabled **at flip**; `SECURITY.md` points at it |
| E7.6 | Community files | `CONTRIBUTING.md` (how to build, test, claim discipline); issue templates: bug, dishonest-claim/docs |
| E7.7 | Actions pin note | Prefer SHA-pinned actions when touching workflows; at least track Node 20 deprecation |

**Exit E6+E7:** freeze SHA is the public candidate **and** the GitHub project is operable as a public open-source repo on day one (checks, alerts, reporting, contribution path).

---

## 9. Wave F: Flip (explicit “approve public” only)

Order is fixed.

1. Re-run E6 red team on the freeze SHA.
2. Confirm E7 governance controls are configured (branch protection can apply while private).
3. `git push origin main` and `git push origin v0.1.1`.
4. Create GitHub Release `v0.1.1` with assets + digests; **re-download** and re-scan.
5. Set visibility public (`gh repo edit ... --visibility public --accept-visibility-change-consequences`).
6. Enable Security Advisories / private vulnerability reporting; fix `SECURITY.md` if needed.
7. README status line to public 0.1.1 (follow-up commit OK).
8. Clean-machine smoke (D5 + clone via HTTPS).
9. Record smoke in `docs/PUBLIC_SMOKE.md` or PHASE_GATES.
10. Verify branch protection still requires CI on `main` after public.
11. **Do not** crates.io same day unless separate approve.

**Abort conditions:** any path hit in assets; CI red; digest mismatch; accidental claim of full in-chat; missing security reporting path.

---

## 10. Wave G: Post-public hardening (first week)

| # | Task |
|---|------|
| G1 | Confirm Dependabot PRs policy (auto-merge off; CI required) |
| G2 | Pin or bump Actions if Node 20 deprecation becomes failure |
| G3 | Linux binary release if not in E (`v0.1.2` OK) |
| G4 | Attestations/cosign if deferred from E/F |
| G5 | Close stale private-only wording in docs if any remain |
| G6 | CODEOWNERS optional; triage labels for bugs vs claim disputes |

---

## 11. Wave H: Deferred tracks (after public 0.1.1 is real)

Keep these **named** so they do not reopen 0.1 scope.

### H1. Track 12R (release residual)

- crates.io (API freeze review first)  
- Windows binary if claimed  
- cosign / attestations  
- fuller multi-arch matrix  
- RaTeX / ttf-parser migration (`RUSTSEC-2026-0192`)  

### H2. Track H (harness product bar)

| Host | Next real work |
|------|----------------|
| OMP | O3 only if upstream content renderer API exists |
| OpenCode | C2 only with display/source split |
| Grok | P1 only with host paint API or maintained fork |
| Claude / Codex | IP3/IP4 host display hooks; else stay oneshot-honest |

### H3. Track 13 (protocols)

13A probe experimental build; 13B Sixel; 13C SSH/tmux; 13D locales/a11y.  
No default-on behavior in 0.1.x.

### H4. Track S (semantic depth)

Deeper linearization / MathML only with fixtures and claim review. Do not silently expand README a11y language.

---

## 12. Decision register (best path defaults)

| Topic | Default for best path |
|-------|------------------------|
| First public version | **0.1.1** |
| crates.io day one | **No** |
| macOS binaries day one | **Yes** (remapped, scanned) |
| Linux binary day one | **Prefer yes if cheap**; else source + CI test, binary in 0.1.2 |
| Signatures day one | **No** (digests yes) |
| History rewrite again | **No** unless new path leak |
| Public flip without freeze match | **No** |
| Feature work during freeze week | **No** except C-wave polish |

---

## 13. Execution schedule (suggested)

Not calendar-forced; dependency order.

| Phase | Contents | Stop condition |
|-------|----------|----------------|
| Day 1 | Wave A + B | README/SUPPORT/CHANGELOG honest; ledger consistent |
| Day 2 | Wave C + D1–D8 | local + CI green; smoke scripted |
| Day 3 | Wave E | tag v0.1.1; assets; red team pass |
| Day 4 | Pause for **approve public** | human gate |
| Day 4/5 | Wave F | public + smoke recorded |
| Following week | Wave G | hardening |
| Later | Wave H tracks | separate plans |

Comprehensive path accepts a pause at Day 4. Fast path skips C polish and Linux stretch; **best path does not skip A/B/E/F integrity.**

---

## 14. One-page card

```text
A  Fix stale “already public” docs
B  Simple honest README + SUPPORT + CHANGELOG 0.1.1
C  CLI panic/newline polish
D  Evidence: test/clippy/fmt/deny/smoke/path/CI (+ clean-dir smoke)
E  Freeze: remapped binaries + SBOM + digests + tag v0.1.1
   STOP for approve public
F  Release assets, re-scan, visibility public, security, smoke
G  Post-public hardening
H  12R / harness bar / Phase 13 / semantic depth (later)
```

---

## 15. Immediate next action

When you want execution (not more planning):

1. Say **`execute best path`** (or **`go A`** / **`go A through E`**).  
2. Default first execution block: **Waves A + B** (docs ledger + README), then stop for review before binary freeze if you want a visual check.  
3. Full private prep without flip: **`go A through E`**.  
4. Flip only: **`approve public`** after E red team is green.

---

## 16. Related files

| File | Role |
|------|------|
| [GO_PUBLIC_PLAN.md](GO_PUBLIC_PLAN.md) | Shorter freeze/flip checklist + README draft text |
| [PUBLISH_ROADMAP.md](PUBLISH_ROADMAP.md) | R1/R2/R3 and history rules |
| [PHASE12_PUBLICATION.md](PHASE12_PUBLICATION.md) | Freeze status |
| [PHASE13_SCOPE.md](PHASE13_SCOPE.md) | Post-v1 protocols only |
| [PHASE_GATES.md](PHASE_GATES.md) | Live vs blocked harness ledger |
| [INCHAT_PRESENTATION.md](INCHAT_PRESENTATION.md) | Product bar definition for Track H |
