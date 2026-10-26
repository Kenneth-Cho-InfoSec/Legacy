# Boot Process

## Reconstruction Plan

1. Capture UART logs during reset, case insertion/removal, pairing, and charging.
2. Identify reset vector and early init in firmware dump.
3. Map clock, power, RF, audio, and Bluetooth initialization order.
4. Identify bootloader image validation, OTA slot choice, and fallback behavior.
5. Document factory reset and recovery entry conditions.

## Known States To Test

- Out of case, unpaired.
- Out of case, paired host nearby.
- In case, lid open.
- In case, charging.
- Long touch/button reset.
- Low battery shutdown.

