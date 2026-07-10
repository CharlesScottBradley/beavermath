#!/usr/bin/env bash
# Codex-style oneshot: run a producer command, filter stdout, preserve producer exit status.
# usage: oneshot.sh -- <producer> [args...]
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
if [[ $filt -ne 0 ]]; then
  exit "$filt"
fi
exit "$prod"
