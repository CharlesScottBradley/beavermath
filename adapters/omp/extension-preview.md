# OMP live extension path (H3)

**Status:** implementation recipe (live beyond pure Rust fixtures)

## Mechanism

1. OMP extension listens for completed assistant message / turn-end events (documented `pi.on(...)` surfaces).
2. Extension sends HookRequestV1 to `beavermath hook` via subprocess (isolation; no ESC on extension machine channel):

```ts
// conceptual
const req = {
  kind: "message_end",
  message_id: msg.id,
  revision: msg.revision ?? 1,
  content: msg.content, // exact UTF-8
  write_preview: true,
  session_id: session.id,
};
const { stdout } = await execFile("beavermath", ["hook"], { input: JSON.stringify(req) });
const event = JSON.parse(stdout);
// Supplemental preview only - do not mutate msg.content
await pi.sendMessage({
  role: "custom",
  content: event.semantic_preview ?? "",
  meta: { beavermath: true, source_sha256: event.content_sha256 },
});
```

3. Canonical `msg.content` remains exact LaTeX.

## Limitation (not H7)

Documented OMP APIs favor **supplemental** custom messages/widgets, not arbitrary replacement of assistant-content spans. True inline is H7 / BM-022.

## Verification

```bash
printf '%s' '{"kind":"message_end","message_id":"omp1","revision":1,"content":"From Maxwell $E=mc^2$."}' \
  | beavermath hook | python3 -c 'import json,sys; e=json.load(sys.stdin); assert "power" in e["semantic_preview"] or "E" in e["semantic_preview"]; assert "$E=mc^2$" in e["exact_source"]'
```

This binary path is the same process OMP will call; host packaging is the remaining install step on a machine with OMP.
