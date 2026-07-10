# OpenCode in-chat presentation

**Status:** **C0-C1 live (plugin)** · **C2 API-blocked** for safe in-bubble paint  
**Oneshot print:** [oneshot.sh](oneshot.sh) (proven)

## Ladder

| Step | Status |
|------|--------|
| C0 SPIKE | [SPIKE.md](SPIKE.md) |
| C1 present on `experimental.text.complete` | [plugin/index.js](plugin/index.js) - writes presentation JSON under `.beavermath-opencode/`; does **not** rewrite `output.text` by default |
| C2 bubble paint | **Blocked** - rewriting `output.text` risks poisoning session source; no display-only paint API |

## Install

Add a local plugin path to OpenCode config `plugin` array (see [plugin.example.json](plugin.example.json)). Do **not** commit API keys.

```bash
# Generate an absolute file URI at install time (hosts usually need absolute file:// paths):
# export BEAVERMATH_ROOT="$(git rev-parse --show-toplevel)"
# Expand into config, e.g. plugin: ["file://$BEAVERMATH_ROOT/adapters/opencode/plugin"]
# Or use a host-supported relative path per OpenCode docs.

opencode  # interactive
```

Optional experimental (may store semantic as source - **off by default**):

```bash
BEAVERMATH_OPENCODE_REWRITE_TEXT=1 opencode
```

## Tests

```bash
node adapters/opencode/tests/present-integration.mjs
bash adapters/opencode/oneshot.sh -- printf 'Open $a^2$\n'
```
