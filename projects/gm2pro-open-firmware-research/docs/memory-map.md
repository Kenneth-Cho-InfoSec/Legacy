# Memory Map

## Working Table

| Region | Start | End | Size | Purpose | Evidence |
| --- | --- | --- | --- | --- | --- |
| Boot vector/table | TBD | TBD | TBD | Reset/interrupt vectors | Pending dump |
| Bootloader | TBD | TBD | TBD | First-stage firmware | Pending dump |
| Application | TBD | TBD | TBD | Main firmware | Pending dump |
| Config/calibration | TBD | TBD | TBD | Device-specific data | Pending dump |

## Analysis Cues

- ARM Cortex-M vector table: initial stack pointer followed by reset handler pointer.
- Partition headers may include magic values, CRCs, image lengths, version strings, and slot flags.
- High entropy regions may be compressed or encrypted.

