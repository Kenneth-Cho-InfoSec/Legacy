# Implementation status

Radiant is a Linux-first public-alpha implementation with deterministic offline
and native real-time paths. Release hardware and accessibility jobs remain
environment-specific verification, not missing source functionality.

| Area | Current status | Next gate |
|---|---|---|
| Desktop shell | Resizable/maximizable GTK4/libadwaita window, adaptive panels, persisted geometry, native transport, diagnostics, accessible editor roles, localization boundary, and scale smoke jobs | Validate additional desktop environments |
| Channel Rack | Steps, probability, ratchets, micro-shift, swing, burn-to-pattern, groups, and reversible edits | Product polish only |
| Playlist/Piano Roll | Mixed clips, non-destructive audio edits, fades/crossfades/stretch, comping, markers, note transforms, automation, and reversible commands | Product polish only |
| Mixer/Audio | Native PipeWire/JACK/ALSA, routing, DSP, meters, latency, deterministic offline/live paths, stems, freeze/bounce, and export controls | Hardware coverage only |
| Projects | SQLite migrations, WAL, Online Backup checkpoints, atomic autosave, command replay, recording recovery, portable packaging, and crash reports | Hardware recovery coverage only |
| Plugins | Native LV2, CLAP, and VST3 discovery/processing, worker isolation, cache/quarantine, compatibility reports, and missing-plugin recovery | Distribution matrix expansion |
| Release | Desktop entry, runtime/plugin matrix, backend diagnostics, CI lint/test/soak/GTK jobs, fuzz target, and benchmark procedure | Run external fuzz and hardware campaigns |

The authoritative sequence and acceptance gates are in
[the implementation roadmap](roadmap.md). In particular, an interactive GTK
control is not counted as implemented DAW functionality until it edits the
domain through a reversible command and survives a project round trip.

## Current acceptance checks

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- GTK application startup under a live Wayland/X11 desktop

The current suite contains 49 tests, including one live-PipeWire integration
test that is ignored in device-less runs. Milestone 2 acceptance specifically
proves identical output across render block sizes; sample-accurate event,
loop, tail, tempo, and meter behavior; graph order/cycle/latency behavior;
NaN and denormal containment; and audible Pattern, Audio Clip, automation,
and Mixer-route contributions in a decoded exported WAV.

Milestone 3 acceptance proves sample-identical callback/offline snapshots,
sample-exact transport and loops, audio-clock MIDI timestamps, monitored and
recorded MIDI, explicit queue-overflow/fault/xrun/deadline diagnostics, and
zero callback allocations across 1,000 sustained cycles. The live integration
test verifies PipeWire callbacks and state-preserving stream restart against
the current desktop PipeWire service.
