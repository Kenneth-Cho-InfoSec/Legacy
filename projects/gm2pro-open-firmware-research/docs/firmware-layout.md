# Firmware Layout

Do not commit firmware dumps. Commit only hashes, offsets, structural notes, and scripts that operate on user-supplied dumps.

## Regions To Identify

- ROM/bootloader handoff.
- Bootloader.
- Main application.
- OTA slot or staging area.
- Resource/audio prompt section.
- RF calibration and crystal trim.
- EQ/audio DSP tables.
- Pairing/bonding data.
- Device address and serial/config records.
- Crash logs or factory test records.

## Dump Record Template

| Field | Value |
| --- | --- |
| Dump filename | Private |
| SHA256 | TBD |
| Size | TBD |
| Extraction method | TBD |
| Voltage/tool | TBD |
| Date | TBD |
| Repeated reads matched | TBD |
| Restore tested | TBD |

