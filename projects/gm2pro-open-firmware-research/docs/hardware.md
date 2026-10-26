# Hardware Identification

## Photo Checklist

- Exterior labels, box label, case label, FCC/CE marks, and model text.
- Each earbud before opening, during seam separation, and after PCB removal.
- SoC top marking, package outline, crystal marking, RF matching network, antenna feed, and antenna element.
- External flash if present, charger/PMIC, battery protection board, microphones, speaker driver, LEDs, touch electrode/controller, pogo-pin contacts, and flex cables.
- Charging case PCB: USB-C input, charger IC, battery, pogo pins, LEDs/display, button/reset contacts, and test pads.

## Safe Opening Notes

- Start with one sacrificial or less-valued unit if available.
- Use plastic tools first; use metal only after battery location is clear.
- Keep heat controlled and local. Do not heat or puncture pouch cells.
- Photograph cable and battery routing before moving parts.

## Identification Workflow

1. Read every visible IC marking under a microscope.
2. Record package size, pin count, nearby crystal frequency, and connected passives.
3. Search markings against Bluetooth audio SoC families only after the physical marking is recorded.
4. Trace power from battery and pogo pins to charger/PMIC and SoC rails.
5. Trace RF output from SoC through matching network to antenna.

## Bill Of Materials Template

| Ref | Marking | Package | Function | Confidence | Evidence |
| --- | --- | --- | --- | --- | --- |
| U1 | TBD | TBD | Bluetooth/audio SoC | Low | Pending microscope photo |
| U2 | TBD | TBD | Flash or PMIC | Low | Pending microscope photo |
| B1 | TBD | Pouch cell | Earbud battery | Low | Pending label photo |

