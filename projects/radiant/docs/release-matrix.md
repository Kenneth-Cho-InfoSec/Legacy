# Radiant release matrix

## Linux runtime

| Component | Supported baseline | Verification |
|---|---|---|
| Linux | Ubuntu 24.04 / Fedora 40 class distributions | CI build and test job |
| Desktop | GNOME Shell with GTK4/libadwaita | GTK4 application startup smoke test |
| Audio | PipeWire 1.x (primary), JACK 2 (fallback) | `diagnose_backends()` plus realtime tests |
| MIDI | ALSA Sequencer | MIDI timestamp and queue tests |
| Display | Wayland or X11 | Manual desktop smoke test |

## Native plugin compatibility

| Format | Linux support | Isolation | Notes |
|---|---|---|---|
| LV2 | Native discovery and processing | Worker boundary available | URI/state round-trip required |
| CLAP | Native discovery and processing | Worker boundary available | Descriptor and parameter checks required |
| VST3 | Native discovery and processing | Worker boundary available | Bundle and ABI checks required |

Plugins are never loaded through Wine. A missing or crashing plugin is
quarantined and must not prevent project recovery.

## Release gate

Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, and `cargo test --workspace`. Hardware jobs should additionally
exercise a live PipeWire stream, ALSA MIDI, forced-termination recovery, and
GTK accessibility/scale-factor smoke tests.
