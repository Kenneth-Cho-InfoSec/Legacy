#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <firmware-dump>" >&2
  exit 1
fi

fw="$1"
if [[ ! -f "$fw" ]]; then
  echo "Firmware file not found: $fw" >&2
  exit 1
fi

base="$(basename "$fw")"
safe="${base//[^A-Za-z0-9_.-]/_}"
out="analysis/firmware-triage/${safe}"
mkdir -p "$out"

sha256sum "$fw" > "$out/sha256.txt"
stat "$fw" > "$out/stat.txt"
file "$fw" > "$out/file.txt" || true
xxd -g 1 -l 1024 "$fw" > "$out/head-1k.hex"
hexdump -C "$fw" | head -n 256 > "$out/hexdump-head.txt"
strings -a -n 6 "$fw" > "$out/strings.txt" || true

if command -v binwalk >/dev/null 2>&1; then
  binwalk "$fw" > "$out/binwalk.txt" || true
  binwalk -E "$fw" > "$out/binwalk-entropy.txt" || true
else
  echo "binwalk missing" > "$out/binwalk.txt"
fi

echo "Wrote $out"

