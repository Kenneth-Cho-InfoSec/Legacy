#!/usr/bin/env bash
set -euo pipefail

test -d docs
test -d scripts
test -d firmware-dumps/private-not-for-redistribution
test -f legal/firmware-handling.md

bash scripts/collect_wsl_tool_inventory.sh analysis/tool-inventory-wsl.txt >/dev/null
bash scripts/hash_dumps.sh firmware-dumps/private-not-for-redistribution analysis/firmware-dump-hashes.tsv >/dev/null

if git check-ignore -q firmware-dumps/private-not-for-redistribution/example.bin; then
  echo "Private firmware ignore rule: ok"
else
  echo "Private firmware ignore rule failed" >&2
  exit 1
fi

echo "Smoke tests passed"

