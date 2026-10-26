#!/usr/bin/env bash
# MaterialPipe screenshot harness.
#
# Captures key screens from a fresh install in a deterministic state (light
# theme, dynamic color off) and, when baselines exist, fails on visual
# regression. Mirrors the on-device verification used during M1/M2.
#
# Usage:
#   tools/screenshot_harness.sh <debug-apk> [capture-dir]
#
# Examples:
#   tools/screenshot_harness.sh app/build/outputs/apk/debug/PipePipe_*-debug.apk
#   BASELINE=1 tools/screenshot_harness.sh <apk>   # capture baselines
#
# Requirements: adb on PATH, one emulator/device connected, app_id set below.

set -euo pipefail

APK="${1:?usage: screenshot_harness.sh <debug-apk> [capture-dir]}"
CAPTURE_DIR="${2:-tools/screenshots/current}"
BASELINE_DIR="tools/screenshots/baseline"
APP_ID="org.materialpipe.app.debug"
PKG="org.schabi.newpipe"

: "${BASELINE:=0}"

command -v adb >/dev/null 2>&1 || { echo "adb not found on PATH" >&2; exit 1; }
adb get-state >/dev/null 2>&1 || { echo "no device/emulator connected" >&2; exit 1; }

mkdir -p "$CAPTURE_DIR"

echo "==> Installing $APK"
adb install -r "$APK" >/dev/null

echo "==> Resetting to deterministic state (light theme, no dynamic color)"
printf '%s' '<?xml version="1.0" encoding="utf-8"?><map><string name="theme">light_theme</string><boolean name="key_theme_change" value="false"/><string name="night_theme">dark_theme</string><boolean name="dynamic_color" value="false"/></map>' > /tmp/mp_prefs.xml
adb push /tmp/mp_prefs.xml /data/local/tmp/mp_prefs.xml >/dev/null
adb shell run-as "$APP_ID" cp /data/local/tmp/mp_prefs.xml shared_prefs/org.materialpipe.app.debug_preferences.xml 2>/dev/null || true

echo "==> Launching app"
adb shell am force-stop "$APP_ID" >/dev/null 2>&1 || true
adb shell am start -n "$APP_ID/$PKG.MainActivity" >/dev/null 2>&1
sleep 6

capture() {
  local name="$1"; shift
  adb exec-out screencap -p > "$CAPTURE_DIR/$name.png"
  echo "    captured $name.png"
}

echo "==> Capturing screens"
capture "home"
adb shell input tap 1017 136 2>/dev/null || true
sleep 2
capture "search"
adb shell input keyevent KEYCODE_BACK
sleep 1

echo "==> Comparing against baselines"
if [ "$BASELINE" = "1" ]; then
  rm -rf "$BASELINE_DIR"
  cp -r "$CAPTURE_DIR" "$BASELINE_DIR"
  echo "    baselines written to $BASELINE_DIR"
  exit 0
fi

if [ ! -d "$BASELINE_DIR" ]; then
  echo "    no baselines present; run with BASELINE=1 to create them"
  exit 0
fi

FAIL=0
for f in "$CAPTURE_DIR"/*.png; do
  name="$(basename "$f")"
  base="$BASELINE_DIR/$name"
  if [ ! -f "$base" ]; then
    echo "    WARN: no baseline for $name"
    continue
  fi
  if python3 tools/compare_screenshots.py "$base" "$f"; then
    echo "    OK: $name matches baseline"
  else
    echo "    REGRESSION: $name differs from baseline"
    FAIL=1
  fi
done

echo "==> Done"
exit $FAIL
