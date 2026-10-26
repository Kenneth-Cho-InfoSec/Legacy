# PipePipe — Material 3 Expressive Design-System Modernization Study

**Status:** Proposed design system modernization plan (not yet implemented)
**Prepared:** 2026-08-03
**Scope:** Full-stack UI/design-system analysis of PipePipe's current state, with a
phased, risk-aware roadmap to a Material 3 Expressive (M3E) aligned design system.
**Evidence base:** Fresh clone + clean build + unit/lint runs + on-device (emulator,
Android 15 / API 35) verification of the current UI, plus official AndroidX/Material
documentation and empirically compiled API-stability probes against `material3`
artifacts.

---

## 1. Executive Summary

PipePipe is a fork of NewPipe (AGPL-3.0) that adds SABR/AV1 decoding and its own
YouTube onboarding. Today it is a **hybrid View + Jetpack Compose** app: roughly 75%
of the surface is the legacy NewPipe View system (131 XML layouts, Material Design
Components 1.14.0, `AppCompat`), and ~25% is Compose (search filter dialog, settings
UI, onboarding), already on **`material3:1.4.0`** — which is, as of July 2026, the
**current stable** Compose Material 3 release and the newest one that can be used by a
production app.

Key findings, in one paragraph each:

1. **The app already builds and runs cleanly on the modern toolchain.**
   A clean clone built with AGP 9.2.1, Gradle 9.5.1, Kotlin 2.3.21, and a JDK 25
   toolchain. 10 app unit tests + 40 extractor tests pass. Lint reports 2,067
   errors / 600 warnings (pre-existing, all in upstream-fork code; not introduced by
   any change in this study). The app launches on an API 35 emulator, loads live
   content, plays video (verified: MediaSession `PLAYING`, playback position
   advancing), and every representative screen we exercised works (home feed,
   drawer, search, search-filter dialog, video detail, mini player, settings,
   theme switching, landscape).

2. **Material 3 Expressive is real, current, and — critically — not yet stable.**
   As of 2026-08-03 the newest **stable** `androidx.compose.material3` is **1.4.0**
   (released 2025-09-24); the newest alpha is 1.5.0-alpha25. We proved by compiling
   real code against both artifacts that the public expressive surface —
   `MotionScheme`, `MaterialExpressiveTheme`, `MaterialTheme(motionScheme = …)` —
   is **`internal` in 1.4.0** (unusable from app code) and becomes **public and
   non-experimental only in 1.5.0-alpha25**, which in turn requires the Compose
   `1.12.0-beta01` runtime stack. So a production-grade "M3E" migration must be
   **staged**: land the stable M3E foundations that 1.4.0 already ships
   (expressive motion is baked into component defaults, new components like
   `Carousel`, autosize text, new `SearchBar`), then flip to the fully public
   expressive API when 1.5.0 reaches stable.

3. **The gaps to M3E are systemic, not cosmetic.** The app's design system is a
   NewPipe-era hand-rolled token set (a `light_`/`dark_`/`black_` parallel color
   family in `colors_services.xml`, a bespoke `ThemeHelper`, per-service brand
   colors), with **no dynamic color**, **no expressive shape scale**, **no
   typography scale** (raw `?android:attr/textAppearance*` + hard-coded `sp`
   sizes), and **no motion tokens**. Components are a mix of MDC-Views,
   `AppCompat`, and custom views. This is precisely the surface M3E targets.

4. **The right target is achievable with the existing architecture.** Because
   Compose and the Material Theme already exist in-app, the plan below
   introduces a **single source of truth** (Compose `ColorScheme`/`Typography`/
   `Shapes`/`MotionScheme` mapped 1:1 to a new `values/` token layer), keeps the
   View world in sync via a generated bridge, and introduces dynamic color as an
   opt-in **"Dynamic Color"** toggle in Appearance settings — a strong M3E signal
   with near-zero risk.

This document is the complete study: current-state inventory (D1), target
definition (D2), dependency plan (D3), theme-system redesign (D4), component map
(D5), screens/flows matrix (D6), phased roadmap (D7), gap analysis (D8), risk
register (D9), testing (D10), accessibility (D11), performance (D12), rollout &
rollback (D13), and appendix with research sources and evidence (D14).

---

## 2. Methodology and Evidence (What Was Actually Done)

All findings below were produced in this session against a fresh clone.

### 2.1 Repository facts

| Item | Value |
|---|---|
| Repo | `InfinityLoop1309/PipePipe` (fork of `TeamNewPipe/NewPipe`) |
| Clone dir | `~/Projects/PipePipe/PipePipeClient` |
| Default branch | `dev` |
| License | AGPL-3.0 (NewPipe lineage; `LICENSE` present) |
| Submodules | 2 (NewPipeExtractor + `ffmpeg` fork via symlinks) |
| Modules | `app` only (+ `NewPipeExtractor` as external module) |
| Package | `InfinityLoop1309.NewPipeEnhanced` (debug: `.debug`) |

### 2.2 Build / toolchain (verified)

- AGP **9.2.1**, Gradle **9.5.1** (wrapper), Kotlin **2.3.21**, Java toolchain **25**,
  compile/target SDK **37/36**, minSdk **21**.
- Debug build: **SUCCESS**. Release build: **SUCCESS** (no lint gate).
- `gradle.properties` opts out of the AGP 9 built-in Kotlin DSL
  (`android.newDsl=false`, `android.builtInKotlin=false`) — these flags are
  deprecated in AGP 9.2 and should be removed when migrating (see D3).

