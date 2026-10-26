# Tooling

## Installed In WSL

The WSL environment has the main command-line research stack installed:

- Bluetooth capture/inspection: `bluetoothctl`, `btmon`, `sdptool`, `hcitool`, `gatttool`, `tshark`, `wireshark`
- Firmware triage: `binwalk`, `strings`, `file`, `xxd`, `hexdump`, `sha256sum`
- Reverse engineering: `radare2` as `r2`, `arm-none-eabi-objdump`
- Hardware/debug: `openocd`, `pyocd`, `flashrom`, `sigrok-cli`, `pulseview`, `lsusb`
- Python BLE helper: local `.venv` with `bleak`

The current inventory is saved in `analysis/tool-inventory-wsl.txt`.

## Manual/External Tools

- Ghidra is not available from this Ubuntu apt source. Install it separately on Windows or WSL when a firmware dump is ready for deeper analysis.
- Rizin is not available from this Ubuntu apt source. Use installed `r2` for now, or install Rizin separately later.

## WSL Bluetooth Caveat

WSL2 currently has no exposed Bluetooth HCI controller. `analysis/bluetooth/wsl-hci-status.txt` records the current empty HCI state. Use Android HCI snoop logs, Windows inventory, or pass a USB Bluetooth dongle into WSL before using `btmon` against live traffic.

