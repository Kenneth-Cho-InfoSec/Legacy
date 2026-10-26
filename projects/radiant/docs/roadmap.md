# Radiant implementation roadmap

This roadmap was re-baselined on 2026-08-13 against the repository, not
against screenshots or intended features. Radiant is currently an interactive
workflow prototype on top of a small persistent domain foundation. It is not
yet an audio application.

## Status language

- **Implemented** means the behavior is backed by project state, commands,
  persistence where applicable, tests, and the relevant runtime subsystem.
- **Interactive prototype** means the UI can be operated but uses temporary
  state local to `radiant-ui` and does not survive reopening a project.
- **Compatibility record** means serialized state is retained while an
  optional external capability is unavailable; it is not a fake processing path.
- **Planned** means no implementation should be inferred from the UI.

## Audited baseline — Milestone 0

Status: **complete foundation; Milestones 1 through 3 are complete**.

Implemented:

- a ten-crate Rust workspace: domain, commands, session storage, application
  controller, GTK UI, engine, graph, media, native audio, and the isolated
  PipeWire adapter;
- stable UUID identities, 960 PPQ musical time, Channels, Patterns, untyped
  Playlist lanes, mixed clip types, Mixer inserts, routes, media references,
  plugin slots, and validation;
- atomic reversible commands for project name, initial tempo, structural
  Channel/Pattern/Playlist CRUD, steps, notes, clip ranges, Channel controls,
  and Mixer controls;
- SQLite project creation/opening, a JSON session snapshot, transactional
  command journal, schema check, and WAL configuration;
- a resizable GTK4/libadwaita shell with adaptive panels and system styling;
- an application controller with selection/presentation state, undo/redo,
  dirty-state tracking, explicit save/open boundaries, and project round-trip
  tests;
- Session-backed Channel Rack, Playlist clip, Mixer-control, and separate Piano
  Roll note editing paths;
- schema-version-2 project and presentation persistence with a tested version-1
  migration;
- strict formatting and Clippy checks plus 49 unit and acceptance tests;
- immutable render-plan compilation, deterministic offline rendering,
  musical transport, Mixer routing, media decoding/resampling, and WAV export.

Not implemented:

- Playlist slice/loop tools and several advanced Channel Rack menus remain
  scaffolding;
- destructive audio editing and durable recording/take workflows are not
  functional;
- split, plugin, and many advanced edit commands do not exist;
- there is no waveform cache, recording/take finalization path, plugin scanner,
  or plugin host;
- autosave, recovery, and backup/export packaging do not exist.

The next milestone builds non-destructive media editing and durable recording
on the native engine. Native plugin hosting is now implemented through the
worker boundary described by the plugin milestone.

## Milestone 1 — One durable editing path

Goal: every visible edit in one representative workflow is a real project
edit that can be undone, saved, reopened, and replayed.

Status: **complete on 2026-08-13**. The representative workflow is projected
from `Session`, all edits cross reversible command boundaries, project and
presentation state survive reopening, and continuous gestures are coalesced.

Work:

1. Split the monolithic UI into workspace, Channel Rack, Playlist, Mixer,
   Piano Roll, drawing, and action modules.
2. Introduce an application controller that owns `Session`, selection,
   presentation state, the active project store, and undo/redo stacks.
3. Keep selection, hover, zoom, scroll, and panel geometry outside the
   persistent musical domain; persist presentation state separately.
4. Add commands for steps, notes, note properties, clip movement/resizing,
   channel parameters, mixer parameters, inserts, routes, and automation
   points. Coalesce a continuous pointer drag into one undo transaction.
5. Replace the duplicate UI-only `PianoNote` and `PlaylistClip` models with
   projections of domain objects.
6. Wire New/Open/Save/Save As, dirty-state prompts, undo/redo, and error
   reporting. Add schema version 2 through a tested migration rather than
   silently changing version 1.
7. Persist panel visibility, divider positions, zoom, and last editor state.

Exit gate:

- create a Channel and Pattern, paint steps, add/resize a Piano Roll note,
  place/move/resize its Pattern Clip, change a Mixer fader, undo and redo every
  edit, save, close, reopen, and obtain the same project;
- one drag produces one undo entry;
- malformed and future-schema projects fail without modifying user data;
- UI tests cover action enablement and the dirty-state lifecycle.

Exit evidence:

