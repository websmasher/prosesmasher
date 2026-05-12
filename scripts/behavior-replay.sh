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

"$TEXTLINT" \
  --no-textlintrc \
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/orthography" \
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/words" \
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/phrases" \
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/closers" \
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/contrast" \
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/lead-ins" \
  --rulesdir "$ROOT/packages/textlint-rules/dist/families/syntactic-patterns/repetition" \
  --format json \
  "${FILES[@]}"
