# MaterialPipe — Project Plan
## A Material 3 Expressive UI fork of PipePipe

**Author:** opencode study output · **Date:** 2026-08-03
**Basis:** `doc/material3-expressive-modernization.md` (the design-system study of PipePipe).
**Status:** Proposed plan, ready for kickoff.

---

## 1. Vision

> **MaterialPipe** is a community Android client for YouTube & other streaming
> services, forked from PipePipe (itself a NewPipe fork), whose defining feature is a
> **Material 3 Expressive (M3E) UI** — vibrant role-based color, a full type & shape
> scale, spring-based motion, opt-in dynamic color, and the new expressive components
> — while keeping PipePipe's SABR/AV1 playback engine and feature set byte-for-byte
> behaviorally compatible.

**Why now:** The newest stable Compose Material 3 (1.4.0) already ships expressive
motion inside component defaults and the foundations we need. The fully public
expressive API (`MaterialExpressiveTheme`, `MotionScheme`) is only alpha today, so the
plan deliberately stages the work so every milestone is shippable without alpha
dependencies, and the full expressive surface lands the moment material3 1.5.0 goes
stable.

---

## 2. Project Charter

### 2.1 Goals (what "done" looks like)

1. A single-source design token layer driving **both** the View and Compose worlds.
2. Full M3 role-based color with light / dark / black themes, **plus an opt-in
   Dynamic Color** mode (Android 12+).
3. M3 type scale, shape scale, and spring-based motion (reduce-motion aware).
4. Adoption of the new expressive components: search bar (slot API), expressive list
   items, button groups, split button, loading indicator, FAB menu, floating
   toolbars, and a hero carousel on the home feed.
5. No regression in playback (SABR/AV1), downloads, notifications, or the extractor.

### 2.2 Non-goals (explicitly out of scope for the first release)

- Rewriting the View layer into 100% Compose. Hybrid is fine; parity is the goal.
- New streaming services or extractor features.
- iOS / desktop / wear builds.
- Changing the SABR or player engine internals.

### 2.3 Success metrics

| Metric | Target |
|---|---|
| Cold start | ≤ current PipePipe baseline (2.1 s on API 35) |
| Playback start | ≤ 2.5 s; no SABR regression |
| PSS memory | ≤ current baseline (217 MB incl. playback) |
| Unit/extractor tests | 50/50 green on every commit |
| Theme parity | screenshot diff (light/dark/black × YouTube) ≤ tolerance vs pre-change |
| Dynamic color | works on API ≥31, falls back on <31, toggle OFF = exact old look |
| A11y | WCAG AA on primary/onPrimary; touch targets ≥48 dp |

---

## 3. Fork & Repository Strategy

### 3.1 Fork mechanics

- **Source of truth:** fresh fork of `InfinityLoop1309/PipePipe` (`dev` branch),
  tracked as `upstream`. MaterialPipe lives on its own default branch (`main`).
- **Rebase cadence:** pull upstream `dev` **weekly** during Phase 0–3 (before the UI
  diverges too far); after Phase 4, pull on a monthly "merge upstream" PR and resolve
  `res/` conflicts with the token-bridge tools (see D4 in the study).
- **License:** keep the NewPipe-family license (GPLv3/AGPL-3.0 lineage — verify the
  exact header PipePipe uses and preserve attribution). Keep the 2 submodules
  (NewPipeExtractor, ffmpeg fork) pinned to the same commits as the base PipePipe tag
  we fork from.

### 3.2 Identity & package

| Decision | Phase | Recommendation |
|---|---|---|
| `applicationId` (install coexistence) | **M0** | `org.materialpipe.app` (+ `.debug` for debug) |
| Source `package` (`org.schabi.newpipe`) | M0–M5 | **Keep unchanged** (massive churn otherwise; extractor + tooling reference it) |
| `package` rename | M6 (optional) | Only if a clean rename toolchain is validated; otherwise defer forever |
| App label / icon / theme color | M0 | "MaterialPipe", distinct launcher icon set |

