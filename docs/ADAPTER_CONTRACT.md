# AdapterEventV1 contract (frozen for harness track)

**Schema version:** `1`  
**Machine rule:** stdout JSON for hooks/RPC **must not** contain ESC (`0x1B`), CSI/OSC/DCS/APC graphics payloads.  
**Source rule:** exact UTF-8 assistant content is immutable; semantic and pixels are presentation only.

## AdapterEventV1 fields

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `schema_version` | string | yes | `"1"` |
| `kind` | enum | yes | see EventKind |
| `message_id` | string | yes | host-stable id |
| `revision` | u64 | yes | bumps on replace |
| `content_sha256` | string | yes | SHA-256 hex of exact content (or empty digest on cancel/delete without body) |
| `artifacts` | MathArtifactV1[] | yes (may be empty) | per-expression universal artifacts |
| `semantic_preview` | string? | no | whole-document filtered semantic text |
| `exact_source` | string? | no | optional echo of exact content for resume tooling |
| `preview_path` | string? | no | sidecar file path when G2a writes preview |

## EventKind

`message_start` | `message_update` | `message_end` | `turn_end` | `branch` | `resume` | `session_switch` | `shutdown` | `cancel` | `replace` | `delete`

## Hook request (stdin JSON)

```json
{
  "schema_version": "1",
  "kind": "message_end",
  "message_id": "m1",
  "revision": 1,
  "content": "exact assistant utf-8 with $x^2$",
  "session_id": "optional",
  "write_preview": false,
  "preview_dir": null
}
```

| Field | Required | Notes |
|-------|----------|-------|
| `kind` | yes | maps to store action |
| `message_id` | yes | |
| `revision` | for end/replace | default 0 |
| `content` | for end/replace/resume rebuild | exact UTF-8 |
| `write_preview` | no | if true, write sidecar under preview_dir |
| `preview_dir` | no | default `~/.beavermath/previews` |

## Idempotency

Key: `(message_id, revision, content_sha256)`. Duplicate `message_end` with same key returns equivalent event.

## ExitStatus (hook binary)

| Code | Meaning |
|------|---------|
| 0 | Success |
| 2 | Usage / bad JSON |
| 3 | Invalid input (UTF-8) |
| 4 | Parse/render issues (event still may be emitted with diagnostics path) |
| 1 | Internal |

## Invariants

1. `exact_source` recovered from store always hashes to `content_sha256`.  
2. Chat paths use `auto_protect=true`.  
3. Delete clears resume; no stale preview.  
4. Cancel marks preview inactive; no partial artifact publish.  
5. Hooks fail-open at the host; BeaverMath returns structured errors on stderr.
