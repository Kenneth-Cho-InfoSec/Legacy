# Bluetooth Baseline

## Local Observations

- Windows paired device name: `联想thinkplus-GM2 pro`
- Device ID fragment: `DEV_41423420DAE5`
- Host adapter: MediaTek Bluetooth Adapter `USB\VID_13D3&PID_3563&MI_00`
- Profiles observed through Windows PnP: A2DP sink, AVRCP target/controller transport, HFP hands-free, RFCOMM serial ports.
- BLE services observed through Windows PnP: `1800`, `1801`, `180A`, `180F`, `FEE0`, `F000FFC0-0451-4000-B000-000000000000`.

## Windows Capture

```powershell
scripts\collect_windows_bt_inventory.ps1
```

Output is written to `analysis/bluetooth/windows-pnp-inventory.txt`.

## Android HCI Capture

1. Enable Developer Options.
2. Enable Bluetooth HCI snoop log.
3. Pair/reconnect, play audio, use call/mic mode, trigger game mode, test reset.
4. Export the bugreport or pull `btsnoop_hci.log`.
5. Open in Wireshark and document SDP, A2DP, AVRCP, HFP, GATT, codec negotiation, and vendor UUIDs.

## Linux/WSL Capture

WSL2 does not expose the host Bluetooth controller by default. Use this only after passing through a USB Bluetooth adapter or moving to native Linux:

```bash
bluetoothctl scan on
bluetoothctl info <MAC>
sdptool browse <MAC>
sudo btmon -w analysis/bluetooth/gm2pro-btmon.snoop
```

