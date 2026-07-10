#!/usr/bin/env bash
# OpenCode oneshot filter wrapper (same pattern as Codex).
set -uo pipefail
if [[ "${1:-}" != "--" ]]; then
  echo "usage: oneshot.sh -- <producer> [args...]" >&2
  exit 2
fi
shift
tmp="$(mktemp)"
set +e
"$@" >"$tmp" 2> >(cat >&2)
prod=$?
set -e
beavermath filter --file "$tmp"
filt=$?
rm -f "$tmp"
[[ $filt -ne 0 ]] && exit "$filt"
exit "$prod"
