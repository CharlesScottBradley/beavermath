# OpenCode plugin spike (C0)

**Host:** `opencode` via `~/.opencode/bin/opencode`  
**Plugin package:** `@opencode-ai/plugin@1.17.7`  
**Date:** 2026-07-10

## Plugin shape

```ts
export type Plugin = (input: PluginInput, options?) => Promise<Hooks>;
```

`PluginInput` provides `client`, `project`, `directory`, `$` shell, etc.

## Hooks relevant to math presentation

| Hook | Role | Mutates store? |
|------|------|----------------|
| `experimental.text.complete` | Fires when a text part completes; `output.text` can be modified | **Likely yes** - treat as unsafe for source immutability by default |
| `experimental.chat.messages.transform` | Transform messages for LLM | Must not inject semantic (would poison context) |
| `event` | Generic bus | Observation only |
| `chat.message` | User message path | Not assistant complete |

## C2 decision

**No documented display-only assistant bubble re-render API.**  
Rewriting `experimental.text.complete` `output.text` risks storing semantic as source → **violates product bar**.

**C2 status: API-blocked** for true in-bubble paint without source mutation.

**C1 shipped:** plugin presents on `experimental.text.complete`, writes presentation to plugin data dir, does **not** rewrite `output.text` (unless `BEAVERMATH_OPENCODE_REWRITE_TEXT=1` opt-in experimental).

## Install

```bash
# From repo: file: plugin path in opencode.json
# or opencode plugin <path> -- when supported for local modules
```

Config `~/.config/opencode/opencode.json` `plugin` array.

## Oneshot (print bar)

`adapters/opencode/oneshot.sh` remains for non-interactive.
