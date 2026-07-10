# Grok Build adapter (lab first-class)

**Stock TUI paint:** blocked (closed binary). See [BLOCKER.txt](BLOCKER.txt), [PATCH_PROPOSAL.md](PATCH_PROPOSAL.md).  
**Shipped interim viewport:** session projection (source untouched).  
**Spike / G2a:** [SPIKE.md](SPIKE.md) · **In-chat design:** [INCHAT.md](INCHAT.md)

## Install

```bash
REPO_ROOT="$(git rev-parse --show-toplevel)"
cargo install --path "$REPO_ROOT/crates/beavermath-cli" --locked --force
mkdir -p ~/.grok/hooks
# Generate a machine-local hook (do not commit generated absolute paths)
sed "s|@BEAVERMATH_ROOT@|$REPO_ROOT|g"   "$REPO_ROOT/adapters/grok-build/hooks/beavermath-stop.json.template"   > ~/.grok/hooks/beavermath-stop.json
chmod +x "$REPO_ROOT/adapters/grok-build/scripts/"*
```

## After a Grok turn: readable math without touching the model transcript

```bash
# Latest session for this directory
beavermath grok-session --cwd "$PWD"
# → prints .../beavermath_display/VIEWPORT.md

# Or pin a session
beavermath grok-session --session-dir ~/.grok/sessions/<encoded-cwd>/<session-id>

# Last assistant only as JSON
beavermath grok-session --session-dir ... --last-only | python3 -m json.tool | head
```

Open `VIEWPORT.md` in a split pane (`less`, editor, Paper Pro, etc.).  
`chat_history.jsonl` still holds exact `$...$` for the model.

Stop hooks also run `grok-session` (fail-open) after turns.

## Limits

| Goal | Status |
|------|--------|
| Math readable without hand-running filter | **Yes** via VIEWPORT.md / present |
| Math inside stock Grok scrollback | **No** until xAI paint API / patch |
| Mutate history to semantic | **Never** (would poison the model) |

## BM-039

Present + session projection + hooks; source preserved; no TUI scrape.
