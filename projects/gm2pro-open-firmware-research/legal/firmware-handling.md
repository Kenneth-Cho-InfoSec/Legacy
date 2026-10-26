# Firmware Handling Policy

This project is for authorized interoperability, repairability, documentation, and clean-room open-source development on owner-controlled hardware.

## Allowed To Publish

- Firmware hashes and sizes.
- Extraction method notes.
- Offset maps and structural descriptions.
- Clean-room function names and behavior notes.
- Scripts that analyze user-supplied firmware.
- Open firmware code written without copying proprietary implementation.

## Not Allowed To Publish

- Vendor firmware dumps.
- Extracted proprietary code or data blobs.
- Device-unique keys, pairing data, or calibration data tied to a private device.
- Instructions aimed at third-party device access or credential theft.

## Repository Rule

Keep proprietary dumps in `firmware-dumps/private-not-for-redistribution/`. That path is ignored by git except for `.gitkeep`.

