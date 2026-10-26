# Architecture overview

Radiant separates editable project state, presentation state, persistence,
and its UI-independent audio engine.

```text
GTK4 UI -> reversible commands -> Session
                                  |      \
                                  |       -> immutable render-plan compiler
                                  |                    |
                                  |                    -> deterministic renderer
                                  -> SQLite project store
```

The central workflow relationship is deliberately not track-to-track:

```text
Pattern Clip -> Pattern -> Channel Rack sources -> Mixer inserts
Audio Clip   -> immutable media source ----------> Mixer inserts
Automation   -> typed target --------------------> parameters
```

Playlist lanes are untyped containers. A lane can hold Pattern, Audio, and
Automation clips, including overlaps. Optional track-mode associations may
link a Playlist lane, Channel, and Mixer insert without changing the base
model.

## Invariants

- Domain types do not depend on GTK or Linux audio APIs.
- Persistent objects use stable UUIDs; list positions are never identities.
- Musical positions use 960 pulses per quarter note.
- Recorded media will be immutable; edits will reference ranges.
- UI mutations are expressed as commands and transactions.
- The real-time audio callback allocates no memory, acquires no locks, touches
  no filesystem, and invokes no GTK code.

## Desktop presentation

The application shell uses GTK4-rs and libadwaita without a custom
stylesheet. `AdwToolbarView` and `AdwHeaderBar` provide the global window
chrome, `AdwOverlaySplitView` adapts the Browser into an overlay at compact
widths, and nested `GtkPaned` containers preserve the Channel Rack, Mixer,
Picker, and Playlist relationship while allowing the user to resize every
work area. Panels are independently closable through their native header
controls and reopenable through persistent View actions. Hiding both children
of a pane also hides that parent pane, allowing the remaining workspace to
consume all available space. The window follows the GNOME system color scheme
and accent.

The application controller owns session commands, selection/presentation
state, coalesced undo/redo, dirty state, and the active `ProjectStore`. Channel
Rack, Playlist, Mixer controls, and Piano Roll notes are projections of
`Session`; window-scoped history actions resynchronize those projections.
Pointer drags change temporary previews or use a coalesced parameter boundary,
so one continuous gesture produces one undo entry. Project schema version 2
persists presentation state separately and migrates version-1 projects in a
transaction. Reusable workspace, action, drawing, and editor conversions live
in focused UI modules while GTK widget composition remains in the desktop
entry point.

## Engine boundary

The engine is developed offline-first. Mutable `Session` state is compiled at
an explicit boundary into an immutable `RenderPlan`; rendering never traverses
GTK or mutable project collections. The plan owns deterministic graph order,
sample-frame note/audio/automation events, decoded and resampled media, Mixer
routes, and musical timeline conversion. The renderer supplies the built-in
test instrument, metronome, gain/pan, summing, loop mapping, tail rendering,
sample sanitation, and stereo WAV export. Equivalent sessions produce exactly
the same samples across tested block sizes.

`radiant-graph` performs stable topological ordering, cycle rejection, and
longest-path latency analysis. `radiant-media` decodes WAV and FLAC into planar
floating-point buffers, performs linear sample-rate conversion, and writes
float WAV. None of `radiant-engine`, `radiant-graph`, or `radiant-media`
depends on GTK. The GTK shell can control the musical playhead and invoke
offline export, but audible device playback intentionally waits for the native
Linux backend milestone.

PipeWire and JACK callbacks may exchange only preallocated bounded messages
and render-plan snapshots with the control side. ALSA Sequencer belongs behind
a MIDI backend boundary and timestamps events before they enter the engine.
GTK, SQLite, plugin discovery, media decoding, graph compilation, allocation,
and ordinary logging remain outside the real-time callback.

## Native Linux real-time boundary

`radiant-audio` owns the device-independent callback processor and bounded
single-producer/single-consumer queues. The control side publishes an immutable
offline-proven audio snapshot and sends fixed-size transport or MIDI messages;
the callback performs no allocation, locking, filesystem access, GTK calls, or
logging. Atomic snapshots expose playhead, peak/RMS meters, quantum, xrun,
deadline, overflow, fault, and reconnect counters to GTK at a lower rate.

`radiant-pipewire` is a deliberately small FFI island. Its C adapter uses the
official mapped-buffer `pw_stream` API with `PW_STREAM_FLAG_RT_PROCESS`; its
safe Rust wrapper owns callback lifetime and stops the PipeWire loop before
releasing callback state. This avoids requiring build-time bindgen while still
using PipeWire directly. JACK uses CPAL's native JACK host as a secondary
adapter. Direct ALSA PCM is not used.

ALSA Sequencer input runs on its own nonblocking thread. It exposes a
subscribable application port, listens for topology announcements, translates
note events against the current audio callback clock, and pushes fixed-size
messages into the audio queue. MIDI monitoring and recording therefore share
the same sample-frame timestamp.

See [the implementation roadmap](../roadmap.md) for staged acceptance gates.

## Clean-room policy

Ardour, LMMS, and FL Studio may be studied to understand user-visible
behavior and Linux integration. Radiant code, assets, presets, and DSP are
independently implemented. Contributions derived from third-party source
must document provenance and license compatibility before review.
