#!/usr/bin/env sh
set -eu

status=0

for layer in 1 2 3 4 5; do
  if scripts/verify-textlint-implementation-manifest.py "$layer"; then
    true
  else
    status=1
  fi
done

exit "$status"
