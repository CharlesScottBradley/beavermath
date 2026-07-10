# OMP message_end spike (O0)

**Host:** `omp` v16.3.15 (`~/.bun/bin/omp`)  
**SDK types:** `@oh-my-pi/pi-coding-agent@16.3.15`  
**Date:** 2026-07-10

## Event: `message_end`

```ts
interface MessageEndEvent {
  type: "message_end";
  message: AgentMessage; // from @oh-my-pi/pi-agent-core
}
```

Also: `message_start`, `message_update` (streaming - paint **source** only).

## Extension API used

| API | Role |
|-----|------|
| `pi.on("message_end", …)` | Present after complete assistant message |
| `pi.sendMessage({ customType, content, display, details })` | In-session preview (O2) |
| `pi.registerMessageRenderer("beavermath.math-preview", …)` | Render custom preview messages |
| `pi.appendEntry` | Optional persistence of presentation metadata (not LLM) |

## Assistant body content renderer

**Not present** in ExtensionAPI. No `registerAssistantContentRenderer`.  
O3 true inline = **upstream gap**. O2 uses custom message + display field.

## CustomMessagePayload (host-verified types)

```ts
interface CustomMessage {
  role: "custom";
  customType: string;
  content: string | (TextContent | ImageContent)[];  // what can be shown
  display: boolean;  // show/hide in transcript - NOT presentation text
  details?: T;
}
type MessageRenderer = (message, options, theme) => Component | undefined;
// Component: { render(width: number): readonly string[] }
```

**Critical:** `display` is a **boolean**. Non-boolean values coerce to hide the message.  
Readable math must live in **`content`** (and/or be painted by a Component renderer).

## Extension discovery

`extensions/*.ts|js`, `extensions/<name>/index.ts|js`, or package.json with `"omp"` / `"pi"` field.  
Configure via OMP `settings.extensions` paths or CLI `-e`.

## Fixture

`tests/fixtures/message_end.json` - synthetic MessageEndEvent for unit tests without live UI.