- `milestone_one_workflow_round_trips_every_edit` covers the complete create,
  step, note, clip, Mixer, history, save, and reopen sequence;
- coalescing tests prove continuous updates create one history entry;
- migration tests prove version-1 upgrade, future-schema rejection, and
  malformed-state failure without rewriting source data;
- UI adapter tests cover history-action enablement, while controller tests
  cover dirty-state transitions;
- strict Clippy, all 24 tests, the GTK build, and native startup pass.

## Milestone 2 — Transport and deterministic offline rendering

Goal: establish musical and signal-processing correctness without involving a
real-time device callback.

Status: **complete on 2026-08-13**. Engine, graph, and media are independent of
GTK; mutable Sessions compile to immutable plans; the same renderer backs
tested offline buffers and the GTK Audio Export action.

Work:

1. Add UI-independent engine, graph, and media crates.
2. Compile mutable `Session` state into an immutable render plan. Publish plan
   snapshots at explicit boundaries; never let the renderer traverse GTK or
   mutable project collections.
3. Implement tempo/time-signature conversion, loop ranges, playhead state,
   sample-accurate note/automation event scheduling, and transport commands.
4. Implement a small built-in sampler or test instrument, metronome, gain/pan,
   summing, and the existing Mixer routing model.
5. Decode at least WAV and FLAC, resample when source and session rates differ,
   and render a session offline to WAV.
6. Test graph ordering, cycle rejection, event boundaries, looping, tails,
   deterministic output, denormals, and NaN containment.

Exit gate:

- the same session produces equivalent output across different render block
  sizes;
- transport positions agree with rendered sample positions across tempo and
  meter changes;
- a Pattern, an Audio Clip, automation, and a Mixer route are audible in an
  exported WAV;
- engine crates have no GTK dependency.

Do not start a device backend before this gate passes.

Exit evidence:

- exact sample equality is tested at 17-frame and 1024-frame render blocks;
- timeline and transport tests cover tempo boundaries, meter changes, seeks,
  explicit state transitions, and sample-exact loop wrapping;
- event tests cover leading silence, exact start boundaries, instrument tails,
  loop repetition, and invalid/denormal sample containment;
- graph tests cover stable ordering, cycle rejection, and longest-path latency;
- the acceptance session routes a Pattern test instrument through two Mixer
  inserts, resamples an Audio Clip, applies linear Mixer automation, exports a
  float WAV, decodes it again, and verifies every contribution is audible;
- `cargo tree` manifests show no GTK dependency in `radiant-engine`,
  `radiant-graph`, or `radiant-media`.

## Milestone 3 — Native Linux real-time audio and MIDI

Goal: play the offline-proven render plan through native Linux services.

Status: **complete on 2026-08-13**. PipeWire is the direct primary adapter,
JACK is the native secondary adapter, and ALSA Sequencer provides MIDI input,
timestamping, subscription, and topology announcements. The GTK transport now
drives live audio and displays backend diagnostics and master meters.

Backend order:

1. PipeWire is the primary desktop audio backend.
2. JACK is a native secondary backend for studio deployments that expose a
   JACK server directly.
3. ALSA Sequencer provides native MIDI ports, subscriptions, timestamps, and
   hot-plug announcements.
4. Direct ALSA PCM is deferred until a demonstrated deployment need justifies
   its device-management and exclusivity complexity.

Work:

- isolate backend traits from the engine and negotiate sample rate, channel
  layout, quantum, and device changes;
- use bounded preallocated queues between GTK/control threads and the audio
  thread;
- keep allocation, locks, filesystem access, GTK, blocking calls, and normal
  logging out of the process callback;
- implement play/pause/stop/seek/loop, MIDI input monitoring, timestamp to
  engine-frame conversion, meters, xrun counters, and recoverable backend
  reconnect;
- provide an audio/MIDI setup page and diagnostics instead of hiding backend
  errors.

Exit gate:

- live output matches offline rendering for the same block sequence;
- MIDI can trigger the built-in instrument and record correctly timestamped
  notes;
- device loss, sample-rate changes, and backend restart do not corrupt the
  project;
- an automated real-time stress harness detects callback allocations, locks,
  deadline misses, and queue overflow.

Exit evidence:

- `realtime_blocks_match_offline_snapshot_exactly` compares every sample from
  varying callback blocks against the Milestone-2 snapshot;
- transport tests cover play, pause, stop, seek, looping, and end-of-stream at
  exact sample frames;