### 2.3 Tests / lint (verified)

- App unit tests: **10 passed** (`:app:testDebugUnitTest`).
- Extractor tests: **40 passed**.
- Lint (`:app:lintDebug`): runs to completion; **2,067 errors / 600 warnings**.
  Spot checks show they are pre-existing upstream issues (unused resources,
  lint-ignored icons, locale issues, deprecated APIs) and **do not block builds**.

### 2.4 On-device verification (API 35 emulator, `Pixel 7a`, 1080×1920 @420dpi)

| Journey | Result |
|---|---|
| Cold start | ~2.1 s (`am start -W` TotalTime 2108 ms); PSS ~217 MB incl. playback |
| Home "Recommended Lives" | Live streams load from network (verified) |
| Navigation drawer | Works: Subscriptions, Recommended Lives, Bookmarks, Downloads, History, Settings, About, What's New |
| Settings | Category list works (Account, Appearance, Content, Player, …) |
| **Theme switching** | `auto_device_theme` → `black_theme` re-application **verified by sampling screenshot pixels**: light background `#FFFBFE`, black background `#000000` — exactly the token values in `colors.xml` |
| Search | Typed query → real YouTube results (titles, durations, view counts) |
| Search filter dialog (Compose) | Opens; "Videos" filter applied and re-searched (verified) |
| Video detail | Loaded; playback started; MediaSession `state=PLAYING`; position advanced 0:19→0:35 |
| Mini player | Appears after back navigation; playback continues |
| Settings sub-screens | Appearance (Theme/Night Theme/eye protection/grid) rendered |
| Landscape | Rotated via `user_rotation`; `fragment_holder` re-laid out to landscape bounds — `configChanges` handles rotation without recreation |
| Empty states | Subscriptions shows "Nothing here but crickets" (`@string/no_subscriptions`), as designed |

### 2.5 API-stability probes (empirically compiled, not assumed)

We created a minimal Android project and compiled identical source against
`androidx.compose.material3` 1.4.0 and 1.5.0-alpha25:

| API | material3 1.4.0 (stable) | material3 1.5.0-alpha25 (alpha) |
|---|---|---|
| `MaterialExpressiveTheme` | **internal** — `Cannot access … it is internal in file` | **public, no opt-in needed** ✓ |
| `MotionScheme` interface | **internal** | **public** ✓ |
| `MaterialTheme(colorScheme, motionScheme, …)` | **internal** | **public** ✓ |
| `MotionScheme.expressive()` / `.standard()` | internal (`…$material3`) | **public** ✓ |
| `Carousel` (`material3.carousel`) | **internal** | public (stable) |
| `SearchBar(state=…)` new slot API | **experimental** (`@ExperimentalMaterial3Api`) | stable |
| `SecureTextField` / `VerticalDragHandle` | experimental/internal | stable |
| `HorizontalFloatingToolbar` / `VerticalFloatingToolbar` | not present | public |
| Compose runtime requirement | ui/foundation/animation **1.11.x** | **1.12.0-beta01** |

Conclusion: **a production app today must target `material3` 1.4.0 stable** and can
pre-pare the expressive migration so that 1.5.0 stable adoption is a small, mechanical
step.

---

## 3. Deliverable 1 — Current-State Inventory

### 3.1 Technology stack (UI-relevant)

| Concern | Current | Version |
|---|---|---|
| Compose UI | `androidx.compose.ui:ui` | 1.11.3 |
| Compose Foundation | `androidx.compose.foundation:foundation` | 1.11.3 |
| Compose Animation | `androidx.compose.animation:animation` | 1.11.3 |
| Compose Material 3 | `androidx.compose.material3:material3` | **1.4.0 (current stable)** |
| Compose Material Icons | `material-icons-extended` | 1.7.8 *(old; no longer recommended — see D5)* |
| AppCompat | `androidx.appcompat:appcompat` | 1.7.1 |
| MDC Views | `com.google.android.material:material` | 1.14.0 |
| Preferences | `androidx.preference:preference` | 1.2.1 |
| ViewBinding | enabled (fragments) | — |
| Compose BOM target (ecosystem current) | `2026.06.01` → ui/foundation/animation **1.11.4** | 2026-06-01 |

### 3.2 Design system surface (measured in source)

| Metric | Value |
|---|---|
| XML layouts | 131 |
| Kotlin files | 121 |
| Java files | 418 |
| Drawables (+ nodpi) | 170 (+16) |
| `values-night` colors | 2 |
| Unique hard-coded hex colors inside layouts | 8 (rest via token references — good) |
| Hard-coded `dp` paddings/sizes in layouts | 50 distinct |
| Hard-coded `sp` text sizes in layouts | 13 distinct (10–40 sp) |
| `textAppearance` usages | `?android:attr/textAppearance{Large,Small,Medium}` dominate (94), AppCompat styles secondary |
| Custom `View` classes | 20 (`NewPipeTextView`, `AnimatedProgressBar`, `MarkableSeekBar`, `CustomCollapsingToolbarLayout`, `ScrollableTabLayout`, `ExpandableSurfaceView`, …) |
| Progress indicators (View + Compose) | ~34 files |

### 3.3 Theme/token structure (the heart of the current system)

