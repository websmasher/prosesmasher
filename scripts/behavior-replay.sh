#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEXTLINT="$ROOT/packages/textlint-rules/node_modules/.bin/textlint"

cd "$ROOT/packages/textlint-rules"
npm run build >/dev/null

cd "$ROOT"

if [ "$#" -eq 0 ]; then
  mapfile -t FILES < <(find behavior/fixtures/textlint-rules -name family.md | sort)
else
  FILES=("$@")
fi

CONFIG_ARGS=(--no-textlintrc)
RULE_ARGS=(
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/metrics"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/orthography"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/words"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/phrases"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/term-policy"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/authority"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/closers"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/contrast"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/generalization"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/lead-ins"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/llm-artifacts"
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/repetition"
)
if [ "$#" -eq 1 ]; then
  FIXTURE_CONFIG="$(dirname "$1")/.textlintrc.json"
  if [ -f "$FIXTURE_CONFIG" ]; then
    FAMILY="$(basename "$(dirname "$1")")"
    CONFIG_ARGS=(--config "$FIXTURE_CONFIG" --rules-base-directory "$ROOT/packages/textlint-rules/dist/families/$FAMILY")
    RULE_ARGS=()
  fi
fi

"$TEXTLINT" \
  "${CONFIG_ARGS[@]}" \
  "${RULE_ARGS[@]}" \
  --format json \
  "${FILES[@]}"
