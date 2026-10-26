#!/usr/bin/env bash
set -euo pipefail

out="${1:-analysis/tool-inventory-wsl.txt}"
mkdir -p "$(dirname "$out")"

{
  echo "Generated: $(date -Is)"
  echo
  echo "== System =="
  uname -a
  if command -v lsb_release >/dev/null 2>&1; then
    lsb_release -a
  else
    cat /etc/os-release
  fi
  echo
  echo "== Tools =="
  for tool in \
    bluetoothctl btmon sdptool hcitool gatttool tshark wireshark binwalk \
    strings file xxd hexdump sha256sum ghidra rizin r2 arm-none-eabi-objdump \
    openocd pyocd flashrom sigrok-cli pulseview lsusb hciconfig rfkill python3 pip3; do
    printf '%-24s' "$tool"
    command -v "$tool" || echo "missing"
  done
} > "$out"

echo "Wrote $out"

