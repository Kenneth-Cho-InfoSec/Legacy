# Reproducible performance checks

The deterministic render test compares output across block sizes. For a
release performance run, execute the recovery soak and the render test with
release optimizations:

```text
cargo run --release -p radiant-session --bin radiant-recovery-soak
cargo test --release -p radiant-engine render::tests::output_is_identical_across_render_block_sizes -- --exact
```

Record wall time, peak RSS, allocation counters, XRUNs, and queue-overflow
diagnostics on the release hardware. Compare runs using the same sample rate,
render block sizes, CPU governor, and PipeWire quantum.

The optional fuzz target can be run with cargo-fuzz:

```text
cargo fuzz run session_json -- fuzz/corpus/session_json
```

Fuzzing must be run outside the normal workspace because libFuzzer supplies
its own runner. Crashes and minimized corpus files are release blockers.
