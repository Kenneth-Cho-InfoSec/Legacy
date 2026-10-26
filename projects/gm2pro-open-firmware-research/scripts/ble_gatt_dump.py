#!/usr/bin/env python3
"""Dump BLE GATT services using bleak.

Usage:
  python scripts/ble_gatt_dump.py --address AA:BB:CC:DD:EE:FF
  python scripts/ble_gatt_dump.py --name "联想thinkplus-GM2 pro"
"""

from __future__ import annotations

import argparse
import asyncio
from pathlib import Path

from bleak import BleakClient, BleakScanner


async def resolve_address(name: str | None, address: str | None) -> str:
    if address:
        return address
    if not name:
        raise SystemExit("Provide --address or --name")

    devices = await BleakScanner.discover(timeout=8)
    matches = [d for d in devices if d.name and name.lower() in d.name.lower()]
    if not matches:
        seen = "\n".join(f"{d.address}\t{d.name}" for d in devices)
        raise SystemExit(f"No device matching {name!r}. Seen:\n{seen}")
    return matches[0].address


async def dump(address: str, output: Path) -> None:
    lines: list[str] = [f"Address: {address}", ""]
    async with BleakClient(address) as client:
        services = client.services
        for service in services:
            lines.append(f"Service {service.uuid} {service.description}")
            for char in service.characteristics:
                props = ",".join(char.properties)
                lines.append(f"  Characteristic {char.uuid} [{props}] {char.description}")
                for desc in char.descriptors:
                    lines.append(f"    Descriptor {desc.uuid} {desc.description}")
                if "read" in char.properties:
                    try:
                        value = await client.read_gatt_char(char.uuid)
                        lines.append(f"    Read: {value.hex()}")
                    except Exception as exc:  # noqa: BLE read failures are useful notes.
                        lines.append(f"    Read failed: {exc}")
            lines.append("")

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text("\n".join(lines), encoding="utf-8")
    print(f"Wrote {output}")


async def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--address")
    parser.add_argument("--name")
    parser.add_argument("--output", default="analysis/bluetooth/ble-gatt-dump.txt")
    args = parser.parse_args()

    address = await resolve_address(args.name, args.address)
    await dump(address, Path(args.output))


if __name__ == "__main__":
    asyncio.run(main())

