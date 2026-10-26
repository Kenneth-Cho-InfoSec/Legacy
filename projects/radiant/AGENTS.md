# Repository Guidelines

## Project Structure

Radiant is a Rust workspace. Core crates live under `crates/`: `radiant-domain`
contains persistent session types, `radiant-commands` reversible edits,
`radiant-session` SQLite storage and recovery, `radiant-engine` deterministic
rendering, `radiant-graph` routing, `radiant-media` decoding/caches,
`radiant-audio` native real-time services, `radiant-pipewire` the C adapter,
`radiant-app` controller logic, and `radiant-ui` GTK4/libadwaita presentation.
Architecture and roadmap documents are in `docs/`; desktop resources are in
`resources/`; CI definitions are in `.github/workflows/`.

## Build, Test, and Development Commands

Run commands from the repository root:

```sh
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
timeout 5s cargo run -p radiant-ui
```

The test suite includes domain, persistence, render, graph, audio, and GTK
adapter tests. The live PipeWire test is ignored unless a desktop PipeWire
session is available; run it explicitly with `cargo test -p radiant-audio --
--ignored --nocapture` when appropriate.

## Coding Style and Naming

Use Rust 2024 conventions, four-space indentation, `snake_case` functions and
modules, `UpperCamelCase` types, and descriptive command labels. Format with
`rustfmt`; treat Clippy warnings as errors. Keep domain/engine crates free of
GTK dependencies. Real-time callbacks must not allocate, lock, access files,
call GTK, block, or perform normal logging. Use `apply_patch` for focused edits.

## Testing Guidelines

Place unit tests beside the implementation in `#[cfg(test)]` modules. Name
tests after observable behavior, such as
`project_round_trip_preserves_session`. Add regression tests for every command,
schema migration, render-path change, and recovery behavior.

## Commits and Pull Requests

Use short imperative commit subjects, for example `Add media relink command`.
Keep commits focused. Pull requests should explain behavior and architectural
impact, list verification commands, link relevant issues, and include GTK
screenshots or recordings when UI behavior changes. Do not commit generated
`target/` artifacts or user project data.

## Architecture Rules

Persistent edits cross reversible command transactions. Source media remains
immutable; trims, slips, ranges, and caches are derived state. Render plans are
immutable snapshots, and offline and real-time paths must remain equivalent.
Native Linux audio uses PipeWire/JACK/ALSA; Wine bridges are out of scope.
