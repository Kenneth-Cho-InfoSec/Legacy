# MaterialPipe

A **Material 3 Expressive** fork of
[PipePipe](https://codeberg.org/NullPointerException/PipePipe) — the
privacy-friendly YouTube, PeerTube, Bandcamp, SoundCloud and media.ccc.de
client.

MaterialPipe keeps PipePipe's privacy model (no Google accounts, no ads, no
tracking) and adds an identity: a single-source design-token system, a
brand-tinted Material 3 theme, and — once the expressive `material3` APIs go
stable — the full Material 3 Expressive toolkit (motion schemes, carousels,
floating toolbars, expressive typography).

## Vision

- **Brand-first theming.** The app colors itself per service (YouTube red,
  SoundCloud orange, …) through one token system that drives both the legacy
  View world and the new Compose world — no more ad-hoc themes.
- **Material 3 Expressive, on stable dependencies only.** Expressive adoption is
  planned for the moment `androidx.compose.material3` **1.5.0 stable** ships;
  until then the theme package is prepared to receive it and no alpha
  dependencies ship in the app.
- **Parity with upstream.** Stay a rebase-friendly fork of PipePipe, installable
  side by side with the stock app (`applicationId org.materialpipe.app`).

## Upstream record

| | |
|---|---|
| Forked from | PipePipe **v5.2.5-beta3** (`b5c33ff`) |
| Upstream remote | `https://codeberg.org/NullPointerException/PipePipeClient.git` |
| Version | `5.2.5-beta3` (`baseVersionCode 1105`) — kept in sync with upstream |
| License | **AGPL-3.0** (see `LICENSE`) |

## Status

- **Done:** brand identity + `applicationId`; `theme/` package (tokens →
  Material 3 `ColorScheme`, type/shape/motion scales, Dynamic Color);
  typography migration to `TextAppearance.PipePipe.*`; CI with baselined lint +
  screenshot regression harness; `material-icons-extended` → core;
  WCAG AA contrast guard.
- **Blocked:** Material 3 Expressive adoption (`MaterialExpressiveTheme`,
  `MotionScheme`, carousel, floating toolbars) — requires `material3` 1.5.0
  stable, which is unreleased. See `doc/materialpipe-project-plan.md`.

## Development

```sh
# Build the debug APK
./gradlew :app:assembleDebug

# Unit tests (app + vendored extractor)
./gradlew :app:testDebugUnitTest

# Lint (baselined: only new issues fail)
./gradlew :app:lintDebug

# Screenshot regression check (emulator/device on adb)
tools/screenshot_harness.sh app/build/outputs/apk/debug/*x86_64*-debug.apk
```

## Documentation

- `doc/materialpipe-project-plan.md` — milestone plan (M0–M6) and
  implementation status.
- `doc/material3-expressive-modernization.md` — study: inventory, gap table,
  risk register, dependency plan.

## License

MaterialPipe is free software: you can redistribute it and/or modify it under
the terms of the **GNU Affero General Public License** as published by the Free
Software Foundation, version 3 (see `LICENSE`). This is a fork of PipePipe
(GPLv3-or-later); all upstream licensing and attribution is preserved.
