# Testing And Recovery

## Before Any Firmware Write

- Three matching firmware backups exist.
- SHA256 and size are recorded.
- Extraction method is documented.
- Restore command is tested on sacrificial hardware or a replaceable flash chip.
- Bootloader or hardware recovery entry is documented.
- Battery is charged and bench supply current limit is set if powering externally.

## Recovery Tests

- Stock firmware restore.
- Bootloader entry.
- UART boot log after restore.
- SWD/JTAG identify-only attach.
- OTA recovery if present.
- Chip-off restore if external SPI flash is used.
- Case-assisted reset and factory reset.

## Bricked Device Procedure Template

1. Record symptoms and current draw.
2. Try documented reset and bootloader entry.
3. Capture UART during boot.
4. Attempt read-only debugger identify.
5. Restore known-good dump using the least invasive proven path.
6. Re-test pairing, charging, audio, mic, and reset.

