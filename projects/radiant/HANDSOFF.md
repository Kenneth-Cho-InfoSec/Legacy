# Radiant handoff

Updated 2026-08-14. Radiant is a GPL-3.0-or-later, Linux-first DAW built
around Rust, GTK4/libadwaita, PipeWire/JACK, ALSA Sequencer, and a
UI-independent render engine. This document records the implementation state
and the remaining roadmap so work can continue without chat history.

## Current state

Milestones 0–3 are complete: persistent domain/session storage, reversible
editing, adaptive GTK4 shell, deterministic offline rendering, native Linux
audio/MIDI, transport, routing, meters, and reconnect diagnostics.

Milestone 4 and its production extensions are implemented: copy/link media
imports, fingerprints, waveform caches, relinking, non-destructive trim/slip,
stretch/fades/crossfades, comping, markers, Piano Roll transforms, native
capture, crash-safe staged takes, WAV finalization, and insertion of takes into
the project. Remaining work is product polish and external hardware coverage.

Milestone 5 core mixer behavior is implemented: route graphs reject cycles and
expose path/compensation latency; plugin latency metadata is persisted and
compiled; compensation delays run through the shared channel strip; gain/pan
are smoothed; phase inversion, stereo width, sidechain ducking, built-in
utility/EQ/dynamics/delay processing, and decimated peak/RMS/clipping meters
are available in the engine; native capture/playback calibration offsets are
represented by `LatencyCalibration` and persisted on `Session`; solo state is
undoable, rendered with downstream-bus preservation, and exposed in the GTK4
mixer; the live PipeWire/JACK path uses the shared channel strip and publishes
peak/RMS/clipping diagnostics; built-in utility/EQ/dynamics/delay behavior and
offline impulse/render regression tests pass.
`LiveMixerGraph` now performs callback-safe pre/post routing, sidechain control,
solo-path resolution, and compensation-delay processing in the native live
callback. Fixed-size `SetMixerChannel` control messages update live gain/pan,
mute/solo, phase, and width without callback allocation; a preallocated
post-mixer audio record-tap queue is exposed through `NativeAudioService` and
covered by a drain test. Existing route slots can be updated without
allocation, and live impulse tests cover pre/post taps, sidechain ducking,
compensation, mute, solo, cycle rejection, and route-slot updates. Immutable
route topologies can now be published safely from the control thread through
`arc-swap`; the callback uses preallocated runtime state, bounded delay-line
reconfiguration, and performs no allocation or locking. Native live topology
publication has a regression test.
Milestone 5 is complete within the engine/audio scope; plugin-specific live
processing remains a Milestone 6 concern.

Milestone 6 progress: `radiant-plugins` now provides native bundle discovery
for LV2/CLAP/VST3, an external JSON scanner with fingerprinted cache and
repeat-failure quarantine, serializable missing-plugin recovery, shared
parameter/state/latency/process contracts, native Lilv LV2 and Clack CLAP
metadata probes, VST3 3.8 discovery/isolated loading through `vst3-host`, and
native LV2/CLAP/VST3 instance wrappers with LV2 block processing, CLAP
activation/silence processing, and VST3 state/audio processing, plus
format capability models (LV2 URID,
Atom/Worker/State/Preset ports; CLAP note ports, GUI, callbacks; VST3 3.8
component/controller, buses/events, Linux views), and a separate worker-process
isolation supervisor with explicit latency and crash/timeout states. ABI loads
remain confined to scanner/worker boundaries; no Wine or VST2 path is
permitted. Focused and full-workspace tests pass, including scanner
timeout/quarantine and worker crash/timeout containment. The dedicated
`radiant-plugin-worker` JSON-line process now exercises instantiate, parameter,
process, state, and shutdown commands out of process. The six-case native
reference matrix (`radiant-plugins::reference_fixtures`) covers instrument and
effect workflows for each format: scan contract, instantiate, automation,
offline process, state save/restore/reopen, and missing-plugin opaque-state
recovery. Environments without third-party binaries report that limitation
explicitly; they do not pretend a system plugin was loaded.
`radiant-plugin-fixture-audit` reports runnable native bundles only (LV2 must
contain a shared object, avoiding false positives from LV2 vocabulary bundles).