- MIDI tests map monotonic timestamps to the callback clock, monitor notes
  through the built-in instrument, and record their exact playhead frames;
- bounded `rtrb` queues report command, MIDI, and record overflow without
  blocking the callback;
- the stress test measures zero allocations across 1,000 callback cycles and
  exposes deadline, xrun, fault, meter, and overflow counters atomically;
- the live PipeWire test processes real desktop callbacks, restarts the stream,
  restores transport position/state, and resumes without mutable project access;
- Audio & MIDI Setup exposes Automatic/PipeWire/JACK selection and discovered
  outputs; GTK displays negotiated rate, quantum, xruns, peak meters, and
  reconnect status.

## Milestone 4 — Non-destructive media editing and recording

Goal: make the Playlist and Piano Roll useful for real production work.

Work:

- media import with copy/link policy, content metadata, missing-media relink,
  background peak generation, and cache invalidation;
- complete selection, move, copy, paint, erase, slice, trim, reveal, loop,
  mute, slip, duplicate, and grouped edit commands;
- distinguish Pattern loop extension, Audio trim/reveal, and explicit Audio
  stretch mode. Edge resizing must not silently time-stretch audio;
- complete Piano Roll resize-from-start option, move, multi-selection,
  velocity/pan/pitch editing, ghost-channel switching, and keyboard preview;
- record arm, input selection, software/hardware monitoring, pre-roll,
  count-in, punch, loop takes, input latency calibration, and crash-safe take
  finalization;
- never rewrite source recordings during editing.

Exit gate:

- import, trim, split, loop, record, undo, save, reopen, and relink media
  without altering the source file;
- interrupted recording leaves recoverable audio and a consistent project;
- waveform and MIDI views stay responsive on long sessions.

## Milestone 5 — Mixer, routing, DSP, and latency compensation

Goal: turn the Mixer prototype into the authoritative signal-flow editor.

Work:

- arbitrary insert/bus routes, sends, pre/post fader taps, sidechains, channel
  layout negotiation, solo/mute semantics, phase, stereo width, and record
  taps;
- smoothed gain/pan, peak/RMS meters, clipping history, and meter decimation
  for the UI;
- built-in EQ, dynamics, delay, and utility processors sufficient to test
  state, automation, latency, tails, and bypass behavior;
- graph latency analysis and plugin delay compensation for both playback and
  offline render, plus calibrated capture/playback offsets;
- routing-cycle diagnostics rather than undefined feedback behavior.

Exit gate:

- impulse tests verify every pre/post route and compensation path to the
  sample;
- offline and real-time mixer output agree;
- bypass, mute, solo, automation, latency changes, and tails are click-free at
  supported transitions.

## Milestone 6 — Native plugin hosting

Goal: host native Linux plugins without making project recovery depend on
plugin quality.

Order and scope:

1. Build an out-of-process scanner, cache, timeout, blacklist/quarantine, and
   missing-plugin compatibility record before loading plugins in a project.
2. Implement LV2 first: discovery, audio/control/Atom ports, URID mapping,
   State, Worker, presets, native UI where compatible, and a generic GTK UI.
3. Implement CLAP next: audio/note ports, parameters, state, latency, tail,
   GUI, main/audio-thread contracts, and host callbacks.
4. Implement VST3 after the shared host model is stable: component/controller,
   buses, events, parameters, state, latency, and Linux views. Use the MIT
   licensed VST3 3.8 SDK/API; do not add VST2.
5. Add processing isolation as a separate deliverable. Out-of-process scanning
   alone is not plugin crash containment, and process isolation may impose
   latency that must be explicit.

Exit gate per format:

- instrument and effect reference plugins pass scan, instantiate, process,
  automate, save/restore, reopen, offline render, and missing-plugin tests;
- a scan crash or timeout cannot terminate Radiant;
- projects preserve unknown plugin IDs and opaque state without data loss;
- unsupported required features produce a clear compatibility report.

No Wine bridge is in scope.

## Milestone 7 — Automation and production workflows

Goal: complete the pattern-first workflow after the underlying engine is real.

Work:

- sample-accurate read/write/touch/latch automation, curve editing, parameter
  gestures, and automation conflict rules;
- advanced Channel Rack loop controls, burn-to-pattern, ratchets,
  probability, micro-shift, channel groups, and swing;
