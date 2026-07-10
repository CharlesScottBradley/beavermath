# BeaverMath go-public plan

**Purpose:** one ordered plan from today’s private tree to a defensible public 0.1 release.  
**Status:** plan only (2026-07-10). Does **not** flip visibility or publish crates.  
**Writing bar:** direct, concrete, short. No overclaims. Numbers and paths over adjectives.

Related:

- [PUBLISH_ROADMAP.md](PUBLISH_ROADMAP.md): R1/R2/R3 stages, history rules  
- [PHASE12_PUBLICATION.md](PHASE12_PUBLICATION.md): freeze / flip checklist  
- [RELEASE_DIGESTS.md](RELEASE_DIGESTS.md): hashes  
- [PHASE_GATES.md](PHASE_GATES.md): harness honesty  

---

## 0. Goal

Ship a public GitHub repository that a cold reviewer can:

1. understand in under two minutes from the README  
2. install and run without lab paths or private context  
3. trust on honesty (what works / what does not)  
4. verify with CI green and path-clean history/assets  

**First public product version:** `0.1.1` (recommended) or a re-frozen `0.1.0` only if digests/tag/HEAD match exactly.

**Day-one scope:**

| In | Out |
|----|-----|
| Public source on GitHub | crates.io |
| macOS CLI binaries (optional but preferred) | Linux/Windows binaries (follow-up OK) |
| Path-clean history on `main` | Full in-chat body rewrite on every harness |
| Honest partial harness status | Sixel / probe / SSH-tmux graphics |
| SBOM + deny policy | Cosign/sigstore required on day one |

---

## 1. Ground truth (start here; re-check before flip)

Recorded 2026-07-10; re-run commands on flip day.

| Fact | Value |
|------|--------|
| Visibility | **PRIVATE** |
| Branch | `main` clean vs origin |
| Latest CI on main | green (`docs: honest private status…`, `fix(ci): rustfmt…`) |
| Path hits on `origin/main` history | **0** for `/Users/beav` and `file:///Users` |
| Release assets (last scan) | BIN_CLEAN + SBOM_CLEAN |
| crates | all `publish = false` |
| Tag lag | `v0.1.0` **behind** current main (CI polish + honesty docs after tag) |

**Implication:** do **not** open public on the old `v0.1.0` freeze alone. Cut a **new freeze** from current HEAD (`v0.1.1` preferred).

---

## 2. Principles (non-negotiable)

1. **Exact candidate.** Freeze digests → approve → build once → attach those bytes → open. No rebuild after approval under the same tag.  
2. **No lab paths.** Tip, public-bound history, SBOM, and release binaries must all scan clean.  
3. **Honesty over completeness.** Partial harnesses are fine if labeled partial.  
4. **README is the product surface.** Short, install-first, claim-disciplined. Internal roadmaps stay linked, not front-page novels.  
5. **crates.io is separate.** Default day one: skip.  
6. **Repo stays private until the final flip step.** All polish happens private.

---

## 3. Target public README (simple)

Replace the current long README with this shape. Keep dual-license and security links. Drop mermaid, long PRD voice, and “planned architecture” as the hero.

### 3.1 Draft content (copy target)

