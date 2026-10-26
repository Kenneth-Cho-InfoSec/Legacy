# Security

SecureeEar is designed as a local-only rebuild. It intentionally removes the original app's Java-side Simicloud license/report flow and refuses the previous non-Wi-Fi native socket bind path.

## Current Guarantees

- The Java/Kotlin application layer contains no external HTTP API clients.
- The manifest requests only Wi-Fi/network permissions needed for local camera operation.
- The launcher Activity is the only exported component.
- Android backup and device-transfer extraction are disabled.
- Runtime permissions are limited to Nearby Wi-Fi on Android 13+ or Fine Location on older Android versions where Wi-Fi SSID access requires it.

## Known Residual Risk

The original native Wi-Fi camera library is still present because it is required for device connection. Treat the native layer as an inherited binary dependency until it is replaced with a clean implementation or fully audited.

Recommended validation:

- Run with cellular disabled.
- Block `*.simicloud.com`.
- Capture DNS and TCP traffic during camera start, preview, and stop.
- Verify only local camera IPs are contacted.

## Reporting

Open a GitHub issue with:

- Device model and Android version
- SecureeEar version
- Reproduction steps
- Network capture summary, if relevant
