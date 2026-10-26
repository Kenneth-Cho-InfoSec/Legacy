#!/bin/sh
set -eu
meson setup build --reconfigure
meson compile -C build