```markdown
# BeaverMath

Terminal math for agent harnesses. Strict LaTeX in, readable text out, source unchanged.

## What it does

Models write `$...$` / `$$...$$` / `\(...\)` / `\[...\]`. BeaverMath turns completed text into:

- **semantic** linear text (default)
- optional **unicode** / **ascii**: optional **PNG/SVG** when you ask for artifacts
- optional **Kitty / iTerm2** pixels only with trusted capability (no terminal probing)

Exact source bytes and SHA-256 digests stay authoritative. Bare ASCII is not auto-math. Repair is opt-in.

## Install

```bash
git clone https://github.com/CharlesScottBradley/beavermath.git
cd beavermath
cargo install --path crates/beavermath-cli --locked
beavermath --version
```

macOS binaries (when published): GitHub Releases → verify SHA-256 in `docs/RELEASE_DIGESTS.md`.

## Quick use

```bash
beavermath filter 'The answer is $x^2+y^2=z^2$.'
beavermath present 'From Maxwell, $E=mc^2$.'
beavermath doctor
```

Machine JSON (hooks / present) contains no terminal escape sequences.

## Works today (0.1.x)

| Surface | Status |
|---------|--------|
| CLI `filter` / `present` / `render` / `hook` / `doctor` | live |
| URL + inline-code protection | live |
| OMP extension | partial (preview/widget; not assistant-body rewrite) |
| OpenCode plugin | partial / oneshot |
| Grok Build | session VIEWPORT / sidecar; stock TUI paint blocked |
| Claude Code / Codex | oneshot-oriented |

## Not claimed in 0.1.x

- Sixel, active terminal probing, SSH/tmux graphics
- Full in-chat body paint on every harness
- crates.io packages
- Cross-emulator pixel byte identity without pinned CI goldens
- Repair as authority over source

## Safety defaults

- Offline core (no telemetry, no hidden network)
- Hostile TeX commands rejected (`\input`, `\write18`, …)
- Machine stdout stays free of graphics protocol bytes
- Graphics require trusted capability + direct TTY

## Docs

- [CHANGELOG](CHANGELOG.md)
- [SECURITY](SECURITY.md)
- [SUPPORT](SUPPORT.md)
- [THIRD_PARTY](THIRD_PARTY.md)
- [Release digests](docs/RELEASE_DIGESTS.md)
- [Phase gates](docs/PHASE_GATES.md)
- [Go-public plan](docs/GO_PUBLIC_PLAN.md)

## License

`MIT OR Apache-2.0`: [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
```

### 3.2 README writing rules (lab standard)

- Short sentences for claims.  
- No em-dashes. No “it’s not X, it’s Y.”  
- No hedging filler.  
- Tables for status; no symmetric “First / Second / Third” scaffolding.  
- No “public” wording until visibility is public.  
- Prefer `cargo install --path` and `git clone` over prose.

---

## 4. Honesty fixes (docs)

Do these **before** freeze, all on private main.

| File | Fix |
|------|-----|
| `README.md` | Replace with §3.1 draft; status line only becomes “public 0.1.1” **after** flip |
| `CHANGELOG.md` | Add `[0.1.1]` entry; stop saying “public” while private; fix outdated “multi-arch/SBOM not produced” if assets exist |
| `SUPPORT.md` | Install matrix: macOS source + binary; Linux source; Windows source (if untested, say so) |
| `SECURITY.md` | Private report path until public Security Advisories enabled at flip |
| `adapters/README.md` | Keep single status table; OMP = O2 partial / O3 blocked (already corrected) |
| `docs/PHASE12_PUBLICATION.md` | Align with **still private** + next tag `v0.1.1` |
| `docs/RELEASE_DIGESTS.md` | Rewrite for the new freeze commit + new binary hashes |
| Internal SPIKE docs | OK to keep lab detail; must not be required for install |

**Claim discipline (copy into README + CHANGELOG):**

Allowed: strict discovery, source digests, semantic present/filter, machine-safe JSON, Kitty/iTerm2 only with trusted capability, partial harness adapters.  

Forbidden: Sixel, probe, SSH-tmux pixels, full Grok/OpenCode/Claude/Codex in-chat body paint, repair-as-source, cross-emulator pixel identity, “public” while private.

---

## 5. Code polish before freeze (small, high leverage)

Not a feature freeze-breaker list. Do only if still open at freeze time.

| Item | Why |
|------|-----|
| CLI JSON `unwrap` on doctor/cache/repair print | avoid rare panic in public CLI |
| Filter trailing newline consistency | small UX quality |
| `events.rs` `expect("serialize")` | same class |
| Confirm no `/Users/` in tip + `git rev-list main` | path gate |

Local gate command (must match CI):

```bash
cargo fmt --all -- --check
cargo clippy --locked --workspace -- -D warnings
cargo test --locked --workspace
cargo deny check
```

Remote: latest `main` CI run **success** on the freeze SHA.

---

## 6. Freeze procedure (private) → `v0.1.1`

Prefer **new tag** so old private `v0.1.0` can remain historical.

### 6.1 Pre-freeze checklist

- [ ] README rewritten (§3) with **private** status still accurate  
- [ ] CHANGELOG 0.1.1 written  
- [ ] Adapters status table honest  
- [ ] Local CI-equivalent green  
- [ ] GitHub Actions green on the freeze commit  
- [ ] `git grep` over `$(git rev-list main)` has **zero** lab absolute paths  
- [ ] No secrets in tip or history (keys, tokens, `.env`)  

