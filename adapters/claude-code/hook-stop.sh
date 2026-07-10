#!/usr/bin/env bash
# Claude Code / generic Stop-style hook: stdin JSON host payload → beavermath hook.
# Maps common fields; fail-open (exit 0) if content missing so host is not blocked.
set -uo pipefail
payload="$(cat)"
# Prefer known content keys; empty → no-op success
content="$(printf '%s' "$payload" | beavermath hook 2>/dev/null && true)"
# If payload is not already HookRequestV1, map via jq-less python
python3 - <<'PY' 2>/dev/null || true
import json, os, sys, subprocess
raw = sys.stdin.read() if False else open(0).read() if False else None
PY
# Robust mapping:
mapped="$(python3 -c '
import json,sys
try:
    d=json.load(sys.stdin)
except Exception:
    sys.exit(0)
content = d.get("content") or d.get("assistantMessage") or d.get("text") or d.get("lastAssistantMessage") or ""
if not content and isinstance(d.get("messages"), list):
    for m in reversed(d["messages"]):
        if m.get("role") in ("assistant","model") and m.get("content"):
            content = m["content"] if isinstance(m["content"], str) else json.dumps(m["content"])
            break
if not content:
    sys.exit(0)
req={
  "kind":"message_end",
  "message_id": str(d.get("message_id") or d.get("messageId") or d.get("sessionId") or "turn"),
  "revision": int(d.get("revision") or 1),
  "content": content if isinstance(content,str) else str(content),
  "session_id": d.get("sessionId") or d.get("session_id"),
  "write_preview": True,
}
print(json.dumps(req))
' <<<"$payload" 2>/dev/null || true)"

if [[ -z "${mapped}" ]]; then
  exit 0
fi
# shellcheck disable=SC2086
printf '%s' "$mapped" | beavermath hook --store-dir "${HOME}/.beavermath/store" || exit 0
exit 0