- Piano Roll quantize, humanize, legato, strum, arpeggiate, scale tools,
  selection stretching, and reusable editing operations;
- Playlist markers, tempo/meter editing, grouped lanes, comping, fades and
  crossfades, explicit high-quality Audio stretching, and freeze/bounce;
- export ranges, stems, normalization/dither choices, and metadata.

Exit gate:

- automation is identical in offline and real-time paths;
- freeze/unfreeze and bounce preserve timing and routing;
- each destructive-looking operation has an undoable, non-destructive source
  representation.

## Milestone 8 — Reliability and Linux release

Goal: make Radiant supportable as a public alpha, then beta.

Work:

- autosave, command-log recovery, SQLite checkpoints, Online Backup API use,
  migrations from every released schema, snapshots, and portable project
  packaging;
- keyboard-complete editors, accessible semantics/actions for custom timeline
  canvases, high contrast, reduced motion, localization, and scale-factor
  testing;
- bounded-memory and long-session profiling, fuzzing of project/plugin/media
  parsers, failure injection, soak tests, and reproducible benchmarks;
- desktop file integration, XDG locations, PipeWire/JACK permissions and
  diagnostics, distribution packaging, crash reports with opt-in privacy,
  user manual, and contributor architecture documentation.

Public-alpha gate:

- no known project-corruption bug;
- recovery is tested after forced termination during editing and recording;
- supported distro/runtime matrix and plugin compatibility matrix are
  published;
- accessibility, migration, real-time stress, and project round-trip checks
  run in CI or documented hardware test jobs.

## Research decisions behind the order

- GTK drawing surfaces need event controllers for input, and custom controls
  need explicit accessible semantics and actions. This makes editor widget
  architecture and accessibility Milestone 1 concerns, not release polish:
  [GTK DrawingArea](https://docs.gtk.org/gtk4/class.DrawingArea.html),
  [GTK accessibility](https://docs.gtk.org/gtk4/section-accessibility.html).
- Libadwaita breakpoints are appropriate for the existing adaptive shell, but
  containers using breakpoints require an explicit supported minimum size:
  [adaptive layouts](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/adaptive-layouts.html).
- PipeWire can emit stream processing on a data real-time thread; JACK likewise
  requires process-thread code suitable for real-time execution. Both support
  the render-plan/control-thread separation above:
  [PipeWire thread loop](https://docs.pipewire.org/page_thread_loop.html),
  [JACK real-time thread rules](https://jackaudio.org/api/group__NonCallbackAPI.html).
- ALSA Sequencer provides dynamic clients/ports, subscriptions, timestamped
  event queues, and announcements, so it is the native MIDI foundation rather
  than a raw byte-only abstraction:
  [ALSA Sequencer](https://www.alsa-project.org/alsa-doc/alsa-lib/seq.html).
- Ardour demonstrates durable session directories, non-destructive regions,
  explicit latency compensation, and native Linux plugin/routing concerns:
  [Ardour sessions and architecture](https://manual.ardour.org/ardourmanual.html),
  [non-destructive workflow](https://manual.ardour.org/introducing-ardour/creating-music-with-ardour/),
  [signal flow](https://manual.ardour.org/signal-routing/signal-flow/).
- LV2 State is designed for portable persistent plugin state, LV2 Worker moves
  non-real-time work out of processing, and LV2 UIs are distinct from DSP
  instances: [LV2 State](https://lv2plug.in/ns/ext/state),
  [LV2 Worker](https://lv2plug.in/ns/ext/worker),
  [LV2 UI](https://lv2plug.in/ns/extensions/ui).
- CLAP explicitly assigns methods to main, audio, or thread-safe contexts and
  defines state, parameters, ports, latency, tail, GUI, and thread support as
  extensions: [official CLAP repository](https://github.com/free-audio/clap).
- VST3 3.8 is now MIT licensed and its official documentation states that MIT
  is compatible with GPLv3; VST2 remains excluded:
  [VST3 licensing](https://steinbergmedia.github.io/vst3_dev_portal/pages/FAQ/Licensing.html).
- SQLite WAL files are part of persistent state and must stay with the database
  unless a safe checkpoint/backup is performed. Portable saves and autosaves
  should use controlled checkpoints or the Online Backup API rather than
  copying `project.sqlite` alone:
  [SQLite WAL](https://www.sqlite.org/wal.html),
  [SQLite Online Backup API](https://www.sqlite.org/backup.html).
