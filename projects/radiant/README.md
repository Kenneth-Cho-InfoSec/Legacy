# Radiant

Radiant is a clean-room, Linux-first digital audio workstation written in
Rust with GTK4-rs. Its interaction model is pattern-first: the Channel Rack
owns sound sources, reusable Patterns own musical events, the Playlist hosts
mixed clip types, and Mixer inserts remain explicitly routed rather than
being tied to Playlist lanes.

This repository contains a persistent editing foundation and a deterministic
offline audio engine. It currently contains:

- the core session model and stable object identifiers;
- transactional, reversible session commands;
- schema-versioned SQLite project storage;
- an application controller for history, dirty state, selection, presentation
  state, and explicit save/open boundaries;
- a responsive GTK4/libadwaita workspace shell that follows the system
  appearance and contains a Browser, Channel Rack, Picker, Playlist, Mixer,
  and separate Piano Roll editor;
- native project lifecycle actions, schema-v2 project/presentation storage,
  migration, dirty-state protection, and coalesced undo/redo.
- UI-independent engine, graph, and media crates with immutable render plans,
  musical transport, a built-in test instrument, metronome, Mixer routing,
  WAV/FLAC decoding, resampling, and offline WAV export.
- native real-time playback with a PipeWire-first stream, JACK fallback, ALSA
  Sequencer MIDI input, lock-free control queues, meters, xrun/deadline
  diagnostics, and recoverable stream restart.

Radiant's representative workflow is durable: Channel Rack steps, Piano Roll
notes and transforms, Playlist audio/pattern/automation editing, Mixer routes,
recording takes, comping, automation, native plugin hosting, and project
recovery all use reversible or immutable representations and survive Save/Open.
Offline and real-time rendering share deterministic plans and native Linux
services. Public-alpha hardware and accessibility jobs remain release checks.

## Build

Install Rust 1.92 or newer plus GTK 4.14, PipeWire, JACK, and ALSA development
packages, then run:

```sh
cargo test --workspace
cargo run -p radiant-ui
```

On Debian/Ubuntu, the GTK dependency is normally provided by
`libgtk-4-dev`. On Fedora it is normally `gtk4-devel`.

## Design principles

- Native Linux implementation; no compatibility layer.
- GTK never runs on the real-time audio thread.
- The interface uses native GTK4/libadwaita widgets and semantic style
  classes instead of an application stylesheet.
- Playlist lanes can contain Pattern, Audio, and Automation Clips.
- Channels, Playlist lanes, and Mixer inserts have independent identities.
- All session edits pass through reversible commands.
- Original Radiant branding and assets; no Image-Line code or content.

See [the architecture overview](docs/architecture/overview.md),
[implementation status](docs/implementation-status.md), and the
[audited implementation roadmap](docs/roadmap.md).

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