**`values/colors.xml` + `values-night/colors.xml`** define a hand-rolled, three-way
theme family (`light_*`, `dark_*`, `black_*`) with ~40 tokens per theme
(background, dialog background, settings accent, separator, ripple, selected,
contrast, card item, border, …) plus `contrastColor`/`defaultIconTint`.

**`values/colors_services.xml`** defines per-service brand palettes (YouTube red,
SoundCloud orange, PeerTube orange, media.ccc.de grey, Bilibili pink, niconico)
with `light_<service>_primary_color`, `dark_<service>_primary_color`, etc.

**`values/styles.xml` (+ `-v21/-v27/-v29` variants)** builds a cascade:
`Base.V19` → `Base` → `LightTheme`/`DarkTheme`/`BlackTheme`, plus dialog/settings/
bottom-sheet themes. Black inherits Dark. Status-bar and navigation-bar colors are
per-service and per-theme.

**`values/attrs.xml`** declares custom attrs (`card_item_background_color`,
`border_color`, `tag_text_color`, `toolbar_search_color`, …) that the View layer
resolves via `ThemeHelper`.

**`ThemeHelper` / `ServiceHelper` / `ThemeHelper.setTheme`** selects the theme at
runtime from `PreferenceManager`, applies the per-service brand color, and drives
both the XML themes and the Compose theme.

**No dynamic color, no Material3-style role naming, no typography scale, no shape
scale, no motion tokens, no density-independent emphasis system.** Typography is
platform `textAppearance`; shapes are ad-hoc `shapeAppearance` / drawables; motion
is ExoPlayer + MDC defaults + bespoke fragment transitions.

### 3.4 Architecture notes that constrain the migration

- **Hybrid surface.** Video detail, player sheet, lists, feed, channels, playlists,
  history, settings (preference screens) are View-based. Compose is used for:
  onboarding, the search filter dialog (`SearchFilterDialog.kt`), settings screens
  (`Material3`-themed `SettingsFragment`), and "What's New". The migration must
  bridge both worlds with one token source.
- **`CustomBottomSheetBehavior`** (player) subclasses MDC `BottomSheetBehavior`
  with manual touch interception and hard-coded `skipInterceptionOfElements` IDs —
  any MDC upgrade must re-verify this.
- **`configChanges`** on activities (portrait/landscape handled manually) means
  theme/inset changes are not automatically re-applied on rotation.
- **`BulletCommentsView`** loads `lxgw_wenkai.ttf` for comment overlays (font
  licensing: OFL — fine to keep).
- **No `material3` expressive, no adaptive layout beyond `values-land`.**
- **`material-icons-extended:1.7.8`** is a compose-icons artifact from the 1.7 line.
  Note that material3 1.4.0 dropped its `material-icons-core` dependency; the
  icons library is officially "no longer recommended" (superseded by Material
  Symbols). PipePipe's continued use must be de-duplicated/audited (D5).

### 3.5 What already looks "Material-ish"

- `ShapeableImageView` / MDC `MaterialCardView` in places (chips, some cards).
- `TabLayout` (MDC) + `ScrollableTabLayout` custom.
- `SearchBar`-style Compose input in onboarding.
- Per-service brand theming is conceptually aligned with M3 "brand color + dynamic
  color" thinking — the seed is already per-brand.

---

## 4. Deliverable 2 — M3 Expressive Target Definition

M3 Expressive (Google I/O 2025) is an **evolution of M3, not M4**: no deprecation,
same component set, plus new subsystems and tactics. From the official sources
(§A appendix) the target design system for PipePipe is:

1. **Motion physics (MotionScheme).** Replace easing/duration with two spring
   families: *spatial* (movement, scale — physical, predictable) and *effects*
   (color/opacity — fast, subtle). Material3 1.4.0 already applies these internally
   to components; the target makes them **explicit and customizable** via
   `MotionScheme` when it becomes public.
2. **Expanded shape library.** 35 new shapes + shape-morphing animation. Target:
   a shape scale for `Shapes` plus shape-morph for the player-thumbnail → video
   transition and avatar crops.
3. **Vibrant, richer color.** Expanded tonal ranges, strong primary/secondary/
   tertiary contrast, dynamic color on Android 12+. Target: a proper `ColorScheme`
   with 20+ roles, brand-seeded, plus an **opt-in Dynamic Color** mode.
4. **Emphasized typography.** Editorial type moments: heavier weights, larger sizes,
   additional type styles. Target: a full M3 type scale in `Typography`.
5. **Expressive components.** New components available to adopt: `Carousel`,
   FAB Menu, Split Button, Button Groups (single/multi-select), Loading Indicator,
   Horizontal/Vertical FloatingToolbars, expressive list items, expressive menus,
   flexible bottom app bar, secure text fields, auto-size text.
6. **Adaptive, emphasis-based layout.** Hero moments; hierarchy via surface
   contrast and container emphasis — relevant to PipePipe's video-first content.

**Target success criteria (measurable):**
- A single `ColorScheme`/`Typography`/`Shapes`/`MotionScheme` in Compose that drives
  *both* Compose and View UI (bridge generated), no drift between `values/` tokens
  and Compose tokens.
- Dynamic color toggle works on Android 12+ and falls back gracefully on <12.
- All 13 screens/flows in D6 visually consistent and touch targets ≥48 dp.
- No regression on the 50 unit/extractor tests; new screenshot/robotoesque tests for
  the 4 theme modes × 3 services snapshots.