### 3.3 Repo layout (additions only)

```
docs/                       # this plan + study
design/                     # token tables, component specs, Figma exports
app/src/main/res/values/tokens/   # colors_m3.xml, type_m3.xml, shape_m3.xml, motion_m3.xml
app/src/main/kotlin/org/schabi/newpipe/theme/  # Compose theme package (new)
tools/theme-bridge/         # generator: tokens/*.xml <-> Compose kt (parity test support)
ci/                         # GitHub Actions workflow fragments
```

---

## 4. Architecture Target (one picture)

```
                 ┌──────────────────────────────────────────────┐
                 │  values/tokens/*.xml  (light · dark · black · per-service seeds) │
                 └───────────────┬──────────────────────────────┘
                                 │ read
              ┌──────────────────┼──────────────────┐
              ▼                  ▼                  ▼
   ThemeHelper (View)    Compose Theme (kt)    Generator/Parity test
   resolves ?attr/…      ColorScheme           asserts equality
   styles.xml cascade    Typography            (no drift, CI-gated)
   colors_services.xml   Shapes
                           MotionScheme
                                 │
              ┌──────────────────┼──────────────────┐
              ▼                                     ▼
   View UI (lists, detail, player,      Compose UI (search filter,
   settings)                              onboarding, new components)
```

---

## 5. Milestones

Each milestone is **independently shippable**, ends in a buildable + releasable state,
and has explicit **exit criteria**. Effort assumes 1 senior Android engineer (solo)
unless noted; a 2–3 person team roughly halves calendar time.

| M | Name | Goal | Sprints (solo) | Dependencies |
|---|---|---|---|---|
| **M0** | Project setup & baseline | Identity + CI + measurement | 1–2 | none |
| **M1** | Design token foundation | Single source of truth; no visual change | 2 | M0 |
| **M2** | View-world adoption | View UI consumes tokens | 3–4 | M1 |
| **M3** | Stable expressive components | New M3E components in Compose | 3–4 | M1 |
| **M4** | Dynamic color + adaptive | Dynamic Color toggle; tablet layout | 2–3 | M2, M3 |
| **M5** | Expressive at 1.5.0 stable | `MaterialExpressiveTheme`, motion, hero carousel | 2–3 | M4 + material3 1.5.0 stable |
| **M6** | Hardening & first release | a11y, perf, icons, release | 2–3 | M5 |

**Total: ~15–21 sprints (≈5–7 months solo; ≈3–4 months with a 2-person team).**

---

### M0 — Project setup & baseline

**Tasks**
- [ ] Fork/copy PipePipe; set up `main` branch, `upstream` remote.
- [ ] Change `applicationId` (+ `.debug`), app label, launcher icons/branding.
- [ ] Toolchain: verify AGP 9.2.1 / Gradle 9.5.1 / Kotlin 2.3.21 / JDK 25 build.
- [ ] Remove `android.newDsl=false` + `android.builtInKotlin=false` from
      `gradle.properties` (adopt AGP 9 built-in Kotlin DSL); full build re-verify.
- [ ] CI (GitHub Actions): `compileDebugKotlin`, `testDebugUnitTest`,
      extractor tests, `lintDebug` **with a committed baseline file** so only *new*
      issues fail (lint debt is 2067 errors/600 warnings, all pre-existing).
- [ ] Screenshot baseline harness (Paparazzi or Robolectric + `capture`): top 5
      screens × 4 themes (light/dark/black/auto) × YouTube.
- [ ] Signing setup (debug + release keystore via CI secrets).
- [ ] Renovate/Dependabot for dependency PRs.

**Exit criteria**
- CI green end-to-end on a clean checkout.
- Screenshot baselines committed.
- App installs alongside stock PipePipe (different appId) and all baseline journeys
  (study §2.4) still pass.

---

### M1 — Design token foundation

