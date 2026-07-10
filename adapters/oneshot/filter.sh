#!/usr/bin/env bash
# Read stdin (or file arg), run beavermath filter, preserve failure codes from beavermath only.
set -euo pipefail
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  echo "usage: filter.sh [file]   # default: stdin"
  exit 0
fi
if [[ $# -ge 1 ]]; then
  exec beavermath filter --file "$1"
else
  exec beavermath filter
fi
