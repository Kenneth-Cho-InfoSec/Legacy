# SecureeEar

[![Android CI](https://github.com/Kenneth-Cho-InfoSec/SecureeEar/actions/workflows/android-ci.yml/badge.svg)](https://github.com/Kenneth-Cho-InfoSec/SecureeEar/actions/workflows/android-ci.yml)
![Platform](https://img.shields.io/badge/platform-Android-green)
![Language](https://img.shields.io/badge/language-Java-blue)
![Mode](https://img.shields.io/badge/mode-local--only-informational)

SecureeEar is a clean Android reconstruction focused on local-only operation for UseeEar-compatible Wi-Fi ear hygiene cameras.

<img src="docs/assets/secureeear-hero.png" alt="SecureeEar secure ear camera illustration" width="300">

## Case Study

**Problem:** The original UseeEar Android app contained privacy-sensitive cloud reporting paths and opaque native libraries while also being needed to operate a local Wi-Fi camera.

**Approach:** SecureeEar rebuilds the Android app around the minimum local camera workflow, removes Java-side telemetry surfaces, documents the reverse-engineering evidence, and keeps risky inherited native components isolated behind a small controller.

**Result:** The project demonstrates Android reverse engineering, IoT privacy analysis, and practical reconstruction of a local-first device companion app.

## Repository Branches

- `main`: clean local-only Android reconstruction.
- [`reveng-v1`](https://github.com/Kenneth-Cho-InfoSec/SecureeEar/tree/reveng-v1): imported UseeEar 1.0.062 JADX/decompiled reference.
- [`analysis`](https://github.com/Kenneth-Cho-InfoSec/SecureeEar/tree/analysis): reverse-engineering findings, endpoint review, and risk notes.
- [`rebuild`](https://github.com/Kenneth-Cho-InfoSec/SecureeEar/tree/rebuild): imported reconstruction experiment used to validate the clean rebuild path.

## Research Narrative

The reverse-engineering summary is documented in [docs/security-analysis.md](docs/security-analysis.md). It covers the original app behavior, suspicious cloud endpoints, permission surface, local camera paths, native-library residual risk, and the hardening decisions that shaped SecureeEar.

## Security Model

- No cloud APIs.
- No analytics.
- No WebView.
- No account system.
- No license reporting.
- No cellular fallback request.
- No phone identifier collection.
- No installed-app enumeration.
- No storage/media/audio permissions.

The app keeps the native Wi-Fi camera bridge behind a small Java controller and patches the native callback helper so the previous external-network bind path is refused. The remaining native `.so` files are opaque inherited components, so runtime network capture is still recommended before trusting any production deployment.

## Architecture Snapshot

- A minimal Android Activity handles permissions, Wi-Fi guidance, connection control, and preview status.
- The Java camera controller keeps local host allow-listing close to the native camera bridge.
- Native libraries remain inherited components and are treated as residual risk until fully replaced or dynamically audited.

## Status and Roadmap

Current status: local-only Android reconstruction with documented reverse-engineering evidence.

Near-term roadmap:

- Capture and publish repeatable network traces for original and reconstructed builds.
- Continue reducing inherited native-library reliance where protocol behavior can be reimplemented safely.
- Add screenshots or a short demo once device preview behavior is verified on hardware.

## Local Device Support

Expected SSID prefixes:

- `UseeEar`
- `i4season`
- `inskam`
- `Yanxuan`

Allowed local hosts:

- `192.168.1.254`
- `10.10.10.254`
- `192.168.1.1`
- `127.0.0.1`

## Build

```powershell
.\gradlew.bat :app:assembleDebug
```

Debug APK:

```text
app/build/outputs/apk/debug/app-debug.apk
```

## Verification Checklist

1. Connect the Android device to the camera Wi-Fi network.
2. Disable cellular data or block it with a firewall.
3. Block `*.simicloud.com`.
4. Start SecureeEar and verify preview frames arrive.
5. Capture traffic and confirm connections remain local.
