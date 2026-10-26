# Reliability and release QA

The deterministic CI gate is:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The test suite covers project round trips, schema migrations, malformed
project rejection, command-log replay, autosave/checkpoint recovery,
recording-take recovery, native plugin worker isolation, realtime allocation
and callback stress, render determinism, and backend diagnostics.

Hardware jobs should run the same commands with a live PipeWire session and
ALSA MIDI device, then exercise forced termination during editing and
recording. Repeat the realtime stress tests for at least 30 minutes and record
XRUN, queue-overflow, allocation, and peak-memory counters.

For parser fuzzing, feed arbitrary UTF-8 and truncated bytes to the session,
plugin-cache, worker-protocol, and media decoders under `cargo fuzz` or an
equivalent sanitizer runner. A parser failure must return a typed error and
must not rewrite the source project or cache.

GTK release jobs should verify keyboard traversal, accessible names/roles,
high-contrast and reduced-motion settings, 100/125/150/200% scale factors,
Wayland and X11 startup, and localization of all user-visible actions.