### 6.2 Build path-clean macOS binaries

From repo root (literal remaps; full absolute repo path first):

```bash
export CARGO_TARGET_DIR=/tmp/beavermath-public-build
export RUSTFLAGS="--remap-path-prefix=$(pwd)=/src/beavermath --remap-path-prefix=$HOME/.cargo=/cargo -C strip=symbols -C debuginfo=0"
cargo build --locked --release -p beavermath-cli
cargo build --locked --release -p beavermath-cli --target x86_64-apple-darwin

mkdir -p dist/public
cp "$CARGO_TARGET_DIR/release/beavermath" dist/public/beavermath-aarch64-apple-darwin
cp "$CARGO_TARGET_DIR/x86_64-apple-darwin/release/beavermath" dist/public/beavermath-x86_64-apple-darwin

# strings gate
strings dist/public/beavermath-aarch64-apple-darwin | rg '/Users/|Desktop/beavermath' && exit 1
strings dist/public/beavermath-x86_64-apple-darwin | rg '/Users/|Desktop/beavermath' && exit 1

# pack as root ownership
(cd dist/public && cp beavermath-aarch64-apple-darwin beavermath && \
  COPYFILE_DISABLE=1 tar --uid=0 --gid=0 --uname=root --gname=root \
  -czf beavermath-0.1.1-aarch64-apple-darwin.tar.gz beavermath && rm beavermath)
# same for x86_64 with 0.1.1 name
```

Regenerate SBOM without host paths (scrub `path+file:///Users/...` → `pkg:cargo/...` if cyclonedx emits them).

### 6.3 Record digests

Update `docs/RELEASE_DIGESTS.md`:

- freeze commit SHA  
- `Cargo.lock` SHA-256  
- both tar.gz SHA-256  
- both unpacked binary SHA-256  
- SBOM SHA-256  
- note: remapped + stripped; strings scan clean  

Commit digests **on the freeze tip** (or amend only if not yet tagged).

### 6.4 Tag

```bash
git tag -a v0.1.1 -m "BeaverMath 0.1.1 public candidate"
# do not push tag until assets recorded and maintainer approves flip sequence
```

**Rule:** tag SHA == freeze commit == binary build source.

---

## 7. Flip day sequence (only with explicit “approve public”)

Execute in order. Stop on any failure.

1. **Re-verify freeze**: `git rev-parse HEAD` == tag target  
   - digests file matches files to attach  
   - path scan on `main` still 0  
   - CI green on that SHA  

2. **Push tag** (if not already)  
   `git push origin main && git push origin v0.1.1`

3. **GitHub Release**: title `BeaverMath 0.1.1`  
   - notes from CHANGELOG known limitations  
   - attach: both tar.gz, SBOM, SHA256SUMS  
   - re-download assets; `strings` + `rg` path scan must pass  

4. **Open visibility**  
   ```bash
   gh repo edit CharlesScottBradley/beavermath \
     --visibility public \
     --accept-visibility-change-consequences
   ```  

5. **README status line**: switch “private candidate” → “public 0.1.1” in a **follow-up commit** if you prefer freeze purity, **or** include the public status in the freeze commit only if flip is immediate the same hour.  
   - Prefer: freeze with private wording → flip → one-line status commit (acceptable; does not rebuild binaries).  

6. **Security Advisories**: enable GitHub private vulnerability reporting / security advisories  
   - SECURITY.md points at that channel  

7. **Post-public smoke (clean machine or clean dir)**  
   ```bash
   git clone https://github.com/CharlesScottBradley/beavermath.git
   cd beavermath
   cargo test --locked --workspace
   cargo install --path crates/beavermath-cli --locked
   beavermath filter 'The answer is $x^2+y^2=z^2$.'
   beavermath present 'From Maxwell, $E=mc^2$.'
   ```  
   Record pass in `docs/PHASE_GATES.md` or a short `docs/PUBLIC_SMOKE.md`.

8. **Do not** publish crates.io on flip day unless separately approved.

---

## 8. Work breakdown (execution order)

### Wave P0: Docs & honesty (1 session)

