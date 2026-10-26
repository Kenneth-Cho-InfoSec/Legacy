# Contributing

Radiant accepts contributions under GPL-3.0-or-later.

## Clean-room rule

Do not copy source code, artwork, icons, presets, project files, samples, or
documentation from FL Studio or any other proprietary product. Ardour and
LMMS may be studied as GPL-compatible Linux audio references, but copied or
adapted code must be declared before review with its exact origin, license,
and modifications. Prefer independent Rust implementations.

## Engineering rules

- Domain and engine crates must not depend on GTK.
- Session mutations must use commands and transactions.
- No allocation, locks, filesystem access, synchronous logging, or GTK calls
  will be allowed in the real-time callback.
- Keep unsafe code denied globally. Isolate unavoidable FFI in narrowly
  reviewed backend crates when those crates are introduced.
- Add tests for every persistent command, migration, and DSP behavior.

Before submitting changes, run:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