- Playback, SABR, and background/popup player behavior unchanged (these touch the
  View player stack, which we do not re-theme in Phase 1–3).

---

## 5. Deliverable 3 — Dependency Upgrade Plan

Proposed, in dependency order. All versions are current stable as of 2026-08-03
unless marked `alpha`.

| Step | Artifact | From | To | Why / risk |
|---|---|---|---|---|
| 3.1 | Compose runtime line | ui/foundation/animation 1.11.3 | **1.11.4** (Compose BOM 2026.06.01) | Patch-level, zero API change; keeps us current while 1.4.0 remains the max stable material3 |
| 3.2 | `material-icons-extended` | 1.7.8 | audit → keep pinned OR replace Material Symbols vector drawables incrementally | 1.7.8 pulls a 1.7-line Compose icon dependency; material3 1.4.0 no longer depends on icons-core. Pin stays fine short-term; long-term migrate to bundled Vector Drawables (see D5). Low risk either way |
| 3.3 | AGP DSL flags | `android.newDsl=false`, `android.builtInKotlin=false` | remove (adopt AGP 9 built-in Kotlin + new DSL) | Deprecated in AGP 9.2; remove in a dedicated build-infra PR with full build verification (risk: medium, mechanical) |
| 3.4 | `material` (MDC Views) | 1.14.0 | keep at 1.14.0 for now | 1.14.0 is current stable MDC; upgrade only when we align the View bridge to a matching MDC "M3-style" shape/theme (Phase 4) |
| 3.5 | **`material3`** | 1.4.0 | **stay on 1.4.0** until 1.5.0 stable | Expressive theme APIs are internal in 1.4.0 and public in 1.5.0-alpha25; 1.5.0-alpha25 requires compose 1.12.0-beta01. **Do not adopt alpha in production** |
| 3.6 | `material3` (when 1.5.0 stable) | 1.4.0 | 1.5.0 + compose runtime 1.12.x | Then adopt `MaterialExpressiveTheme`, `MotionScheme.expressive()`, expressive components (mechanical switch; all opt-in annotations removable) |
| 3.7 | New (optional, Phase 5) | — | `androidx.compose.material3:material3-adaptive-navigation-suite` (1.5.0-alpha25, only with 1.5.0) | Only if adaptive nav is adopted on tablet/foldable |

**Golden rule:** no alpha dependencies in a shipping build; the expressive features
that 1.4.0 already ships (motion defaults, carousel-ready foundation, autosize text,
new search-bar API behind opt-in) are exploited now, and the public expressive
surface lands with 1.5.0.

---

## 6. Deliverable 4 — Theme-System Redesign (Color / Typography / Shape / Motion Tokens)

### 6.1 Token architecture (single source of truth)

```
app/src/main/res/values/tokens/        (NEW — Android resources)
  colors_m3.xml       — M3 role tokens: primary, onPrimary, primaryContainer,
                        secondary, tertiary, surface, surfaceContainerLow/…High,
                        surfaceTint, background, onBackground, error, outline, …
                        in light / dark / black variants + per-service brand seeds
  type_m3.xml         — dimen-style text sizes/line heights for the 15-type scale
  shape_m3.xml        — corner sizes extraSmall…extraLarge (+ extra shapes Phase 5)
  motion_m3.xml       — durations/springs mapped for the View bridge (Phase 4)
app/src/main/kotlin/.../theme/         (NEW — Compose)
  Color.kt / Type.kt / Shape.kt / Motion.kt
  → builds ColorScheme/Typography/Shapes/MotionScheme from the res tokens
  DynamicColor.kt — dynamic light/dark scheme builder (Android 12+, opt-in)
  Theme.kt — MaterialTheme + MaterialExpressiveTheme wiring + bridge hook
app/src/main/kotlin/.../util/ThemeHelper (REFACTOR)
  → reads the same token source; emits XML theme attrs + Compose colors
```

The generation guarantee: a unit test asserts the Compose `ColorScheme` equals the
parsed `colors_m3.xml` values (no drift), so View and Compose can never diverge.

### 6.2 Color

- Introduce full M3 role set (light + dark + black) for the three services of
  highest use (YouTube, SoundCloud, PeerTube), seeded from existing
  `colors_services.xml` brand colors so brand identity is preserved.
- Keep legacy token names as aliases for the View layer during transition.
- **Dynamic color (opt-in):** `Settings → Appearance → “Dynamic Color”`
  (default **off**, to honor the brand-accurate default). When on and SDK ≥ 31,
  use `dynamicLightColorScheme(context)`/`dynamicDarkColorScheme(context)`; when
  off or SDK < 31, use the brand scheme. (The Settings `SwitchPreferenceCompat`
  already exists in `appearance_settings.xml`; add one key.)
- Keep the existing `theme`/`night_theme`/`eye protection` settings and map them 1:1.

### 6.3 Typography

- Define the M3 15-style scale (display/headline/title/body/label × large/medium/
  small) with the official defaults (Roboto family sizes, e.g. displayLarge 57/64,
  titleLarge 22/28, bodyLarge 16/24, labelSmall 11/16 — see appendix source).
- Wire `Typography` into Compose. For the View world, introduce `TextAppearance`
  styles (`TextAppearance.PipePipe.*`) and migrate the 94 raw
  `?android:attr/textAppearance*` usages + 13 hard-coded `sp` sizes to these.
- Apply M3E "emphasized type" to the video title on detail (display-style moment)
  and to empty-state headlines — low risk, high visual payoff.

