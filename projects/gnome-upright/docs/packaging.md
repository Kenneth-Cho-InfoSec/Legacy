# Packaging

Build with Meson and install with `meson install -C build`. The post-install hook compiles the GSettings schema and icon cache. Distribution packages should depend on GTK4, libadwaita, PyGObject, FluidSynth, and the optional Python MIDI packages. The Flatpak manifest provides the desktop runtime permissions needed for audio and MIDI device access.
