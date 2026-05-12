#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASELINE_DIR="$ROOT/behavior/baselines/textlint-rules"
mkdir -p "$BASELINE_DIR"

for fixture in $(find "$ROOT/behavior/fixtures/textlint-rules" -name family.md | sort); do
  family="$(basename "$(dirname "$fixture")")"
  output="$BASELINE_DIR/$family.json"
  status=0
  "$ROOT/scripts/behavior-replay.sh" "$fixture" >"$output" || status="$?"

  if [ "$status" -gt 1 ]; then
    exit "$status"
  fi
done
