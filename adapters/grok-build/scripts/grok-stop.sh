#!/usr/bin/env bash
# Grok Stop/SubagentStop → present path.
# 1) Map host JSON when it carries assistant text
# 2) Else project latest session for CWD via beavermath grok-session (never mutates chat_history)
# Always exit 0 (fail-open).
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MAPPER="$ROOT/scripts/map_stop_payload.py"
BM="${BEAVERMATH_BIN:-beavermath}"
STORE="${BEAVERMATH_STORE:-$HOME/.beavermath/store}"
PREVIEWS="${BEAVERMATH_PREVIEWS:-$HOME/.beavermath/previews}"
CWD="${GROK_WORKSPACE:-${PWD:-.}}"

payload="$(cat || true)"
mkdir -p "$HOME/.beavermath/spikes" 2>/dev/null || true
ts="$(date +%Y%m%dT%H%M%S 2>/dev/null || echo now)"
printf '%s' "$payload" >"$HOME/.beavermath/spikes/stop-$ts.json" 2>/dev/null || true

mapped=""
if [[ -n "$payload" ]]; then
  mapped="$(printf '%s' "$payload" | python3 "$MAPPER" 2>/dev/null || true)"
fi

if [[ -n "$mapped" ]]; then
  mapped="$(printf '%s' "$mapped" | python3 -c '
import json,sys,os
d=json.load(sys.stdin)
d["write_preview"]=True
d.setdefault("preview_dir", os.path.expanduser(os.environ.get("BEAVERMATH_PREVIEWS","~/.beavermath/previews")))
print(json.dumps(d))
' 2>/dev/null || printf '%s' "$mapped")"
  printf '%s' "$mapped" | "$BM" hook --store-dir "$STORE" >/dev/null 2>>"$HOME/.beavermath/spikes/hook-stderr.log" || true
fi

# Session projection (display-only files next to session)
"$BM" grok-session --cwd "$CWD" >>"$HOME/.beavermath/spikes/hook-stderr.log" 2>&1 || true
exit 0