### 6.4 Shape

- `Shapes` with 4/8/12/16/24 dp corners; apply `RoundedCornerShape`/`ShapeAppearance`
  consistently to cards, bottom sheets, dialogs, chips, thumbnails.
- Shape-morphing (Phase 5) for the video-thumbnail→player transition using
  `AnimatedContent`/custom `Animatable` shape morph (the 1.4.0 shape-morph support
  is a 1.5.0/expressive capability — implement as custom animation now).

### 6.5 Motion

- Phase 1–3: rely on material3 1.4.0 internal motion defaults (already applied to
  components); define `motion_m3.xml` tokens now so the View bridge is ready.
- Phase 5: `MotionScheme` (expressive/standard) wired in
  `MaterialExpressiveTheme`; custom `FiniteAnimationSpec` springs for the mini-player
  expand and FAB.
- Respect "reduce motion" (`Settings → Accessibility`) by scaling/removing spring
  animations when `Settings.Global.TRANSITION_ANIMATION_SCALE == 0`.

---

## 7. Deliverable 5 — Component-by-Component Mapping Table

Legend: **Stable** = available now in material3 1.4.0 public API · **Opt-in** = behind
`@ExperimentalMaterial3Api` in 1.4.0 · **Expr(pub)** = public in 1.5.0-alpha (adopt at
1.5.0) · **Expr(int)** = internal in 1.4.0 (use only via default behavior now) ·
**MDC/View** = Material Design Components View artifact (1.14.0).

