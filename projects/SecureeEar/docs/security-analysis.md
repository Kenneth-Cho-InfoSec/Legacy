# SecureeEar Security Analysis

SecureeEar is a local-only Android reconstruction of a UseeEar-compatible Wi-Fi
ear camera companion app. This document summarizes the reverse-engineering
findings that informed the clean rebuild.

## Reverse-Engineering Workflow

- Decompiled the UseeEar 1.0.062 APK with JADX and preserved the reference work on the `reveng-v1` branch.
- Separated reconstruction experiments onto the `rebuild` branch so the clean `main` branch can remain focused and buildable.
- Preserved endpoint, permission, and telemetry findings on the `analysis` branch for auditability.
- Rebuilt the app around the minimum local camera path instead of carrying forward the full decompiled application surface.

## Key Privacy Findings

The Java review did not identify classic spyware behavior such as SMS theft,
contact scraping, call-log collection, accessibility abuse, device-admin
persistence, boot persistence, or dynamic dex loading.

The original app was still privacy-sensitive. The strongest finding was a
license/reporting path that could post app, phone, firmware, serial, MAC,
vendor, product, and license data to Simicloud infrastructure. The app also
contained code designed to request cellular networking, which is notable for a
Wi-Fi camera app because the camera connection itself is local.

## Cloud And Local Endpoints

High-risk cloud routes and domains observed during analysis included:

| Endpoint or domain | Risk |
| --- | --- |
| `https://yun.simicloud.com/licplatform/user/v2/report` | Java-side license/report POST with persistent device and app metadata. |
| `https://lic.simicloud.com/api/nto/miss?` | License provisioning or missing-license route. |
| `http://www.simicloud.com/c/UStorage/android/version.xml` | Cleartext cloud version endpoint. |
| `lic.simicloud.com` and `info2.simicloud.com` | Native-library strings tied to licensing or reporting behavior. |

Expected local camera routes included:

| Endpoint | Assessment |
| --- | --- |
| `http://192.168.1.254/DCIM/MOVIE/%s` | Local camera media route. |
| `http://192.168.1.254/DCIM/PHOTO/%s` | Local camera media route. |
| `http://10.10.10.254:` | Local Wi-Fi device base URL. |
| `http://127.0.0.1:` | Local loopback proxy or storage route. |

## SecureeEar Hardening Delta

SecureeEar removes or avoids the Java-side surfaces that created the clearest
privacy risk:

- No cloud APIs.
- No analytics.
- No account system.
- No WebView screens.
- No Java-side license reporting flow.
- No cellular fallback request.
- No phone identifier collection.
- No installed-app enumeration.
- No broad storage, media, or audio permissions.

The app keeps only the local camera support needed for UseeEar-compatible Wi-Fi
devices and restricts expected hosts to local camera or loopback addresses.

## Residual Risk

The native `.so` files are inherited opaque components. Static Java review can
reduce the app surface, but it cannot prove native code never emits network
traffic. Production trust should depend on dynamic traffic capture, native
analysis, or replacement of the camera protocol implementation.

Recommended verification:

1. Connect only to the camera Wi-Fi network.
2. Disable cellular data.
3. Block `*.simicloud.com` and `www.simicloud.com`.
4. Start SecureeEar and confirm preview frames arrive.
5. Capture traffic and confirm connections remain local.

## Branch Map

- `main`: clean local-only Android reconstruction.
- `reveng-v1`: decompiled UseeEar 1.0.062 reference.
- `analysis`: detailed endpoint and risk notes.
- `rebuild`: reconstruction experiment used to validate the rebuild path.
