# Native Linux plugin hosting

Radiant keeps plugin loading outside the GTK and realtime callback paths.
`radiant-plugin-scanner` probes one bundle at a time; `ScanSupervisor` applies
timeouts, cache fingerprints, blacklist/quarantine policy, and missing-plugin
recovery. `radiant-plugin-worker` is the separate processing boundary and uses
the line-delimited `HostRequest`/`HostResponse` protocol.

Native format adapters are intentionally format-specific:

- LV2 uses Lilv for bundle metadata and port discovery. The host model exposes
  audio, control, Atom, URID, State, Worker, preset, and UI capabilities.
- CLAP uses Clack for entry/factory discovery and lifecycle activation. Main,
  audio, callback, note-port, parameter, latency, tail, and GUI capabilities
  are represented in the shared compatibility model.
- VST3 uses the MIT-licensed `vst3-host` 3.8 API with process isolation. The
  adapter preserves component/controller, bus/event, parameter, state,
  latency, and Linux-view metadata.

Useful checks:

```text
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p radiant-plugins --bin radiant-plugin-fixture-audit
```

The fixture audit reports runnable `.lv2` bundles containing a shared object,
`.clap` files, and `.vst3` bundles. LV2 vocabulary/specification bundles are
excluded. A machine with no third-party binaries must use the six built-in
reference fixtures; it must not report native plugin coverage as live coverage.

Project files retain unknown plugin IDs and opaque state. Unsupported required
features produce a compatibility report, and missing plugins are represented by
recoverable serialized records rather than preventing a project from reopening.
