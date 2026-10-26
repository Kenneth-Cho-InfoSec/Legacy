# Lenovo GM2 Pro Open Firmware Research

Owner-controlled research workspace for documenting Lenovo/thinkplus GM2 Pro Bluetooth earbuds and building a path toward clean-room open firmware or compatible tooling.

This repository is for hardware notes, Bluetooth captures, scripts, reverse-engineering metadata, and open firmware experiments. Proprietary firmware dumps stay private under `firmware-dumps/private-not-for-redistribution/` and must not be committed or redistributed.

## Current Local Baseline

- Device name observed on Windows: `联想thinkplus-GM2 pro`
- Bluetooth device ID fragment: `DEV_41423420DAE5`
- Observed profiles/services: A2DP, AVRCP, HFP, BLE `1800`, `1801`, `180A`, `180F`, `FEE0`, `F000FFC0-0451-4000-B000-000000000000`
- Host Bluetooth adapter: MediaTek `USB\VID_13D3&PID_3563&MI_00`
- WSL: Ubuntu 24.04 on WSL2; no Bluetooth HCI exposed unless a USB dongle is passed through

## First Milestones

1. Capture Windows and Android Bluetooth baselines.
2. Photograph and map the PCB/test pads.
3. Identify SoC, flash, charger, battery, mics, speaker, antenna, and case interface.
4. Extract firmware only after a tested backup and recovery workflow exists.
5. Publish clean-room documentation and tooling; keep vendor binaries private.

## Local Commands

```powershell
scripts\collect_windows_bt_inventory.ps1
```

```bash
./scripts/collect_wsl_tool_inventory.sh
./scripts/hash_dumps.sh firmware-dumps/private-not-for-redistribution
./scripts/triage_firmware.sh firmware-dumps/private-not-for-redistribution/<dump>.bin
```

For BLE GATT enumeration, use the local virtualenv:

```bash
. .venv/bin/activate
python scripts/ble_gatt_dump.py --name "联想thinkplus-GM2 pro"
```
