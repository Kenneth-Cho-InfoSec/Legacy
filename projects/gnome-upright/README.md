# GNOME Upright

GNOME Upright is a native GTK4/libadwaita virtual piano and MIDI workstation. It offers a 25-key C3–C5 keyboard, computer-keyboard and touch input, MIDI input, velocity and sustain, recording, responsive playback, quantization, a metronome, and Standard MIDI File import/export.

## Build and run

Install GTK4, libadwaita, PyGObject, FluidSynth, a General MIDI SoundFont, and the Python packages listed in `pyproject.toml`. Then:

```sh
meson setup build
meson compile -C build
GSETTINGS_SCHEMA_DIR=build/data src/main.py
```

For a development sound font, set `UPRIGHT_SOUNDFONT=/path/to/FluidR3_GM.sf2`. Audio and MIDI capabilities are surfaced clearly when their optional system dependencies are unavailable. See `docs/packaging.md` for Flatpak use.

## Architecture

The application separates rendering (`widgets`), transport/session UI (`window`), MIDI device I/O (`midi`), synthesis (`audio`), timeline recording/playback (`recording`), and file formats (`formats`). All timing is monotonic and note events carry a source identifier, allowing chords and multiple simultaneous input devices.

## License

MIT. No SoundFont is redistributed; install a suitably licensed one on the system.
