# Product Requirements Document: BeaverMath

**Product, repository, and CLI:** `beavermath`  
**Repository:** `CharlesScottBradley/beavermath`  
**Implementation language:** Rust  
**Primary renderer:** RaTeX  
**License:** `MIT OR Apache-2.0`  
**Release policy:** private until the approved v1 publication event  
**Status:** implementation contract

## 1. Introduction

BeaverMath is a local terminal mathematics renderer for developers, agent-harness users, and harness authors. It turns explicitly delimited LaTeX into readable terminal output while preserving the exact source. The default path is conservative: recognize explicit math, derive semantic text and optional visual artifacts, then select a terminal presentation only when the output channel and terminal capability are established.

Terminal pixels are an enhancement. Safe, copyable, linear UTF-8 is the universal result for TTYs, pipes, files, CI logs, structured protocols, screen readers, and unsupported terminals. Graphics never replace source or semantic text.

RaTeX is the primary math parser and renderer behind a BeaverMath-owned adapter. RaTeX describes a pure-Rust TeX parser, layout engine, and renderer, which fits the offline and cross-platform design goals ([RaTeX](https://github.com/erweixin/RaTeX)). The adapter prevents upstream types and error strings from becoming BeaverMath's public API.

OMP is the first harness integration. Codex, Claude Code, and OpenCode follow through a shared structured artifact contract. The model-facing behavior is automatic: a model writes ordinary `$...$` or `$$...$$`; the host adapter renders the completed content without requiring a rendering tool call or prompt instruction.

## 2. Fixed decisions

1. The product, repository, Cargo package, and executable are named `beavermath`.
2. The implementation is Rust. RaTeX is the primary renderer, isolated behind an internal adapter.
3. Source is dual-licensed under `MIT OR Apache-2.0`.
4. The repository, issues, CI artifacts, release candidates, and packages remain private until the approved v1 publication event.
5. Strict explicit-LaTeX discovery is the default at every entry point.
6. V1 recognizes `$...$`, `$$...$$`, `\(...\)`, `\[...\]`, and fenced `math`, `latex`, or `tex` blocks. Host-protected code ranges take precedence over delimiter discovery.
7. The exact original input and each exact expression slice remain immutable. Every result carries SHA-256 digests computed before parsing.
8. Arbitrary ASCII or Unicode reconstruction is opt-in. Reconstructed TeX is a derivative with confidence, provenance, and source mapping. It never replaces the original.
9. Model-assisted repair is provided by an explicit host-owned provider interface. BeaverMath core performs no network request, telemetry, update check, or hidden provider call.
10. Linear semantic text is the universal output. PNG, SVG, MathML, visual Unicode, and terminal protocol payloads are optional derivatives.
11. V1 graphics require a direct TTY plus a trusted, transaction-scoped capability attestation from the host or an explicit local trusted-capability option. Terminal names and environment hints may deny or nominate a candidate but never authorize protocol bytes. V1 performs zero active terminal queries.
12. Redirected, captured, CI, log, MCP stdio, and other machine channels contain no terminal graphics or probe sequences.
13. BeaverMath never screen-scrapes a harness TUI. Adapters consume documented events, message APIs, or rendering hooks.
14. Structured adapters carry artifacts and metadata, never terminal escape sequences.
15. Persistent caching is local, private, bounded, and opt-in. Process-local caching is allowed by default.

## 3. Definitions

- **Original input:** the exact valid UTF-8 byte sequence received at the request boundary.
- **Exact source:** the byte-identical original expression slice, including delimiters when present.
- **Source span:** a zero-based, half-open byte range into the original input.
- **Strict mode:** discovery of explicitly delimited LaTeX only.
- **Protected range:** a host-supplied byte range where math discovery is disabled, such as inline code, fenced code, a URL, or a shell transcript.
- **Semantic text:** deterministic, linear reading-order text that states established mathematical structure.
- **Visual text:** cell-oriented Unicode or ASCII optimized for sighted reading.
- **Artifact:** a validated derivative with media type, dimensions where applicable, digest, exact-source association, and accessible text.
- **Repair proposal:** opt-in inferred TeX with source mapping, confidence, provider identity, and provenance.
- **Presentation overlay:** display-only content associated with a source span. It does not mutate the canonical message or transcript.

## 4. Users and primary workflows

1. **Terminal reader:** reads model output or documentation containing LaTeX and wants legible math without leaving the terminal.
2. **OMP user:** wants automatic math previews while session history, copy, resume, branch, and model context retain the original response.
3. **Automation user:** needs deterministic text or structured artifacts from stdin, files, JSONL, or MCP without control-sequence contamination.
4. **Screen-reader or braille user:** needs a semantic representation that does not depend on pixels, color, box drawing, or alignment.
5. **Harness integrator:** needs a versioned request/result/event contract, idempotent replay, stable diagnostics, cancellation, and explicit capabilities.
6. **Security-conscious maintainer:** needs bounded untrusted-input handling, offline operation, typed terminal encoders, auditable dependencies, and reproducible releases.

Representative workflows:

- Render `The answer is $x^2 + y^2 = z^2$.` while preserving every non-math byte.
- Render a multiline display expression as an inline image in a confirmed Kitty or iTerm2 session.
- Pipe a document to a file and receive deterministic linear UTF-8 with no escape sequences.
- Use Apple Terminal and receive semantic and visual-text output without an unsupported image protocol.
- Ask for a repair proposal for `sqrt(x^2 + 1)` while retaining that exact source.
- Resume, branch, replay, replace, or delete a harness message without orphaning or duplicating overlays.
- Render one explicit expression to SVG or PNG without consulting terminal capability.
- Inspect backend selection with `doctor` or an explain field without changing content stdout.

## 5. Goals

- Render supported explicit LaTeX automatically without changing source content.
- Produce a useful, safe result for TTY, pipe, file, CI, SSH, multiplexer, unknown-terminal, and structured-protocol destinations.
- Keep semantic reconstruction consent-based, auditable, reversible, and independently measurable.
- Give OMP, Codex, Claude Code, and OpenCode one adapter-neutral artifact and lifecycle contract.
- Prevent TeX execution, terminal injection, unbounded resource use, hidden networking, cache disclosure, and cross-message overlay corruption.
- Ship a private v1 release candidate with reproducible artifacts, complete license provenance, and measured compatibility before public publication.

## 6. User stories

### BM-001: Establish the private project baseline

**Description:** As a maintainer, I need a private, licensed repository with an approved product contract so implementation starts from one source of truth.

**Acceptance criteria:**
- [ ] The private GitHub repository is `CharlesScottBradley/beavermath` with `main` as its default branch.
- [ ] `README.md`, `ROADMAP.md`, this PRD, `LICENSE-MIT`, and `LICENSE-APACHE` are present.
- [ ] The README states private pre-v1 status and contains no claim that unfinished features work.
- [ ] The dual-license expression is `MIT OR Apache-2.0` everywhere.

### BM-002: Freeze the v1 logical contract

**Description:** As an integrator, I need versioned schemas and fixtures so CLI and harness adapters cannot drift.

**Acceptance criteria:**
- [ ] `RenderRequestV1`, `MathArtifactV1`, `OutputContextV1`, diagnostics, and exit statuses are documented and serialized.
- [ ] Every fixture includes exact input, input digest, expression spans, expression digests, semantic output, and expected status.
- [ ] Unknown optional fields are ignored; missing required fields reject the request.
- [ ] Schema compatibility tests cover older readers and newer additive fields.

### BM-003: Preserve the input envelope

**Description:** As a user, I need exact source recovery after every success and failure.

**Acceptance criteria:**
- [ ] Valid UTF-8 is retained byte-for-byte, including BOM, CRLF, tabs, normalization form, leading spaces, and trailing newlines.
- [ ] SHA-256 is computed before parsing for the full input and each selected expression.
- [ ] Invalid UTF-8 fails before scanning with the first invalid byte offset and a lossless structured byte representation.
- [ ] Parse, render, timeout, cancellation, and limit failures retain the original and its digest.

### BM-004: Discover strict inline math

**Description:** As a reader, I want inline explicit math detected without selecting currency, code, or escaped delimiters.

**Acceptance criteria:**
- [ ] `$...$` and `\(...\)` are recognized with deterministic escaping and nesting rules.
- [ ] Host-protected ranges, inline code, URLs, and escaped delimiters are never selected.
- [ ] Unclosed delimiters remain source text.
- [ ] Discovery is invariant under streaming chunk boundaries.

### BM-005: Discover strict display math

**Description:** As a reader, I want display expressions detected and assigned block layout.

**Acceptance criteria:**
- [ ] `$$...$$`, `\[...\]`, and fenced math blocks are recognized.
- [ ] Delimiter precedence and mixed-delimiter behavior are fixed by fixtures.
- [ ] Unterminated blocks remain source text with a stable diagnostic.
- [ ] Display spans never overlap inline spans.

### BM-006: Build a lossless document partition

**Description:** As an adapter author, I need text and math spans that reconstruct the original exactly.

**Acceptance criteria:**
- [ ] Concatenating all exact span slices reproduces the original bytes.
- [ ] Spans are ordered, non-overlapping, and cover the complete input.
- [ ] Source maps use UTF-8 byte offsets and expose optional line/column views separately.
- [ ] Fuzzing proves no panic, overlap, gap, or quadratic rescanning.

### BM-007: Integrate RaTeX behind a bounded worker

**Description:** As a maintainer, I need math rendering without exposing the process to unbounded or external side effects.

**Acceptance criteria:**
- [ ] A BeaverMath adapter maps the internal AST/request to RaTeX and normalizes errors.
- [ ] Rendering has no file, network, environment-secret, shared-memory, or external-process access beyond immutable packaged assets.
- [ ] Each render is cancellable and subject to enforced wall-time, memory, nesting, token, dimension, and output limits.
- [ ] Worker failure returns exact source plus semantic fallback and never aborts the parent process.

### BM-008: Produce semantic linear text

**Description:** As a screen-reader, braille, log, or narrow-terminal user, I need mathematical structure in reading order.

**Acceptance criteria:**
- [ ] Fractions, roots, scripts, matrices, cases, sums, products, integrals, accents, and limits have deterministic linearization.
- [ ] Ambiguous notation is read literally unless source semantics establish a stronger meaning.
- [ ] Every parsed node maps to semantic output or is explicitly decorative.
- [ ] Failure produces a labeled literal-source fallback, never empty alt text.

### BM-009: Produce visual text fallbacks

**Description:** As a text-terminal user, I want readable math without relying on pixel protocols.

**Acceptance criteria:**
- [ ] Reversible ASCII, semantic linear text, and visual Unicode are separate formats.
- [ ] Width-sensitive layout operates on grapheme clusters under a declared Unicode width profile.
- [ ] Unknown width, mixed bidi, unsupported clusters, or insufficient columns select semantic text instead of a damaged grid.
- [ ] Apple Terminal and unknown emulators receive no unsupported graphics escapes.

### BM-010: Produce deterministic artifacts

**Description:** As an automation user, I need PNG, SVG, JSON, and optional MathML artifacts tied to exact source.

**Acceptance criteria:**
- [ ] Every artifact records schema version, exact-source digest, renderer version, configuration digest, media type, and content digest.
- [ ] PNG dimensions and decoded allocation are validated before publication.
- [ ] SVG contains no script, external reference, entity, URL, or model-controlled active content.
- [ ] Pinned CI renderer, font assets, and backend produce byte-stable golden artifacts.

### BM-011: Expose the stable CLI

**Description:** As a terminal user, I need composable commands with predictable stdout, stderr, and exit behavior.

**Acceptance criteria:**
- [ ] V1 commands include `render`, `filter`, `doctor`, `cache`, and opt-in `repair`; `probe` is reserved for a post-v1 experimental build.
- [ ] Supported formats include `auto`, `semantic`, `unicode`, `ascii`, `svg`, `png`, and `json`.
- [ ] Content is written to stdout; sanitized diagnostics are written to stderr.
- [ ] Broken pipe exits without retrying to another sink.
- [ ] JSON/JSONL modes never contain terminal protocol bytes.

### BM-012: Resolve terminal capability conservatively

**Description:** As a terminal user, I need pixel output only when the complete path supports it.

**Acceptance criteria:**
- [ ] Output kind, transport, multiplexer depth, accessibility preference, host descriptor, and evidence resolve to one deterministic decision.
- [ ] Non-TTY, CI auto mode, `TERM=dumb`, GNU screen, unconfirmed tmux, and unknown nested transports choose text.
- [ ] Environment branding can nominate a candidate but cannot independently authorize a protocol.
- [ ] `doctor` explains the decision without printing untrusted terminal replies.

### BM-013: Design post-v1 safe probing

**Description:** As a future interactive user, I need an experimental capability probe design that cannot steal input or alter terminal state.

**Acceptance criteria:**
- [ ] The code is excluded from the supported v1 build and release claims.
- [ ] Any experimental probe requires a foreground controlling TTY plus an exclusive input lease; input and output refer to the same terminal.
- [ ] Complete termios state is restored on success, timeout, malformed response, signal, panic, and cancellation.
- [ ] Unrelated bytes are returned to the input owner in order.
- [ ] Direct probes time out within 250 ms; configured remote or multiplexer probes never exceed 2 seconds.

### BM-014: Add the Kitty backend

**Description:** As a Kitty-compatible terminal user, I want efficient inline PNG display after capability confirmation.

**Acceptance criteria:**
- [ ] Only direct transfer `t=d` is supported; file and shared-memory media are unrepresentable.
- [ ] Payload chunks are at most 4096 base64 bytes and use random owned identifiers.
- [ ] Cleanup deletes only BeaverMath-owned resources and never emits delete-all.
- [ ] Exact protocol byte fixtures pass directly, over SSH PTY, and through one confirmed tmux layer.

### BM-015: Add the iTerm2 backend

**Description:** As an iTerm2 user, I want append-only inline PNG display without download behavior.

**Acceptance criteria:**
- [ ] Generated OSC 1337 frames always set `inline=1`, omit user-controlled filenames, and terminate with ST.
- [ ] Payloads respect the documented protocol limit and BeaverMath's lower resource limits.
- [ ] Capability is confirmed through a trusted descriptor or exact feature-report evidence.
- [ ] The backend is append-only because the protocol has no portable image deletion primitive.

### BM-016: Add a post-v1 Sixel backend

**Description:** As a confirmed Sixel terminal user, I may receive display-math pixels after the narrower v1 protocol surface is secure.

**Acceptance criteria:**
- [ ] Sixel is excluded from the supported v1 build and publication claims.
- [ ] DA1 false positives are rejected; explicit capability confirmation is required.
- [ ] Output is bounded, append-only, monochrome, palette-preserving, and display-math only.
- [ ] Cursor and rendition state are restored; interrupted DCS receives fixed best-effort termination.
- [ ] The backend ships only after its parser, repeat-expansion, denial-of-service, and real-terminal gates pass.

### BM-017: Enforce typed terminal output

**Description:** As a security reviewer, I need model content structurally unable to create terminal commands.

**Acceptance criteria:**
- [ ] Terminal sinks accept only private validated text or graphics frame types.
- [ ] Untrusted strings cannot become protocol keys, identifiers, paths, URLs, filenames, dimensions, or diagnostics.
- [ ] Plain output contains no ESC, C1 controls, OSC, DCS, APC, BEL, cursor rewrite, clipboard, title, link, query, or file-transfer operations.
- [ ] An independent ECMA-48 state-machine validator and adversarial PTY suite pass.

### BM-018: Add private local caching

**Description:** As a repeat user, I want faster rendering without leaking formulas or context.

**Acceptance criteria:**
- [ ] Persistent cache is opt-in, owner-only, bounded by size and age, and clearable.
- [ ] Filenames use a per-cache-secret keyed digest rather than raw source or an unkeyed expression hash.
- [ ] Entries contain render artifacts and minimum provenance, never surrounding model text, prompts, credentials, or terminal capability responses.
- [ ] Corrupt, stale, or version-incompatible entries are ignored and safely removed.

### BM-019: Implement opt-in repair proposals

**Description:** As a user, I want ambiguous ASCII or Unicode math reconstructed only when I request it.

**Acceptance criteria:**
- [ ] Strict mode never selects undelimited text.
- [ ] A proposal contains original source, inferred TeX, byte mapping, confidence, provider, model/version if applicable, and deterministic request parameters.
- [ ] Core makes no network call; a host must explicitly configure and invoke a repair provider.
- [ ] Accepting a proposal creates a presentation derivative and never edits the canonical transcript.
- [ ] Repair quality is measured separately from strict parser correctness.

### BM-020: Implement the adapter-neutral stream contract

**Description:** As a harness author, I need idempotent rendering independent of any one host.

**Acceptance criteria:**
- [ ] Events include stable message ID, revision, lifecycle state, exact content digest, protected ranges, output context, and cancellation token.
- [ ] Duplicate and out-of-order events do not duplicate or orphan overlays.
- [ ] Revisions, deletions, branch switches, resumes, and session restores invalidate only affected derivatives.
- [ ] Machine channels carry artifacts or artifact references, never terminal escapes.

### BM-021: Ship OMP extension-only preview

**Description:** As an OMP user, I want automatic post-turn previews without forking core.

**Acceptance criteria:**
- [ ] The extension observes documented message lifecycle events and renders only completed assistant content.
- [ ] It uses a custom message or widget as a derivative, preserving the assistant message as source of truth.
- [ ] Replay, branch, resume, replacement, and deletion are idempotent.
- [ ] The limitation is explicit: current documented custom renderers cover custom messages and assistant thinking, not arbitrary replacement of assistant-content spans ([OMP extension guide](https://raw.githubusercontent.com/can1357/oh-my-pi/main/docs/extensions.md)).

### BM-022: Add an OMP assistant-content renderer hook

**Description:** As an OMP user, I want true inline math overlays inside normal assistant content.

**Acceptance criteria:**
- [ ] A generic core hook receives immutable assistant content, content index, message revision, viewport context, theme, and an asynchronous rerender callback.
- [ ] The hook is presentation-only and cannot mutate provider/session content.
- [ ] Streaming uses source text; completed spans can be replaced visually while copy and session APIs retain exact source.
- [ ] The hook is generic enough for non-BeaverMath renderers and has upstream-quality tests and documentation.

### BM-023: Add structured Codex, Claude Code, and OpenCode adapters

**Description:** As a cross-harness user, I want the same behavior and artifacts in supported agent tools.

**Acceptance criteria:**
- [ ] Each adapter uses documented structured events or noninteractive output rather than screen scraping.
- [ ] OMP ships first; Codex, Claude Code, and OpenCode follow in that order.
- [ ] Missing native display hooks degrade to sidecar previews, artifact links, or safe post-turn text.
- [ ] All adapters pass the same lifecycle, source-identity, and machine-channel fixtures.

### BM-024: Publish the approved v1 atomically

**Description:** As a maintainer, I need a public v1 whose source, package, archives, checksums, and provenance match the reviewed candidate.

**Acceptance criteria:**
- [ ] Dependency advisories, licenses, bundled fonts, third-party notices, SBOM, and release archive contents are audited.
- [ ] CI builds and tests the pinned lockfile and toolchain; release dependencies use registry or vendored sources, not floating Git revisions.
- [ ] The exact approved candidate digests are promoted without rebuilding after approval.
- [ ] GitHub publication, crate publication, tags, checksums, and documentation refer to the same commit and artifact set.
- [ ] Privacy review confirms that no pre-v1 private issue, artifact, cache, secret, or model content is published.

### BM-025: Freeze diagnostics and exit behavior

**Description:** As a script author, I need stable diagnostics and statuses that do not depend on renderer wording.

**Acceptance criteria:**
- [ ] Diagnostic codes, severities, source ranges, and exit statuses are BeaverMath-owned enums.
- [ ] Human and structured diagnostics carry the same code and range.
- [ ] Raw renderer errors, terminal replies, paths, and hostile source are never printed unsanitized.

### BM-026: Validate host-protected ranges

**Description:** As a harness adapter, I need code and URL ranges excluded without weakening source mapping.

**Acceptance criteria:**
- [ ] Ranges are sorted, non-overlapping, in bounds, and aligned to UTF-8 boundaries.
- [ ] Invalid ranges reject the request before discovery.
- [ ] Range behavior is identical for whole-buffer and streamed input.

### BM-027: Deliver reversible ASCII

**Description:** As a restricted-terminal or archival user, I need a reversible ASCII representation.

**Acceptance criteria:**
- [ ] ASCII output contains bytes `0x20..0x7E` plus writer-owned LF only.
- [ ] Escaping and structural markers have an unambiguous decoder.
- [ ] Round-trip fixtures recover the exact expression source and digest.

### BM-028: Deliver width-qualified visual Unicode

**Description:** As a text-terminal user, I want two-dimensional visual math when cell widths are reliable.

**Acceptance criteria:**
- [ ] The width profile records Unicode version, ambiguous-width policy, emoji policy, and terminal family.
- [ ] Layout never splits an extended grapheme cluster.
- [ ] Unknown width or resize races discard the grid and retain semantic/source output.

### BM-029: Make artifacts immutable and content-addressed

**Description:** As an adapter and cache author, I need artifacts that cannot drift from their source or provenance.

**Acceptance criteria:**
- [ ] Artifact bytes, media metadata, source digest, renderer version, and configuration digest are bound before publication.
- [ ] Mutation requires a new artifact digest and revision.
- [ ] Resume and replay reject mismatched source/artifact associations.

### BM-030: Prove resource limits and cancellation

**Description:** As a host, I need hostile or cancelled work to terminate within fixed bounds.

**Acceptance criteria:**
- [ ] Exact-boundary and boundary-plus-one fixtures cover every input, token, depth, time, memory, raster, and output limit.
- [ ] Cancellation terminates and reaps the worker without affecting later requests.
- [ ] No failure path begins a terminal protocol frame.

### BM-031: Prove deterministic replay

**Description:** As a session user, I need the same message revision to reconstruct the same logical artifact.

**Acceptance criteria:**
- [ ] One hundred repeated logical renders per normative fixture match byte-for-byte.
- [ ] Locale, terminal environment, width, and graphics variables do not alter non-TTY logical results.
- [ ] Pinned-CI pixel artifacts and real-terminal compatibility claims are reported as separate guarantees.

### BM-032: Implement the OMP RPC and SDK wrapper

**Description:** As an OMP integrator, I need a typed wrapper around BeaverMath events and artifacts.

**Acceptance criteria:**
- [ ] RPC stdout remains protocol-only and diagnostics use a separate channel.
- [ ] Message revisions, protected ranges, cancellation, and artifact references map losslessly.
- [ ] Reconnect and replay tests create no duplicate previews.

### BM-033: Integrate Codex structured events

**Description:** As a Codex user, I want completed assistant math rendered from documented structured output.

**Acceptance criteria:**
- [ ] The adapter consumes documented noninteractive or app-server events.
- [ ] Machine stdout remains free of terminal protocol bytes.
- [ ] Missing display hooks produce artifact references or semantic post-turn output.

### BM-034: Add Codex one-shot filtering

**Description:** As a shell user, I want one-shot Codex output passed through BeaverMath without losing exit status or source.

**Acceptance criteria:**
- [ ] The wrapper preserves command exit status and stderr separation.
- [ ] Partial or failed producer output remains bounded and source-preserving.
- [ ] Redirected output is deterministic semantic text or structured JSON.

### BM-035: Integrate Claude Code hooks

**Description:** As a Claude Code user, I want a documented hook integration that remains safe in headless sessions.

**Acceptance criteria:**
- [ ] Hook payloads map to the neutral event contract without screen scraping.
- [ ] Unsupported rich display degrades to artifacts or semantic text.
- [ ] Lifecycle and exact-source fixtures match the OMP contract.

### BM-036: Add the Claude rich wrapper

**Description:** As a local Claude Code user, I want optional terminal presentation outside machine protocol channels.

**Acceptance criteria:**
- [ ] The wrapper owns the TTY and receives a trusted capability option before pixels.
- [ ] Hook and machine stdout never contain terminal escapes.
- [ ] Wrapper failure leaves the underlying Claude session usable and source intact.

### BM-037: Integrate OpenCode server and plugin events

**Description:** As an OpenCode user, I want server or plugin events rendered through the same artifact contract.

**Acceptance criteria:**
- [ ] Server, SSE, or plugin events are versioned and mapped idempotently.
- [ ] Reconnect, duplicate, reorder, replacement, and deletion fixtures pass.
- [ ] The adapter never mutates canonical message content.

### BM-039: Grok Build first-class adapter (lab)

**Description:** As a Grok Build user, I want completed assistant math rendered automatically without tool calls or screen scraping.

**Acceptance criteria:**
- [ ] Documented Stop/SubagentStop (or equivalent) mechanism and spike record.
- [ ] Semantic preview for completed turns containing explicit LaTeX; exact source retained for model/session.
- [ ] URL and inline code auto-protected.
- [ ] Hooks fail-open; machine channels free of terminal graphics protocol.
- [ ] True inline presentation only if host exposes a presentation-only API (otherwise G2a sidecar is labeled).

### BM-038: Add OpenCode one-shot filtering

**Description:** As a shell user, I want safe rendering for OpenCode one-shot output.

**Acceptance criteria:**
- [ ] Exit status, stdout/stderr separation, source bytes, and digests are preserved.
- [ ] Pipe, file, CI, and log captures contain no terminal protocol controls.
- [ ] Structured and semantic fallbacks remain useful without a native renderer hook.

## 7. Functional requirements

### 7.1 Input, parsing, and source identity

- **FR-001:** Accept valid UTF-8 from stdin, a named file, a direct expression argument, or a structured request.
- **FR-002:** Preserve exact bytes without normalization, trimming, tab expansion, newline conversion, or delimiter removal.
- **FR-003:** Compute `input_sha256` and `expression_sha256` before parsing.
- **FR-004:** Reject invalid UTF-8 before scanning and report the first invalid byte offset without printing hostile bytes raw.
- **FR-005:** Return `unchanged` for empty input or documents with no accepted math.
- **FR-006:** Discover all fixed explicit delimiter forms with deterministic precedence.
- **FR-007:** Treat host-protected byte ranges as authoritative and validate that they are ordered, in bounds, and on UTF-8 boundaries.
- **FR-008:** Never discover math inside protected inline code, fenced code, URLs, or shell examples.
- **FR-009:** Preserve unclosed, malformed, and unsupported expressions as source text with stable diagnostics.
- **FR-010:** Produce a complete, ordered, non-overlapping document partition.
- **FR-011:** Streaming and whole-buffer parsing must produce equivalent logical results.
- **FR-012:** A direct expression endpoint may accept undelimited TeX only when the caller supplies `inline` or `block` display mode.

### 7.2 Logical artifact contract

Every result must expose the following logical fields, whether serialized as Rust types, JSON, JSONL, or an adapter event:

```text
MathArtifactV1 {
  schema_version
  input_exact_utf8
  input_sha256
  expression_id
  expression_byte_range { start, end }
  expression_exact_utf8
  expression_sha256
  source_display
  discovery { explicit | direct_expression | repair_proposal }
  parse { status, diagnostics[] }
  semantic { text, locale, status, node_spans[] }
  alt { text, status }
  repair? { inferred_tex, mapping[], confidence, provider, provenance }
  mathml?
  visual { unicode?, ascii?, png?, svg? }
  provenance { beavermath_version, renderer_version, config_sha256 }
}
```

- **FR-013:** Exact-source fields and digests are present on success and every recoverable failure.
- **FR-014:** `source_display` visibly escapes terminal controls and bidi overrides while exact source remains unchanged.
- **FR-015:** `semantic.text` and `alt.text` cannot both be absent.
- **FR-016:** A visual artifact cannot exist without exact source, source digest, semantic text, alt text, and provenance.
- **FR-017:** Artifact digests cover final immutable bytes; associations include the exact-source digest.
- **FR-018:** Adapter replay and resume keys include stable message revision plus exact-content digest.
- **FR-019:** MathML is a structured artifact for capable hosts, not text printed into a terminal. MathML semantics and `intent` are emitted only when established by the AST ([MathML 4](https://w3c.github.io/mathml/)).

### 7.3 Rendering and fallback

- **FR-020:** Build semantic text before terminal capability negotiation.
- **FR-021:** Parse or render failure affects only the expression span and leaves surrounding text unchanged.
- **FR-022:** Explicit PNG or SVG artifact requests do not consult terminal capability.
- **FR-023:** V1 terminal auto mode uses this visual preference order: pixels authorized by a trusted transaction-scoped capability attestation, width-qualified visual Unicode, semantic text.
- **FR-024:** Accessibility preference, non-TTY output, unknown width, or machine channel selects semantic text with no pixel side effect.
- **FR-025:** Visual failure cannot remove, overwrite, duplicate, reorder, or change semantic/source output.
- **FR-026:** One-shot output is append-only. It does not enter the alternate screen or use carriage-return redraw.
- **FR-027:** Copy operations return exact source, not inferred TeX, visual text, labels, or an added newline.
- **FR-028:** Product-owned stdout and stderr writes are record-serialized so diagnostics cannot split an expression.

### 7.4 Terminal policy and protocols

- **FR-029:** Detect TTY status through the platform's terminal APIs. TTY is necessary but not sufficient evidence.
- **FR-030:** Never emit protocol bytes to a pipe, regular file, socket, log, CI auto channel, structured stdout, or unknown handle.
- **FR-031:** Supported v1 modes emit zero terminal queries. A named graphics preference does not force emission without trusted capability attestation.
- **FR-032:** Post-v1 experimental probing, if built, must require explicit opt-in, restore complete terminal state, preserve unrelated input, and remain outside v1 release claims.
- **FR-033:** Capability attestations are transaction-scoped and invalidated on resume, reattachment, terminal identity change, or multiplexer topology change.
- **FR-034:** Kitty uses direct transfer only, bounded chunks, owned IDs, and ID-specific cleanup ([Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)).
- **FR-035:** iTerm2 uses PNG, explicit `inline=1`, bounded OSC 1337 frames, and no user-controlled filename ([iTerm2 images](https://iterm2.com/documentation-images.html)).
- **FR-036:** Sixel is post-v1 because its DCS parser, repeat expansion, palette/cursor behavior, and lack of deletion expand the terminal attack surface.
- **FR-037:** ANSI half-block or visual Unicode output never carries essential information through color alone.
- **FR-038:** Apple Terminal receives semantic or visual text because no supported inline-image protocol is assumed.
- **FR-039:** SSH without a PTY is non-TTY. SSH with a PTY receives text in supported v1 modes unless a future trusted end-terminal attestation contract is approved.
- **FR-040:** GNU screen, tmux, SSH transport, and nested multiplexers receive text fallback in supported v1 modes. Their protocol passthrough paths remain post-v1 compatibility work.

### 7.5 Security and limits

- **FR-041:** Only typed, privately constructed terminal frames may write protocol bytes.
- **FR-042:** Model content, exact source, diagnostics, URLs, paths, filenames, and environment values never become protocol control fields.
- **FR-043:** Plain terminal text visibly escapes C0/C1 controls, ESC, DEL, terminal control strings, and bidi overrides. Writer-owned LF is the only permitted control in semantic mode.
- **FR-044:** BeaverMath emits no clipboard, title, hyperlink, notification, terminal-remote-control, arbitrary file-transfer, file-backed image, shared-memory image, or shell command.
- **FR-045:** Supported TeX is closed and resource-free. File inclusion, packages, shell escape, URL fetching, macro definition, catcode changes, and external resources fail closed.
- **FR-046:** Initial hard limits are 1 MiB input, 64 KiB per expression, 128 expressions per message, 100,000 tokens, nesting depth 64, 4096 px per axis, 4,194,304 pixels, 16 MiB decoded raster, 8 MiB encoded artifact, 500 ms per formula, and 2 seconds per message.
- **FR-047:** Arithmetic is checked before allocation. Boundary-plus-one, integer overflow, decompression-bomb, and timeout fixtures must fail before protocol emission.
- **FR-048:** Limits are calibrated during the corpus phase. Structured adapters may lower them, never raise them. Local CLI users may raise non-protocol limits only through an explicit option.
- **FR-049:** Renderer work occurs in a bounded worker with enforceable termination. Cooperative cancellation alone is insufficient for a stuck renderer.
- **FR-050:** Output failures stop at the current sink. BeaverMath never retries to a file, clipboard, URL, or different descriptor.

### 7.6 Accessibility and text behavior

- **FR-051:** Semantic-only mode is available at every entry point and overrides all graphics preferences.
- **FR-052:** Fractions, matrices, scripts, roots, cases, and operators retain relationships in linear speech/braille text.
- **FR-053:** Pixel or grid output is never the sole representation.
- **FR-054:** Default text uses terminal default foreground and background. Color cannot be the sole signal.
- **FR-055:** Plain and semantic modes contain no SGR. `NO_COLOR` disables optional product color but does not itself prove or disprove image capability.
- **FR-056:** Wrapping and clipping operate on extended grapheme clusters under a pinned Unicode data version.
- **FR-057:** Unknown cell width or bidi layout selects semantic fallback rather than breaking a grapheme or grid.
- **FR-058:** Accessibility claims are phrased as tested WCAG 2.2-derived requirements for terminal software, not blanket WCAG conformance. WCAG2ICT specifically discusses command-line, redirection, and piping contexts ([WCAG2ICT 2.2](https://www.w3.org/TR/wcag2ict-22/#text-command-line-terminal-applications-and-interfaces)).

### 7.7 CLI, cache, repair, and adapters

- **FR-059:** `beavermath render` accepts an expression, stdin, or a named file and supports explicit output format.
- **FR-060:** `beavermath filter` transforms a document while preserving non-math text.
- **FR-061:** `beavermath doctor` reports output context, evidence, selected backend, fallback reason, and limit configuration.
- **FR-062:** `beavermath probe` is reserved for a post-v1 experimental build and must never consume unrelated terminal input.
- **FR-063:** `beavermath cache stats|clear` never prints cached source by default.
- **FR-064:** `beavermath repair` is opt-in and exposes proposal provenance before presentation.
- **FR-065:** Persistent cache is disabled by default and capped initially at 256 MiB with age eviction.
- **FR-066:** Cache paths and files are owner-only. Keys use a per-cache-secret keyed digest. Surrounding prompt/context is never stored.
- **FR-067:** `beavermath mcp`, if included in v1, uses framed structured stdout exclusively and places human diagnostics on stderr.
- **FR-068:** Harness adapters receive structured artifacts, never terminal escapes or mutable cache paths.
- **FR-069:** Adapters render completed content by default. Streaming discovery may track incomplete spans but must show source until a span is complete.
- **FR-070:** Adapter lifecycle is idempotent across message update, completion, replacement, deletion, resume, branch, replay, and session switch.
- **FR-071:** Public diagnostics use stable codes, severities, byte ranges, and documented remediation; upstream error strings remain internal.
- **FR-072:** JSON Schema and generated Rust/TypeScript fixtures define `RenderRequestV1`, `MathArtifactV1`, `OutputContextV1`, and adapter lifecycle events.
- **FR-073:** Documents without accepted spans are returned byte-identically with `unchanged` status.
- **FR-074:** Scanner limits include maximum document size, expression size, span count, nesting, and token count before renderer invocation.
- **FR-075:** Every expression records explicit inline or block display mode; direct-expression calls must supply it.
- **FR-076:** Diagnostics map to original UTF-8 byte ranges and expose line/column only as a derived view.
- **FR-077:** Rendering configuration is explicit, serializable, hashable, and free of locale-dependent defaults.
- **FR-078:** Fonts and renderer assets are packaged immutably with checksums, source, version, and license provenance.
- **FR-079:** Semantic text records a locale. V1 provides one frozen English profile; additional locales cannot alter exact source or AST.
- **FR-080:** Reversible ASCII has a documented decoder and contains no non-ASCII or terminal-control bytes.
- **FR-081:** Visual Unicode reports required rows and columns before emission and is discarded if the current width no longer fits.
- **FR-082:** PNG and SVG outputs record media type, dimensions, byte length, digest, source digest, renderer version, and configuration digest.
- **FR-083:** Published artifacts are immutable. Any byte or provenance change creates a new digest and artifact revision.
- **FR-084:** Persistent cache writes use atomic create/rename, no-follow semantics, private permissions, and no shared temporary path.
- **FR-085:** Initial persistent-cache policy is 256 MiB and 30 days, with least-recently-used and age eviction; `--no-cache` bypasses reads and writes.
- **FR-086:** Persistent cache stores no raw source, surrounding prompt, user identity, credentials, terminal reply, or harness context. Source association uses a keyed digest derived from a private per-cache secret.
- **FR-087:** Repair providers require an explicit per-request or host policy grant; strict rendering never invokes them.
- **FR-088:** Repair mappings relate every inferred token to original byte ranges or mark it inserted with provider provenance.
- **FR-089:** Repair confidence never promotes a proposal into canonical content. Thresholds affect presentation eligibility only.
- **FR-090:** OMP extension preview consumes completed message events and persists only derivative state needed for idempotent replay.
- **FR-091:** The proposed OMP core hook is generic, immutable, asynchronous, presentation-only, and source-preserving.
- **FR-092:** Codex integration uses documented structured or noninteractive output and preserves producer exit status.
- **FR-093:** Claude Code integration uses documented hooks or wrappers and preserves headless behavior.
- **FR-094:** OpenCode integration uses documented server, SSE, CLI, or plugin surfaces and handles reconnect replay.
- **FR-095:** Structured and harness channels contain artifacts, references, and diagnostics only; terminal framing is confined to a terminal backend that owns the TTY.
- **FR-096:** Cancellation is propagated from CLI, RPC, and harness requests to queued and active render work; terminated work publishes no partial artifact.
- **FR-097:** Core and default CLI perform no telemetry, crash upload, update check, DNS, socket, or provider request.
- **FR-098:** Every release archive is installed into a clean target environment and must pass version, semantic render, exact-source, and redirected-output smoke fixtures.
- **FR-099:** Each release target publishes checksums, an SBOM, provenance, and signatures or attestations appropriate to that platform.
- **FR-100:** Public publication promotes the exact approved private candidate commit and artifact digests without rebuilding.

## 8. Non-functional requirements

- **NFR-001 Correctness:** Normative strict fixtures pass 100%; bytes outside accepted spans change 0 times.
- **NFR-002 Determinism:** Logical parse, source maps, semantic text, statuses, and pinned-CI artifacts are byte-identical across 100 repeated runs per fixture.
- **NFR-003 Safety:** Adversarial inputs cause no panic, abort, hang, terminal side effect, external resource access, or unbounded allocation.
- **NFR-004 Performance:** Proposed calibrated targets are p95 ≤10 ms for cached text, ≤25 ms for cached artifact lookup, and ≤200 ms for one simple cold strict render on the named reference host.
- **NFR-005 Memory:** Peak memory remains within calibrated limits and one hostile message cannot create unbounded worker fan-out.
- **NFR-006 Portability:** Logical behavior is tested on current stable macOS, Linux, and Windows. Pixel determinism is guaranteed only in the pinned CI renderer/font/backend environment.
- **NFR-007 Compatibility:** V1 real-terminal tests cover direct Kitty, direct iTerm2, WezTerm as the preferred graphics reference terminal, Apple Terminal text fallback, SSH/tmux/screen text fallback, pipes, files, and CI. A Sixel-capable emulator remains in the post-v1 protocol corpus.
- **NFR-008 Accessibility:** VoiceOver plus Apple Terminal, NVDA plus Windows Terminal and braille viewer/display, and Orca plus GNOME Terminal complete the release task set.
- **NFR-009 Privacy:** Core has no telemetry, background networking, crash upload, or update check. Repair networking can exist only in an explicit host provider.
- **NFR-010 Supply chain:** V1 has no unresolved High/Critical advisory, unknown/incompatible license, unaudited font, floating release dependency, or unpinned third-party release action.
- **NFR-011 Reproducibility:** Same-target candidate rebuilds produce matching unpacked payloads and checksums under the pinned toolchain.
- **NFR-012 Maintainability:** Public types use closed enums and stable diagnostics. Terminal protocols, renderer, parser, cache, CLI, and harness adapters remain separate modules.
- **NFR-013 Observability:** Diagnostics expose stable codes, timings, backend decisions, and limit classes without raw source or sensitive context by default.
- **NFR-014 Testability:** Parser, renderer, encoders, transports, cache, adapters, and release packaging each have independent fixtures plus end-to-end contract tests.
- **NFR-015 Fuzz resilience:** Required parser, source-map, terminal-frame, response-parser, image-metadata, and structured-event fuzz campaigns complete with zero crash, hang, or bound violation.
- **NFR-016 Concurrency:** Concurrent requests cannot share mutable source, terminal IDs, cache temp files, cancellation state, or adapter revisions.
- **NFR-017 API stability:** V1 public schemas and Rust APIs follow semantic versioning; breaking changes require a major version and migration notes.
- **NFR-018 Documentation:** Every supported command, format, backend, fallback, limit, privacy behavior, and compatibility claim has a tested example or fixture reference.
- **NFR-019 Release privacy:** Full history and release payloads pass secret and private-content scans before repository publication.
- **NFR-020 Installer safety:** Official installers are version-bound, checksum-verifying, user-local by default, non-privileged, and do not edit shell profiles without consent.

## 9. Compatibility matrices

### 9.1 Terminal destinations

| Destination | Universal result | Pixel plan | V1 claim gate |
|---|---|---|---|
| WezTerm | semantic + source | Kitty-compatible path with trusted attestation | Direct real-terminal conformance |
| Kitty | semantic + source | Kitty direct transfer with trusted attestation | Encoder, chunk, cleanup tests |
| iTerm2 | semantic + source | OSC 1337 inline PNG with trusted attestation | Encoder, size, terminator tests |
| Sixel-capable xterm | semantic + source | post-v1 | Text fallback in v1 |
| Apple Terminal | semantic + visual text | none assumed | Text, copy, VoiceOver tests |
| Windows Terminal | semantic + visual text | none required for v1 | Text, NVDA, braille tests |
| GNOME Terminal/VTE | semantic + visual text | none required for v1 | Text and Orca tests |
| SSH without PTY | deterministic semantic text | forbidden | Byte scan |
| SSH with PTY | semantic + source | post-v1 transport work | Text fallback in v1 |
| tmux | semantic + source | post-v1 transport work | Text fallback in v1 |
| GNU screen | semantic + source | disabled in v1 | Fallback matrix |
| pipe/file/CI/log/MCP | deterministic text or structured artifact | forbidden | Zero-control byte scan |

### 9.2 Harnesses

| Harness | First integration | Native path | Required fallback |
|---|---|---|---|
| OMP | yes | extension post-turn preview, then generic core assistant-content renderer | custom message/widget, artifact reference, or semantic text |
| Codex | after OMP | documented structured/noninteractive events | sidecar preview or artifact reference |
| Claude Code | after Codex | documented hook/structured output | post-turn artifact or semantic text |
| OpenCode | after Claude | documented server/plugin events | post-turn artifact or semantic text |

## 10. Success metrics and release gates

BeaverMath collects no production telemetry. Metrics come from versioned, redistributable fixtures and private CI.

| Metric | Blocking v1 target |
|---|---:|
| Strict parser fixture pass rate | 100% |
| Bytes changed outside accepted spans | 0 |
| Undelimited expressions selected in strict negative corpus | 0 |
| Exact source and SHA-256 recovery on all success/failure paths | 100% |
| Parsed semantic nodes omitted without a decorative marker | 0 |
| Empty alt text on a visual artifact | 0 |
| Terminal controls in pipe/file/log/structured captures | 0 |
| Terminal side effects from adversarial source | 0 |
| Runtime network attempts in core corpus | 0 |
| Panics, aborts, hangs, arithmetic wraps, or unbounded allocations in required fuzzing | 0 |
| Required terminal, transport, accessibility, and adapter matrix cells | 100% pass |
| High/Critical unmitigated dependency advisories | 0 |
| Unknown or incompatible dependency/font/asset licenses | 0 |
| Candidate median performance regression from frozen baseline | ≤10% |
| Candidate p95 regression from frozen baseline | ≤15% |
| Same-target reproducibility mismatches | 0 |
| Release archives that fail install and smoke fixtures | 0 |

## 11. Non-goals

- Full TeX or LaTeX document compilation.
- Shell escape, packages, user macros, file inclusion, URLs, bibliography, fonts loaded from arbitrary paths, or external processes.
- Screen scraping or rewriting another harness's terminal buffer.
- Silent or default model-assisted rewriting of arbitrary prose.
- Mutation of provider messages, canonical transcripts, session history, or copied source.
- Universal pixel output in terminals without a supported graphics path.
- Graphics in redirected output, logs, CI auto mode, or structured protocol stdout.
- A new terminal emulator, full-screen interactive viewer, editor, notebook, browser UI, or equation authoring environment in v1.
- PDF output in v1.
- Remote cache, cloud account, telemetry, usage analytics, update checks, or crash uploads.
- Pixel-byte identity across unrelated terminal emulators, font stacks, GPUs, and operating systems.
- Automatic support for GNU screen or arbitrary nested multiplexer topologies in v1.
- Declaring inferred ASCII/Unicode meaning authoritative.
- Active terminal capability queries in supported v1 builds.
- Sixel, GNU screen graphics, SSH graphics, tmux graphics, or nested-multiplexer graphics in supported v1 builds.

## 12. Open decisions

None block repository initialization. These decisions must be closed by the named roadmap gates:

1. **RaTeX support envelope:** exact supported commands, fonts, output stability, and sandbox requirements, decided after the Phase 2 corpus spike.
2. **Visual Unicode backend:** adopt an existing crate or implement from BeaverMath IR, decided after semantic and width-profile fixtures exist.
3. **Post-v1 Sixel scheduling:** implementation begins only after v1 publication and uses the existing protocol threat model and conformance corpus.
4. **OMP core hook upstreaming:** upstream contribution, maintained patch, or external wrapper, decided after the extension-only preview proves the artifact and lifecycle contract.
5. **MCP command in v1:** include only if it adds a real structured consumer beyond the harness adapters and preserves strict stdout framing.
6. **Repair provider:** first host-owned model provider and quality corpus, decided after strict rendering reaches release-candidate quality.

## 13. Primary references

- [RaTeX](https://github.com/erweixin/RaTeX)
- [Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
- [iTerm2 inline images](https://iterm2.com/documentation-images.html)
- [WezTerm `imgcat`](https://wezterm.org/imgcat.html)
- [ratatui-image](https://github.com/ratatui/ratatui-image)
- [ECMA-48 terminal control functions](https://ecma-international.org/publications-and-standards/standards/ecma-48/)
- [WCAG 2.2](https://www.w3.org/TR/WCAG22/)
- [WCAG2ICT terminal guidance](https://www.w3.org/TR/wcag2ict-22/#text-command-line-terminal-applications-and-interfaces)
- [MathML 4](https://w3c.github.io/mathml/)
- [OMP extension guide](https://raw.githubusercontent.com/can1357/oh-my-pi/main/docs/extensions.md)
- [Codex noninteractive mode](https://developers.openai.com/codex/noninteractive)
- [Claude Code hooks](https://code.claude.com/docs/en/hooks)
- [OpenCode CLI](https://opencode.ai/docs/cli/)
- [OpenCode plugins](https://opencode.ai/docs/plugins/)
