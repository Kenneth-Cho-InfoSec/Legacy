#!/bin/sh
set -eu
meson setup build --reconfigure
meson compile -C build
interpreter="python3"
if [ -x "$PWD/.venv/bin/python" ]; then
  interpreter="$PWD/.venv/bin/python"
fi
GSETTINGS_SCHEMA_DIR="$PWD/build/data" PYTHONPATH="$PWD/src" UPRIGHT_SOUNDFONT="${UPRIGHT_SOUNDFONT:-/usr/share/soundfonts/FluidR3_GM.sf2}" "$interpreter" src/main.py
