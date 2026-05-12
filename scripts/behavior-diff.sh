#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASELINE_DIR="$ROOT/behavior/baselines/textlint-rules"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

for fixture in $(find "$ROOT/behavior/fixtures/textlint-rules" -name family.md | sort); do
  family="$(basename "$(dirname "$fixture")")"
  baseline="$BASELINE_DIR/$family.json"
  current="$TMP_DIR/$family.current.json"

  if [ ! -f "$baseline" ]; then
    echo "Missing baseline: $baseline" >&2
    exit 1
  fi

  status=0
  "$ROOT/scripts/behavior-replay.sh" "$fixture" >"$current" || status="$?"

  if [ "$status" -gt 1 ]; then
    exit "$status"
  fi

  diff -u "$baseline" "$current"
done