| # | Task | Done when |
|---|------|-----------|
| P0.1 | Rewrite README to §3.1 (status still private) | cold-reader can install from README alone |
| P0.2 | CHANGELOG 0.1.1 + limitations accurate | no false “public” / no stale “no SBOM” |
| P0.3 | SUPPORT install matrix | clone path works; binary verify path documented |
| P0.4 | PHASE12 + RELEASE_DIGESTS placeholders for 0.1.1 | consistent with this plan |
| P0.5 | Spot-check adapters/SPIKE for required absolute paths | none required for install |

### Wave P1: Code gate (same or next session)

| # | Task | Done when |
|---|------|-----------|
| P1.1 | Local fmt/clippy/test/deny | all green |
| P1.2 | Optional unwrap polish | no public CLI panic on doctor/repair print |
| P1.3 | Confirm CI green on pushed tip | Actions success on freeze parent |

### Wave P2: Freeze & assets (private)

| # | Task | Done when |
|---|------|-----------|
| P2.1 | Remapped binary build both macOS targets | strings clean |
| P2.2 | SBOM scrubbed | no `/Users/` |
| P2.3 | Digests committed | RELEASE_DIGESTS matches files |
| P2.4 | Tag `v0.1.1` on that commit | tag == HEAD |
| P2.5 | Full-history path scan on public-bound refs | 0 hits |

### Wave P3: Flip (explicit approve)

| # | Task | Done when |
|---|------|-----------|
| P3.1 | Release assets uploaded + re-scanned | BIN/SBOM clean |
| P3.2 | Visibility public | `gh repo view` public |
| P3.3 | Security reporting live | SECURITY.md accurate |
| P3.4 | Clean-machine smoke | recorded pass |
| P3.5 | Status line “public 0.1.1” | README matches reality |

### Wave P4: After public (not blockers)

- Linux binary via `release-prep`  
- crates.io decision  
- cosign / attestations  
- OMP O3 upstream  
- RaTeX / `ttf-parser` tracking (`RUSTSEC-2026-0192`)  

---

## 9. Decision log

| Decision | Choice | Reason |
|----------|--------|--------|
| First public tag | **`v0.1.1`** | main moved after private `v0.1.0`; avoid lagging tag |
| crates.io day one | **No** | API young; `publish = false` |
| Linux binary day one | **Optional** | source install is enough; macOS assets nice-to-have |
| Signatures day one | **Optional** | digests required; cosign follow-up |
| Sequence | **S2-lite** | private freeze + assets, then flip (not source-only unless assets slip) |
| Public flip | **Blocked** until Waves P0-P2 done + explicit approve | path + honesty + freeze integrity |

---

## 10. Flip go / no-go table

| Gate | Required |
|------|----------|
| Repo private during prep | yes |
| README simple + honest | yes |
| CI green on freeze SHA | yes |
| Path scan main history 0 | yes |
| Binary strings scan 0 | yes if shipping binaries |
| SBOM path scan 0 | yes if shipping SBOM |
| Tag == freeze SHA == binary source | yes |
| Maintainer “approve public” | yes |
| crates.io | no |

---

## 11. One-page execution card

```text
NOW (private)
  1. Rewrite README (simple, private status)
  2. CHANGELOG/SUPPORT honesty for 0.1.1
  3. fmt/clippy/test/deny + CI green
  4. Rebuild remapped macOS binaries + scrub SBOM
  5. Write RELEASE_DIGESTS; tag v0.1.1

FLIP (approve only)
  6. GitHub Release attach assets; re-scan
  7. gh repo visibility public
  8. enable security reporting
  9. clean-machine smoke
  10. README status → public 0.1.1

LATER
  Linux binaries, crates.io, signatures, O3, RaTeX upgrade
```

---

## 12. Explicit non-goals for this public cut

- Becoming a full MathJax/KaTeX replacement in every TUI  
- Guaranteeing in-chat paint inside closed harness binaries  
- Claiming WCAG-complete math a11y from current semantic heuristics  
- Rewriting git history again unless a new path leak appears  

---

## 13. Owner cue

Default execution order when you say **go**:

1. **P0 docs** (README + CHANGELOG + SUPPORT)  
2. **P1 gates**  
3. **P2 freeze `v0.1.1`**  
4. Stop for **approve public**  
5. **P3 flip**  

If you want source-only public faster: skip binary attach in P2/P3, still require path-clean source + README + CI; ship binaries as `v0.1.2`.