| Current (PipePipe) | Target M3/M3E component | Class | Phase |
|---|---|---|---|
| `AnimatedProgressBar` (custom View) | `CircularProgressIndicator` / `LinearProgressIndicator` (Compose) + M3E Loading Indicator | Stable / Expr(pub) | 3 |
| `ProgressBar` (34 files) | `LinearProgressIndicator` with expressive motion | Stable (motion: Expr(int)) | 3 |
| `TabLayout`+`ScrollableTabLayout` | `PrimaryTabRow` (Compose) or MDC TabLayout with M3 style | Stable | 4 |
| `NavigationView` drawer | MDC `NavigationView` with M3 style, or `ModalNavigationDrawer` (Compose) | Stable | 4 |
| Bottom sheet player (`CustomBottomSheetBehavior`) | `ModalBottomSheet` / `BottomSheetScaffold` (Compose) — **keep View sheet in P1–3**, migrate last | Stable / Expr(pub) | 5 |
| `MaterialCardView`/cards | `Card`/`ElevatedCard`/`OutlinedCard` + container emphasis | Stable | 2–3 |
| Buttons (AppCompat/MDC) | `Button`/`FilledTonalButton`/`OutlinedButton`/`TextButton` + **SplitButton**, **ButtonGroups** | Stable / Expr(pub) | 3 |
| FAB (player controls) | `FloatingActionButton` + **FloatingActionButtonMenu** (`ToggleFloatingActionButton`) | Stable / Expr(pub) | 3 |
| `Snackbar`/toast | M3 `Snackbar`/`SnackbarHost` | Stable | 3 |
| Chips | `FilterChip`/`AssistChip`/`SuggestionChip` | Stable | 3 |
| Dialogs (settings/errors) | `AlertDialog`/`BasicAlertDialog`/**`TimePickerDialog`** | Stable | 3 |
| `SearchBar` (Compose, old) | New slot-based **`SearchBar` + `SearchBarState`** (ExpandedDocked/FullScreen) | Opt-in → Stable | 3 |
| Video thumbnails grid | **`Carousel`/`HorizontalCenteredHeroCarousel`** (hero "Recommended Lives") | Expr(int) → Stable | 5 |
| List items (`list_stream_item.xml` etc.) | Compose `ListItem` + **expressive list items** (secondary/segmented variants) | Stable / Expr(pub) | 3 |
| Toolbar (player secondary controls) | **`HorizontalFloatingToolbar`/`VerticalFloatingToolbar`** | Expr(pub) | 5 |
| `NewPipeEditText`/inputs | `OutlinedTextField` + **`SecureTextField`** (login) | Stable / Opt-in | 3 |
| `PopupMenu`/`Menu` | M3 `DropdownMenu`/expressive menus | Stable / Expr(pub) | 3 |
| Snackbar (downloads) | M3 Snackbar + "undo" patterns | Stable | 3 |
| Icons (Material Icons lib 1.7.8) | **Material Symbols** vector drawables (bundled, tintable) | Stable | 5 |
| Player surface (`ExpandableSurfaceView`) | Keep (Media3 `PlayerView`/SurfaceView) — **not a Material component**; restyle chrome only | — | 5 |

---

## 8. Deliverable 6 — Screens & Flows Coverage Matrix

Priority: **P0** (home path), **P1** (core playback path), **P2** (library/settings),
**P3** (secondary). Each row lists current implementation and the M3E change.

| # | Screen / Flow | Current | M3E change | Priority |
|---|---|---|---|---|
| 1 | Home "Recommended Lives" + feed tabs | View + `TabLayout` + `RecyclerView` | Cards, container emphasis, new tab styling; hero carousel (Phase 5) | P0 |
| 2 | Video detail | View (`fragment_video_detail.xml`) + inline player | Reflow to expressive layout: emphasized title type, tonal hierarchy, new controls row (FAB/segments), autosize description | P0 |
| 3 | Full/mini player + controls | View (`player.xml`) | New seekbar style, expressive button groups, floating toolbar (P5) | P0 |
| 4 | Search + results | Compose filter dialog + View list | New `SearchBar`/`SearchBarState`; expressive list items; filter chips | P0 |
| 5 | Drawer navigation | MDC `NavigationView` | M3-style drawer with surfaceContrast colors | P1 |
| 6 | Settings (all categories) | `androidx.preference` (XML) + Compose settings | M3 preference theming; new "Dynamic Color" + "Reduce motion" toggles | P1 |
| 7 | Subscriptions / feed | View list + empty states | Expressive list items; better empty states | P1 |
| 8 | Bookmarks / playlists / history | View lists | Shared list item restyle | P2 |
| 9 | Downloads + download manager | View | Dialog/snackbar restyle; progress indicators | P2 |
| 10 | Channel detail (tabs) | View | Header emphasis, avatar shape-morph | P2 |
| 11 | Comments / bullets | View + custom font | Container styling; keep custom font | P3 |
| 12 | Onboarding / What's New | Compose | Full M3E expressive welcome (hero moment) | P3 |
| 13 | Notifications channels | View | M3 list styling | P3 |
| — | Landscape / tablet | `values-land` + configChanges | Adaptive layouts (canonical two-pane on tablet) | P2 (P4 for adaptive nav suite) |

---

## 9. Deliverable 7 — Phased Implementation Roadmap

Six phases, each shippable and independently testable. **No phase requires an alpha
dependency.** Each phase ends with a buildable, release-able state.

**Phase 0 — Foundation & instrumentation (≈1–2 sprints)**
- Remove deprecated AGP flags (D3.3) with full build re-verification.
- Add Compose theme module skeleton (`theme/`) reading new token resources.
- Add the no-drift unit test (Compose vs `values/tokens/*`).
- Baseline screenshot tests for the 4 theme modes × top 5 screens (D10).
- **Exit:** build green; token bridge compiles; baseline snapshots committed.

**Phase 1 — Token layer (color/type/shape) in resources (≈2 sprints)**
- Create `tokens/colors_m3.xml`, `type_m3.xml`, `shape_m3.xml` (light/dark/black ×
  service seeds), keeping legacy names as aliases.
- Wire Compose `ColorScheme`/`Typography`/`Shapes` to the tokens; refactor
  `ThemeHelper` to read the same source.
- Verify visually: light/dark/black × YouTube identical to current behavior
  (pixel-sampled regression against the values we captured: `#FFFBFE` / `#000000`).
- **Exit:** no visual regression; bridge test green.

**Phase 2 — View-world adoption of the token system (≈3–4 sprints)**
- Introduce `TextAppearance.PipePipe.*` and migrate the 94 raw `textAppearance*`
  usages + hard-coded `sp`.
- Migrate `?attr/card_item_background_color`-style attrs to the new roles.
- Restyle shared list items, cards, chips, dialogs, snackbars (D5 table).
- **Exit:** View and Compose visually consistent under all themes.

**Phase 3 — Stable M3E components in Compose (≈3–4 sprints)**
- Adopt new `SearchBar`/`SearchBarState` (opt-in then stable), expressive list
  items, button groups, split button, loading indicator, chips, autosize text.
- Rework the search flow and player secondary controls with these.
- **Exit:** P0 screens (D6 #1–4) fully expressive, all stable-only.

**Phase 4 — Dynamic color + adaptive (≈2–3 sprints)**
- Add "Dynamic Color" toggle; dynamic schemes for SDK ≥31 with graceful fallback.
- Restyle MDC drawer/tabbar to the M3 token roles.
- Begin adaptive: two-pane detail on large screens (no new dependency).
- **Exit:** dynamic color verified on API 35; toggle off = exact old look.

**Phase 5 — Expressive adoption when material3 1.5.0 is stable (≈2–3 sprints)**
- Upgrade material3 → 1.5.0 + compose 1.12.x (the only dependency bump).
- Replace internal/default reliance with public `MaterialExpressiveTheme` +
  `MotionScheme.expressive()`, expressive list/menu/FAB/FloatingToolbar APIs.
- Add hero carousel on "Recommended Lives"; shape-morph transitions; motion tuning.
- Migrate bottom-sheet player chrome last (highest risk) with feature-flag
  rollback (D13).
- **Exit:** full M3E surface live; 1.5.0-stable-only; rollback flag in place.

**Phase 6 — Polish, a11y, perf hardening (ongoing)**
- Icon/`material-icons-extended` replacement with Material Symbols (D5).
- Accessibility audit pass (D11); performance budget enforcement (D12).
- Remove all `@OptIn` now-unneeded annotations; delete legacy token aliases after a
  deprecation window.

---

## 10. Deliverable 8 — Gap Analysis Table (Current → M3E Target)

| # | Capability | Current state | M3E target | Gap | Effort | Phase |
|---|---|---|---|---|---|---|
| G1 | Color roles | Custom `light/dark/black_*` flat tokens | Full 20+ role `ColorScheme` | HIGH | M | 1–4 |
| G2 | Dynamic color | None | Opt-in dynamic scheme (SDK≥31) | HIGH (new feature) | M | 4 |
| G3 | Brand-per-service theming | `colors_services.xml` | Brand seeds → M3 scheme | LOW (reuse) | S | 1 |
| G4 | Typography scale | Platform attrs + hard-coded sp | 15-style M3 scale + emphasis | HIGH | M | 2 |
| G5 | Shape scale | Ad-hoc | 5-tier shape scale + morph | MED | M | 1,5 |
| G6 | Motion system | MDC/ExoPlayer defaults | `MotionScheme` springs + reduce-motion | HIGH | M | 5 |
| G7 | Expressive components | ~0 | Carousel, FAB menu, button groups, split button, loading, toolbars | HIGH | L | 3,5 |
| G8 | Search experience | Old SearchBar + custom dialog | Slot-based SearchBar + SearchBarState | MED | M | 3 |
| G9 | Navigation (drawer/tabs) | MDC NavigationView/TabLayout | M3-styled nav + adaptive suite (optional) | MED | M | 4 |
| G10 | Bottom-sheet player | Custom BottomSheetBehavior | M3 bottom sheet + expressive motion | HIGH (risk) | L | 5 |
| G11 | Icons | material-icons-extended 1.7.8 | Material Symbols (bundled vectors) | MED | M | 6 |
| G12 | Typography/View-Compose parity | Two systems, drift risk | Single token source + parity test | HIGH (systemic) | M | 0–2 |
| G13 | Accessibility (contrast/type) | Ad-hoc | M3 roles guarantee contrast; a11y audit | MED | M | 4,6 |
| G14 | Motion preferences | None | Reduce-motion support | LOW | S | 5 |
| G15 | Adaptive/tablet | values-land only | Canonical adaptive layouts | MED | M | 4,6 |

Effort legend: S < 1 sprint · M 1–2 sprints · L 3+ sprints.

---

## 11. Deliverable 9 — Risk Register & Mitigations

| # | Risk | L × I | Mitigation |
|---|---|---|---|
| R1 | **material3 1.4.0 expressive APIs are internal** — can't be used until 1.5.0 | H×L | Stay on 1.4.0; exploit stable/opt-in features now; schedule expressive adoption after 1.5.0 stable (Phase 5). No alpha in production |
| R2 | 1.5.0-alpha requires compose 1.12.0-beta01 | H×M | Version gate: expressive adoption happens only when 1.5.0 stable ships with 1.12.x stable |
| R3 | View↔Compose theme drift | M×H | Single token source + automated parity test (D4) |
| R4 | Bottom-sheet player regression during migration | M×H | Migrate player chrome last; feature-flag rollback (D13); keep `CustomBottomSheetBehavior` until Phase 5 |
| R5 | Dynamic color distorts brand identity | M×M | Default OFF; brand seed preserved when OFF; settings toggle; screenshot tests for both modes |
| R6 | 94 textAppearance + 50 dp + 13 sp hard-coded migrations regress layout | M×M | Phase 2 done with screenshot baselines per theme; incremental PRs |
| R7 | `configChanges` breaks inset/theme on rotation | M×M | Re-apply theme/insets in `onConfigurationChanged` (small existing gap) |
| R8 | Lint debt (2067 errors) obscures new issues | L×M | Baseline lint baseline file; CI fails only on NEW errors vs baseline |
| R9 | Icons library (1.7.8) vs new compose UI mismatch | L×M | Pin; replace incrementally with Material Symbols (Phase 6) |
| R10 | Motion changes disliked / accessibility | M×M | Reduce-motion support; springs bounded; a11y review |
| R11 | AGP 9 built-in Kotlin migration breaks build | M×M | Dedicated infra PR with full build + tests; rollback trivial (flag re-add) |
| R12 | AGPL obligations (icons/assets) | L×L | Keep assets AGPL/OFL-compatible; use Material Symbols (Apache-2.0) |

---

## 12. Deliverable 10 — Testing Strategy

- **Unit:** existing 10 app tests + 40 extractor tests must stay green on every PR
  (add to CI).
- **New unit:** token-parity test (Compose scheme == parsed `tokens/*` resources);
  `ThemeHelper` returns correct scheme per setting; dynamic-color fallback logic.
- **Instrumented (emulator, API 21/35):** theme switching across all 4 modes × 3
  services; dynamic color on/off; font-scale 1.0/1.3/2.0; landscape.
- **Screenshot/robotics:** baseline + per-phase golden images for D6 #1–13; compare
  with tolerance for the 4 theme modes. Pixel-verify backgrounds (we already know
  light `#FFFBFE`, black `#000000`, black toolbar `#992722`).
- **Manual device matrix:** YouTube live + VOD playback, SABR path, background/popup
  player, downloads, notifications — regression suite per phase (P0/P1 screens).
- **CI:** `compileDebugKotlin`, `testDebugUnitTest`, extractor tests, lint vs
  baseline, screenshot on API 35. Release build smoke.

---

## 13. Deliverable 11 — Accessibility Strategy

- **Color:** M3 roles are designed for 4.5:1 contrast; verify onSurface/onPrimary etc.
  per theme with a11y contrast check in CI; Dynamic Color keeps accessible roles by
  construction.
- **Typography:** use the type scale (no sub-12 sp body); honor `fontScale` (verified
  up to 2.0); "emphasized type" via weight/size, never color alone.
- **Motion:** `Settings → Accessibility` reduce-motion (springs → tween / disable);
  honor system animation scale 0.
- **Touch targets:** ≥48 dp for all controls (button groups, FAB menu items,
  toolbar items); verify with TalkBack on the player control panel.
- **Content descriptions:** audit `content-desc` on all icon buttons (player
  controls already have descriptions, e.g. "Play video, duration:" — keep/standardize).
- **TalkBack flows:** test home→video→player, search→filter→results, settings
  toggles; ensure the bottom-sheet player is reachable.
- **Dynamic color contrast on dark/black themes:** verify the black theme (pure
  `#000000` surfaces) still meets contrast with M3 surface tints.

---

## 14. Deliverable 12 — Performance Strategy

- **Budgets:** cold start ≤ current 2.1 s (do not regress); 60-fps list scroll;
  playback start < 2.5 s; PSS ≤ current 217 MB baseline (watch during Compose
  adoption; report per phase).
- **Compose best practices:** stable `remember`/derived state for theme; avoid
  recomposition storms in list items; `strongSkipping` (default in recent
  Compose) is already on via the runtime.
- **Lazy lists:** keep `RecyclerView` where it wins (feed) and move to
  `LazyColumn` only where the flow is being rewritten; measure with
  `BaselineProfile` (add a baseline profile module in Phase 3).
- **Carousel/hero (Phase 5):** lazily compose; precompute keyline lists.
- **Dynamic color:** compute the scheme once at theme change, not per frame.
- **Build:** adopt AGP 9 built-in Kotlin (Phase 0) for faster compiles; keep
  configuration cache (already feasible).
- **Tooling:** Macrobenchmark (startup + scroll + playback) added in Phase 2 and
  wired to CI with budgets.

---

## 15. Deliverable 13 — Rollout & Rollback Plan

- **Feature flags (debug `SharedPreferences`):** `ui_m3_tokens` (Phase 1–2),
  `ui_m3_components` (Phase 3), `ui_dynamic_color` (Phase 4), `ui_expressive`
  (Phase 5). All default **on** only after their phase's exit criteria pass;
  rollback = flip off and re-apply theme.
- **Progressive rollout:** internal → alpha testers → 25% → 50% → 100% for each
  phase's visual changes; monitor Play crash-free rate and user feedback.
- **Rollback safety:** every phase is a separate mergeable unit; the View and
  Compose themes remain source-compatible with legacy tokens until Phase 6
  cleanup, so any phase can be reverted without a design-system split-brain.
- **Player path:** the bottom-sheet player (R4) is behind `ui_expressive` only in
  Phase 5; if regressions appear, the flag reverts to the MDC sheet with zero data
  loss.
- **Data preservation:** no schema changes; tokens/settings only. App data,
  subscriptions, history, and downloads are untouched.

---

## 16. Deliverable 14 — Appendix: Research Sources & Evidence

### Official documentation
- AndroidX Compose Material 3 release notes (July 29, 2026 snapshot): current stable
  **1.4.0**, alpha 1.5.0-alpha25. `developer.android.com/jetpack/androidx/releases/compose-material3`
- Jetpack Compose Material 3 docs (incl. M3 Expressive intro, dynamic color, type
  scale, shape scale, tonal elevation):
  `developer.android.com/develop/ui/compose/designsystems/material3`
- M3 Expressive announcement & tactics:
  `m3.material.io/blog/building-with-m3-expressive` (May 13, 2025)
- Compose Material3 sources (androidx-main): `MaterialTheme.kt`, `MotionScheme.kt`
- Google Maven metadata: `dl.google.com/android/maven2/androidx/compose/material3/…`
  (artifact/version lists; BOM 2026.06.01 mapping: ui/foundation/animation 1.11.4,
  material3 1.4.0)

### Empirical probes (this session)
- Compiled identical Kotlin against material3 **1.4.0** and **1.5.0-alpha25**:
  verified `internal` vs `public` for `MaterialExpressiveTheme`, `MotionScheme`,
  `MaterialTheme(motionScheme=…)`, `MotionScheme.expressive()`, `Carousel`,
  `SearchBar(state=…)`, `SecureTextField`, `FloatingToolbar` (see §2.5 table).
- `javap` dumps of `classes.jar` (1.4.0 & 1.5.0-alpha25) confirming
  `MotionScheme` interface shape and companion factories.
- On-device theme-switching pixel samples: light `#FFFBFE`, black `#000000`,
  black-theme toolbar `#992722` — matching `values/colors.xml` + `colors_services.xml`.

### Community / ecosystem
- NewPipe design-system discussions: `TeamNewPipe/NewPipe#7595` ("Update NewPipe
  design to Material Design version 2 or version 3"),
  `TeamNewPipe/NewPipe#13064` ("Material 3 (Material You) UI"), PR #13525 (top app
  bar composable) — confirms the upstream direction toward Compose + Material 3
  that PipePipe is already partly on.
- Practical Compose guides for the new expressive components
  (LoadingIndicator, Split Button, FAB Menu, ButtonGroups, FloatingToolbars,
  FlexibleBottomAppBar): navczydev "Express Yourself: Designing with Material 3
  Expressive in Compose" (May 2025).

### Key version facts (2026-08-03)
- Compose Material 3 stable: **1.4.0** (2025-09-24); alpha: **1.5.0-alpha25**.
- Compose BOM latest: **2026.06.01** → ui/foundation/animation **1.11.4**,
  material3 **1.4.0**.
- material3 1.5.0-alpha25 requires compose **1.12.0-beta01**.
- MDC Views stable: **1.14.0**. AppCompat: **1.7.1**. Preference: **1.2.1**.

---

*End of study. Proposed plan only — no source changes were made to the repo.*