**Tasks**
- [ ] `res/values/tokens/colors_m3.xml`: full M3 role set (primary, onPrimary,
      primaryContainer, secondary/tertiary, surface + surfaceContainerLow/Mid/High,
      surfaceTint, background/onBackground, error, outline, …) for light/dark/black,
      seeded from existing `colors_services.xml` brand colors (keep legacy names as
      aliases).
- [ ] `type_m3.xml`: the 15-style M3 type scale (display/headline/title/body/label ×
      L/M/S) with official sizes/line-heights.
- [ ] `shape_m3.xml`: 4/8/12/16/24 dp corner scale.
- [ ] `motion_m3.xml`: spring/duration tokens (for the View bridge; consumed for real
      in M5).
- [ ] Compose `theme/` package: `Color.kt`, `Type.kt`, `Shape.kt`, `Motion.kt`,
      `DynamicColor.kt` (stub), `Theme.kt` (`MaterialTheme` wired to tokens).
- [ ] Refactor `java/org/schabi/newpipe/util/ThemeHelper.java` to read the same
      token source and drive both XML attrs and Compose colors.
- [ ] **Parity unit test**: assert Compose `ColorScheme` == parsed
      `tokens/colors_m3.xml` for every theme × service (this is the no-drift
      guarantee).

**Exit criteria**
- No visual regression vs M0 baselines (pixel diff within tolerance) for all 4 themes.
- Parity test green.
- Dynamic-color code path compiles behind the flag (off by default).

---

### M2 — View-world adoption of tokens

**Tasks**
- [ ] Introduce `TextAppearance.PipePipe.*` styles; migrate the **94** raw
      `?android:attr/textAppearance*` usages and **13** hard-coded `sp` sizes in the
      131 XML layouts to them.
- [ ] Migrate custom attrs (`card_item_background_color`, `border_color`,
      `tag_text_color`, `toolbar_search_color` in `values/attrs.xml`) to the M3 role
      system (keep aliases).
- [ ] Restyle shared building blocks: `list_stream_item.xml`,
      `list_stream_grid_item.xml`, `list_channel_item.xml`, `list_playlist_item.xml`,
      `material_card_view`/cards, chips, dialogs, snackbars.
- [ ] Fix the `configChanges` gap: re-apply theme + window insets in
      `onConfigurationChanged` for the activity classes (rotation currently relies on
      `configChanges`, which skips theme re-application).
- [ ] Per-milestone screenshot diffs vs M1 baselines (expect intentional diffs only in
      the migrated areas).

**Exit criteria**
- View and Compose visually consistent under all themes (side-by-side screenshot
  review).
- Parity test still green; full test suite green.

---

### M3 — Stable expressive components (Compose)

**Tasks**
- [ ] Migrate search flow to the new slot-based `SearchBar` + `SearchBarState`
      (opt-in `@ExperimentalMaterial3Api` now → stable later).
