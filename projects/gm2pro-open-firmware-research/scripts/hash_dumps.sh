#!/usr/bin/env bash
set -euo pipefail

dir="${1:-firmware-dumps/private-not-for-redistribution}"
manifest="${2:-analysis/firmware-dump-hashes.tsv}"

if [[ ! -d "$dir" ]]; then
  echo "Directory not found: $dir" >&2
  exit 1
fi

mkdir -p "$(dirname "$manifest")"
printf "sha256\tsize_bytes\tpath\n" > "$manifest"

find "$dir" -type f ! -name ".gitkeep" -print0 | sort -z | while IFS= read -r -d '' file; do
  sha="$(sha256sum "$file" | awk '{print $1}')"
  size="$(stat -c '%s' "$file")"
  printf "%s\t%s\t%s\n" "$sha" "$size" "$file" >> "$manifest"
done

echo "Wrote $manifest"