Validation baseline: `cargo fmt --all -- --check`,
`cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D
warnings` pass. The live PipeWire test is intentionally ignored unless a live
desktop PipeWire session is available.

## Milestone roadmap

### Milestone 4 — Non-destructive media editing and recording

Finish Playlist paint/erase/slice/reveal/loop/group operations; distinguish
Pattern looping from Audio trim/reveal and explicit Audio stretching; add
Piano Roll multi-selection, resize-from-start, pan/pitch/ghost-channel tools;
complete record arm/input selection, software/hardware monitoring, pre-roll,
count-in, punch/loop takes, latency calibration, and recovery tests. Exit only
when import, trim, split, loop, record, undo, save/reopen, relink, interruption
recovery, and long-session waveform/MIDI responsiveness are covered.

### Milestone 5 — Mixer, routing, DSP, and latency compensation

Make the Mixer authoritative: arbitrary buses/sends, pre/post taps,
sidechains, channel layout, solo/mute, phase, stereo width, record taps,
smoothed gain/pan, peak/RMS/clipping meters, built-in EQ/dynamics/delay/utility,
graph latency compensation in real-time and offline paths, and calibrated
capture/playback offsets. Exit with sample-accurate impulse tests and matching
offline/live output through bypass, mute, solo, automation, latency, and tail
transitions.

### Milestone 6 — Native plugin hosting

Build an out-of-process scanner with cache, timeout, blacklist, quarantine, and
missing-plugin compatibility records. Host LV2 first, then CLAP, then VST3 (VST3 3.8
only; no VST2/Wine). Support discovery, audio/control ports, state, parameters,
latency/tails, presets, GUIs where available, generic GTK UI, offline render,
and opaque-state preservation. Scanner failures must not crash Radiant; worker
processing isolation makes plugin crashes recoverable.

### Milestone 7 — Automation and production workflows

Implemented sample-accurate read/write/touch/latch automation and conflict rules;
advanced Channel Rack loop/burn/ratchet/probability/micro-shift/swing tools;
Piano Roll quantize, humanize, legato, strum, arpeggiate, scales, and reusable
selection operations; Playlist markers, tempo/meter editing, grouped lanes,
comping, fades/crossfades, explicit high-quality stretch, freeze/bounce, stems,
normalization, dither, and metadata export.

### Milestone 8 — Reliability and Linux release

Implemented autosave, command-log recovery, SQLite Online Backup checkpoints,
schema migrations, portable packaging, keyboard/accessibility semantics,
localization/scale smoke testing, fuzz and soak targets, bounded-memory
diagnostics, desktop integration, backend diagnostics, distribution artifacts,
opt-in crash reports, QA guidance, and contributor documentation. External
hardware and long-running fuzz campaigns remain release verification jobs.
Public alpha requires no known project-corruption bug, forced-termination
recovery tests, published distro/runtime and plugin matrices, and CI or
documented hardware coverage for migration, accessibility, real-time stress,
and project round trips.

## Architectural rules

- Native Linux means PipeWire/JACK/ALSA; no Wine bridge is in scope.
- GTK4/libadwaita owns presentation; the domain remains UI-independent.
- Audio callbacks never allocate, lock, access files, call GTK, or block.
- Source recordings and imported media are immutable; editing stores offsets,
  ranges, gains, and derived caches.
- Every user edit crosses a reversible command transaction.
- Render plans are immutable snapshots; offline and real-time paths must agree.
- Cache and recovery files are disposable/repairable; project snapshots and
  command logs are durable state.

## Important files

- `docs/roadmap.md` — full audited roadmap and research rationale.
- `crates/radiant-domain/src/lib.rs` — persistent model and schema version.
- `crates/radiant-commands/src/lib.rs` — reversible command interpreter.
- `crates/radiant-session/src/lib.rs` and `recording.rs` — SQLite and takes.
- `crates/radiant-engine/src/plan.rs`, `render.rs`, `delay.rs`, `smoothing.rs`.
- `crates/radiant-audio/src/capture.rs` — native input capture queue.
- `crates/radiant-ui/src/main.rs` — GTK4 transport and editors.
- `docs/plugins.md` — native plugin scanner, adapter, worker, and fixture workflow.