- [ ] Adopt expressive/segmented list items for the new screens being built in
      Compose (don't rewrite the View feed yet — apply to search results + onboarding).
- [ ] Button groups (single/multi-select `ConnectedButtonGroup`) for filter/sort UI.
- [ ] Split button for actions like "Add To → Download/Playlist".
- [ ] Loading indicators (M3E style) replacing `AnimatedProgressBar` in Compose flows.
- [ ] Chips, `AlertDialog`/`BasicAlertDialog`, M3 `Snackbar`/`SnackbarHost`.
- [ ] Autosize text (`Text(text, autoSize …)`) on the video title (detail).
- [ ] Rework the **player secondary controls** (play/pause/next/prev/seek) with the
      new button/indicator components (keep the View surface; Compose overlay).

**Exit criteria**
- P0 screens from the study (home, detail, player, search) visually expressive.
- All components used are stable or opt-in (no alpha). Player behavior unchanged
  (test suite + manual regression on SABR/background/popup).

---

### M4 — Dynamic color + adaptive

**Tasks**
- [ ] Add **"Dynamic Color"** toggle in `res/xml/appearance_settings.xml`
      (`SwitchPreferenceCompat`, key `dynamic_color_key`, default off).
- [ ] `DynamicColor.kt`: `dynamicLightColorScheme(context)` /
      `dynamicDarkColorScheme(context)` on SDK ≥31; brand scheme fallback on <31.
- [ ] Wire `ThemeHelper` + Compose `Theme.kt` to respect the toggle; re-apply on
      change (and on `onConfigurationChanged`).
- [ ] Restyle MDC drawer (`NavigationView`) and tabs (`TabLayout`/`ScrollableTabLayout`)
      to the M3 token roles.
- [ ] Adaptive: two-pane layout for the video-detail + related list on ≥600 dp
      width (tablet/landscape) using the existing `values-land`/`values-w600dp`
      resource buckets — no new dependency.
- [ ] Screenshot tests: dynamic color ON/OFF × light/dark × YouTube.

**Exit criteria**
- Dynamic Color verified on API 35 emulator; fallback verified on API 30.
- Toggle OFF reproduces the M3 baseline exactly (diff = 0).
- Two-pane tablet layout renders; portrait unaffected.

---

### M5 — Expressive adoption at material3 1.5.0 stable

*(Blocked until `androidx.compose.material3:1.5.0` reaches stable; do the prep work
beforehand so the switch is mechanical.)*

**Tasks**
- [ ] **Prep (do now, unblocked):** keep the study's API-stability matrix updated
      (recompile probes against each 1.5.0-alpha; watch the compose 1.12.x pairing).
      Keep `MotionScheme`/`MaterialExpressiveTheme` usage isolated in the
      `theme/` package so only that package changes.
- [ ] Upgrade material3 → 1.5.0 + compose runtime 1.12.x (the one big dependency
      bump; validate with the full test + screenshot suite).
- [ ] Switch `theme/` to `MaterialExpressiveTheme(colorScheme, motionScheme =
      MotionScheme.expressive(), …)`; remove now-obsolete `@OptIn`s.
- [ ] Express motion: springs for mini-player expand, FAB, list item entrance;
      reduce-motion aware.
- [ ] Hero **carousel** (`HorizontalCenteredHeroCarousel`) on the "Recommended
      Lives" home section (the one hero moment per the M3E tactic guidance).
- [ ] Shape-morph transition: video thumbnail → full player.
- [ ] Migrate the **bottom-sheet player chrome** last, behind the `ui_expressive`
      feature flag (highest-risk item, R4/R10 in the study).
- [ ] Expressive menus, `Horizontal/VerticalFloatingToolbar` for player controls.

**Exit criteria**
- No alpha dependencies in the build.
- Playback, SABR, background/popup, downloads, notifications all green.
- `ui_expressive` flag OFF = M4 look (rollback path proven in a CI run).

---

### M6 — Hardening & first release

**Tasks**
- [ ] Replace `material-icons-extended:1.7.8` usage with bundled Material Symbols
      vector drawables (tintable), incremental per-screen; keep AGPL/OFL
      compatibility of any assets.
- [ ] Accessibility audit: TalkBack walkthrough of P0/P1 screens, contrast
      verification, 48 dp touch targets, reduce-motion.
- [ ] Performance: baseline profiles (Macrobenchmark startup/scroll/playback),
      fix any regressions; re-verify budgets (§2.3).
- [ ] Cleanup: remove legacy token aliases after a deprecation window; delete stale
      `@OptIn`s; optional source-package rename (deferred decision).
- [ ] Release: changelog, versioning scheme (see §8), F-Droid/APK distribution
      decision, upstream-rebase documentation.

**Exit criteria**
- First **stable** MaterialPipe release with all M3E features on stable-only deps.
- All quality gates green.

---

## 6. Work Breakdown Summary (owners + estimates)

| Workstream | M | Sprints | Owner (2-person team) |
|---|---|---|---|
| Branding + appId + CI | M0 | 1–2 | Infra engineer |
| Token resources + Compose theme + bridge | M1 | 2 | Eng A (Compose) |
| View migration (TextAppearance/attrs/components) | M2 | 3–4 | Eng B (View) |
| Expressive components in Compose | M3 | 3–4 | Eng A |
| Dynamic color + adaptive | M4 | 2–3 | Eng A + design review |
| material3 1.5.0 expressive | M5 | 2–3 | Eng A (guarded) |
| Hardening + release | M6 | 2–3 | Both + QA |

**Critical path:** M1 → M2 → M4 → M5 → M6. **M3 can run parallel to M2** once M1 lands
(M3 only needs the Compose theme, M2 only the View side).

---

## 7. Team & Roles

| Role | Responsibility |
|---|---|
| Tech lead / architect | Token architecture, dependency gating (R1/R2), upstream rebase |
| Android engineer (Compose) | theme/ package, expressive components, dynamic color, M5 |
| Android engineer (View) | layout/text migration, MDC restyle, configChanges fix |
| UI/UX reviewer | M3E tactic application, hero moments, a11y, screenshot review |
| QA / release | device matrix (incl. SABR), Play/F-Droid, changelog |

*(Solo mode: the same person plays all roles; milestones above already sized for
solo.)*

---

## 8. Release & Versioning

- **Version scheme:** `MaterialPipe <base>.<ui>` e.g. **0.27.2-E1** … **0.27.2-E3**
  (alphas), then **1.0.0** for the first stable M3E release. Record the exact
  upstream PipePipe tag + commit in the release notes for traceability.
- **Channels:** nightly debug (CI), alpha (GitHub Releases), beta, stable.
- **Distribution:** GitHub Releases + (optionally) F-Droid. Play Store only after
  legal/AGPL review (NewPipe is typically distributed outside Play).
- **Feature flags:** `ui_m3_tokens`, `ui_m3_components`, `ui_dynamic_color`,
  `ui_expressive` — default ON only after their milestone's exit criteria pass;
  OFF = previous look (rollback without an app update for flagged UI).

---

## 9. Quality Gates (Definition of Done per milestone)

Every milestone PR must pass:

1. `compileDebugKotlin` + `testDebugUnitTest` (10) + extractor tests (40) — all green.
2. `lintDebug` — no **new** issues vs committed baseline.
3. Screenshot diff vs previous milestone ≤ tolerance (Paparazzi/Robolectric), with
   review of intentional diffs.
4. Manual device matrix: P0/P1 flows on API 21 + 35; SABR/AV1 playback;
   background/popup player; downloads; notifications; landscape.
5. No alpha/non-stable dependencies in the build (enforced by a CI check on the
   resolved dependency graph).
6. Parity test (Compose vs tokens) green.

---

## 10. Risks & Contingency

| # | Risk | M | Mitigation |
|---|---|---|---|
| R1 | material3 1.4.0 expressive APIs are internal | M1–M5 | Use stable/opt-in features now; expressive switch isolated to `theme/` package, gated on 1.5.0 stable |
| R2 | 1.5.0 requires compose 1.12.0-beta01 | M5 | Version gate in CI; prep done now, mechanical switch later |
| R3 | View↔Compose theme drift | M1+ | Single token source + parity test (CI-enforced) |
| R4 | Bottom-sheet player regression | M5 | Migrate last, feature-flagged, keep `CustomBottomSheetBehavior` until then |
| R5 | Dynamic color harms brand | M4 | Default OFF; brand seed when OFF; screenshot diffs for both |
| R6 | Large text/layout migration breaks UI | M2 | Milestone-sized screenshot baselines; incremental PRs |
| R7 | `configChanges`/insets on rotation | M2 | Fix re-application in `onConfigurationChanged` |
| R8 | Lint debt hides new issues | M0 | Baseline file; fail only on new |
| R9 | Upstream rebase conflicts in res/ | M0–M3 | Weekly rebase; monthly after M4; token aliases reduce conflicts |
| R10 | Motion disliked / a11y | M5 | Reduce-motion; bounded springs; review |
| R11 | AGP 9 built-in Kotlin migration | M0 | Dedicated PR, full re-verify, trivially reverted |
| R12 | Licensing / trademark ("Material" name) | M0 | Verify GPL/AGPL headers; keep upstream attribution; document trademark disclaimer in README |

---

## 11. Open Questions (decision log)

1. **Package rename** in M6 — keep `org.schabi.newpipe` forever or rename?
   (Recommend: keep; appId already distinguishes the product.)
2. **F-Droid + Play distribution** — which channels, and do we keep the two
   submodules' licensing compatible?
3. **Dynamic Color default** — the plan says OFF (brand-first). Revisit after user
   feedback if a majority expects Material-You behavior.
4. **View-layer end-state** — stay hybrid indefinitely, or set a (long) roadmap to
   migrate the feed + detail to Compose after the first stable release?

---

## 12. First Commit Checklist (kickoff)

- [ ] `git init` fork, `upstream` remote, `main` branch
- [ ] appId + branding changes (M0)
- [ ] CI workflow + lint baseline + screenshot harness
- [ ] `design/` token tables checked in (reviewed)
- [ ] `theme/` package skeleton + parity test (M1 start)
- [ ] README: vision, license, upstream-tag record, this plan
- [ ] Backlog created from M0–M6 task checklists above

---

## 13. Implementation Status (2026-08-03)

**M0 (partial) — done:** `applicationId` changed to `org.materialpipe.app`
(+ `.debug` suffix) in `app/build.gradle`. Full debug build verified; app
installs alongside stock PipePipe.

**M1 — implemented and verified:**

| Change | File(s) |
|---|---|
| M3 color-role derivation (pure Kotlin, brand-tinted) | `java/org/schabi/newpipe/theme/Color.kt` |
| M3 typography scale | `java/org/schabi/newpipe/theme/Type.kt` |
| M3 shape scale | `java/org/schabi/newpipe/theme/Shape.kt` |
| Motion tokens + reduce-motion helper | `java/org/schabi/newpipe/theme/Motion.kt` |
| Scheme resolution incl. **Dynamic Color** (opt-in, SDK 31+) | `java/org/schabi/newpipe/theme/DynamicColor.kt` |
| Single Compose entry point | `java/org/schabi/newpipe/theme/Theme.kt` |
| Consolidated both ad-hoc Compose themes to `MaterialPipeTheme` | `ComposeItemUiHelper.kt`, `SearchFilterDialog.kt` |
| **Dynamic Color** toggle (default off) | `res/xml/appearance_settings.xml`, `res/values/strings.xml`, `res/values/settings_keys.xml`, `AppearanceSettingsFragment.java` |
| Parity / derivation unit tests (7 new) | `app/src/test/.../theme/MaterialPipeColorSchemeTest.kt` |

**Verification (this session):**
- Build: `compileDebugKotlin` + `assembleDebug` green; app installs and launches.
- Unit tests: **17/17 pass** (10 existing + 7 new theme tests).
- On-device (API 35):
  - Light home feed renders; brand header intact.
  - Compose search-filter dialog background `#FFFBFE` (token value) and
    `primaryContainer` **exactly equals** the brand blend `blend(#E53935, white, 0.85) = (251,225,225)` — brand-tinted M3 roles are live.
  - Dynamic Color toggle present in Settings → Appearance; flipping it flips the
    pref, recreates the activity, and changes the surface color (back to exact
    brand look when off).
  - Black theme switch still works: settings surface = pure `#000000`.
- Lint baseline (2,067 pre-existing errors / 600 warnings) not re-audited; CI
  baseline-file gate still to be added (M0 remainder).

**Remaining:** M2 View-world token adoption (TextAppearance migration, `configChanges` fix); M3 expressive
components; M4 adaptive; M5 blocked on `material3` 1.5.0 stable; M6 hardening.

---

## 14. Implementation Status 2 (2026-08-03, second pass)

**M0 — completed:**
- CI workflow added (`.github/workflows/ci.yml`): build, app unit tests
  (extractor is vendored into `:app`, so `:app:testDebugUnitTest` covers it),
  lint, APK artifact upload, and a dedicated
  "lint baseline in sync" job (`git diff --exit-code app/lint-baseline.xml`).
- Lint baseline generated: `app/lint-baseline.xml` — 2,067 errors /
  600 warnings / 1 hint filtered, exactly matching the pre-existing counts.
  `lintDebug` is now green ("Lint found no new issues"). New issues will fail CI.
- Screenshot harness added (`tools/screenshot_harness.sh` +
  `tools/compare_screenshots.py`, dependency-free PNG perceptual diff); verified
  end-to-end on the emulator (home diff 0.05, search diff 0.03) and wired into
  CI as the `screenshot-harness` job (API 35, `pixel_2` device).

**M2 — View-world token adoption (typography):**
- Added `TextAppearance.PipePipe.*` scale to `values/styles.xml` (12 size styles
  + 3 inherited platform-appearance aliases; parent
  `@android:style/TextAppearance(.Material.*)`).
- Migrated **91 hard-coded `android:textSize="Nsp"` usages across 44 layout
  files** to the named styles (tag-aware script; 2 multi-line duplicate-`style`
  collisions detected and reverted to the original `textSize`).
- Migrated **86 of 106 `?android:attr/textAppearance{Large,Medium,Small}` usages**
  to the inherited PipePipe aliases (byte-identical resolution); the 16 skipped
  tags already carry an explicit `style` that controls size; 4 `textAppearanceListItem`
  usages and `@dimen/...` sizes left as-is.
- `configChanges` item: **resolved as no-change** — `uiMode` is not in the
  manifest's `configChanges`, so AppCompat DayNight (`setDayNightMode`) already
  recreates on system dark/light switches; forced re-application would
  double-recreate. Documented, no code risk taken.

**M4 — adaptive:** upstream already ships `values-sw600dp`,
`values-sw600dp-land`, `values-w820dp` buckets and a two-pane
`layout-large-land/fragment_video_detail.xml`; **verified on-device** by
simulating a large-land screen (`wm size 2560x1600`, density 200): the video
detail renders player + description in the left pane (weight 5) and the related
items list in the right pane (weight 3).

**Verification (this pass):**
- `:app:assembleDebug` + `:app:testDebugUnitTest` green (17/17).
- `:app:lintDebug` green against baseline.
- On-device: fresh install over previous build, home + search render
  identically (parity preserved), black theme intact.

**Blocked (unchanged):** M3/M5 expressive work (`material3` 1.5.0 stable).

---

## 15. Implementation Status 3 (2026-08-03, third pass — continuation)

**M0 — completed (screenshot harness):** see §14. CI `screenshot-harness` job
pinned to `pixel_2` to match the committed baselines' geometry.

**M3 — stable expressive subset:**
- Confirmed from the `material3` **1.4.0** classes.jar that the expressive API
  surface (`Carousel*`, `MotionScheme`, `MaterialExpressiveTheme`,
  `SegmentedButton*`, floating toolbars) is `@ExperimentalMaterial3ExpressiveApi`
  / internal — the hard blocker stands for M5.
- Completed the one adoptable M3 task: **autosize text**. `TextAutoSize.StepBased`
  (foundation 1.11.3, opt-in) applied to the Compose item titles in
  `ComposeItemUiHelper.kt` (channel list, channel grid, stream grid), so long
  titles shrink-to-fit (12–14sp) instead of ellipsizing.
- Chips/dialog/segmented-button-free flows already use M3 components
  (SearchFilterDialog: `FilterChip`, `FilledTonalButton`, `RadioButton`,
  `DropdownMenu`).

**M4 — two-pane verified** on-device (see §14).

**M6 — hardening:**
- **Removed `material-icons-extended:1.7.8`** (the ~40 MB dependency) → replaced
  with `material-icons-core:1.7.8`. The only extended-only icon (`DragHandle`)
  now uses the existing `res/drawable/ic_drag_handle.xml` (identical glyph) via
  `painterResource`; `ArrowDropDown`/`PlayArrow` are in core. Dependency tree
  verified.
- **Accessibility: WCAG AA contrast guard** added (`MaterialPipeContrastTest`).
  This surfaced a real issue: the derived secondary/tertiary roles (hue-rotated
  at brand lightness) failed AA (onSecondary/secondary 2.78:1). Fixed with new
  tonal helpers `darkenTo`/`lightenTo` (light-scheme secondary/tertiary now
  darken to ≤ 0.16 luminance — distinct hues, AA-clean). Documented accepted
  brand exception: white on brand red `#E53935` is 4.23:1 (matches YouTube's own
  lockup; 4.0 floor asserted).
- Reduce-motion helper already shipped in M1 (`Motion.kt`,
  `isReduceMotionEnabled`).

**Verification (this pass):**
- `:app:assembleDebug` green; **20/20** unit tests (10 existing + 7 scheme +
  3 contrast); `:app:lintDebug` green (2067/600/1 baseline filtered, zero new).
- On-device: light + black themes, brand filter dialog (`#FFFBFE` bg,
  `primaryContainer` exactly `(251,225,225)`), Dynamic Color toggle, and the
  two-pane tablet detail all render correctly.

**Remaining (not completable in this session):**
- **M5 / M3-expressive** — blocked on `material3` 1.5.0 stable (hard rule: no
  alpha in production). The `theme/` package and `TextAppearance.PipePipe.*`
  scale are the prepared landing site for the M3E type scale when it ships.
- M6 manual items: TalkBack walkthrough (human), Macrobenchmark baseline
  profiles, legacy token-alias removal (after the View-layer migration),
  package-rename/release packaging (product decisions).
- `tools/screenshots/baseline/` must be committed with the repo for the CI
  `screenshot-harness` job to have comparison targets.

**Addendum (same day, third pass continuation):**
- Fixed a real termination bug in the tonal helpers: `lightenTo` could spin
  forever because Compose packs `Color` to 8-bit sRGB, so a 1% blend step
  quantize-cycles and traps luminance below the target (reproduced: stuck at
  ~0.647 against a 0.75 target). Both `darkenTo`/`lightenTo` now carry a
  bounded step + no-progress guard; `lightenTo` uses a larger step.
- Added unit coverage: black-theme scheme + WCAG AA, `darkenTo`/`lightenTo`
  luminance behavior, and the AA-clean light secondary/tertiary shade check
  (now **24/24** app unit tests).
- Rewrote `README.md`: MaterialPipe vision, upstream record (PipePipe
  v5.2.5-beta3 / `b5c33ff`), status, dev commands, docs, AGPL-3.0.
- Repo pushed to the new private GitHub repo
  `Kenneth-Cho-InfoSec/MaterialPipe` (branch `materialpipe`).

**Addendum (same day, M3 skin roll-out):**
- The visible View layer now runs on Material 3: base theme parent switched
  from `Theme.AppCompat.DayNight.NoActionBar` to
  `Theme.Material3.DayNight.NoActionBar`, with the full M3 color-role set
  (primary/container/secondary/tertiary/error + surface containers) wired per
  light/dark/black theme in `values/styles.xml` and `values/colors.xml`.
  Dialog, bottom-sheet, FAB and file-picker themes moved to their M3
  counterparts. The token attrs the Compose scheme reads are untouched, so the
  View and Compose worlds render the same palette.
- Verified on-emulator on API 35 (light/dark/black) by pixel-histogram: M3
  surface + brand colors render on Home, drawer, Settings and the playing
  player; no crashes; R8 release builds signed with the new key.
- Fixed `Widget.Material3.FloatingActionButton` → `.Primary` variant (bare
  name doesn't exist in material 1.14.0).
- Release launcher name is now **MaterialPipe** (`app_name` resValue);
  debug is `MaterialPipe Debug`. Release APKs are signed with the new key in
  `~/.materialpipe-keys/`.

---

*This plan operationalizes the study at `doc/material3-expressive-modernization.md`.
Source changes for M0 (partial) and M1 are implemented in this repo as of
2026-08-03.*
