# Codex in-chat (IP4)

**Status:** **oneshot-only** for interactive product bar  
**Reason:** no Codex interactive TUI display-override integration in this repo  
**Oneshot (live):** [oneshot.sh](oneshot.sh) preserves producer exit status; proven via binary `beavermath filter`

```bash
adapters/codex/oneshot.sh -- printf 'Codex result $E=mc^2$\n'
```

**Present readiness:** when Codex app-server exposes completed message content, call:

```bash
beavermath present --mode semantic
```

and paint `display_text` only.
