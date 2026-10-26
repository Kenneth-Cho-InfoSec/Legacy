# Test Pad Pinout

## Mapping Method

1. Find GND by continuity to battery negative, USB shield, and large copper pours.
2. Measure all pads against GND in these states: off, in case, out of case, pairing, charging.
3. Use high impedance probes; avoid shorting adjacent pads.
4. For serial candidates, connect logic analyzer RX-only first.
5. For reset candidates, use a resistor and confirm behavior before direct shorts.

## Pad Table

| Pad | Location | Voltage Off | Voltage On | Evidence | Candidate Function | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| TP1 | TBD | TBD | TBD | TBD | Unknown | Low |

## UART Notes

- Start with 1.8 V thresholds.
- Common baud rates to try: 115200, 460800, 921600, 1000000, 1500000, 2000000.
- Capture boot/reset with RX-only before transmitting.

## SWD/JTAG Notes

- Use target voltage sensing.
- First operation is identify-only.
- Do not mass erase or unlock unless working on a sacrificial unit with a verified restore path.

