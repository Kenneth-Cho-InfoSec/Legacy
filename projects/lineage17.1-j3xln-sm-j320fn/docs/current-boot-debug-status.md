# Current Boot Debug Status

Date: 2026-05-15

Device: Samsung Galaxy J3 2016 SM-J320FN, Spreadtrum SC9830A / SC8830 family.

## Current Result

The LineageOS 17.1 build flashes but does not boot to Android. ADB never appears during normal boot. Current behavior is an early screen on/off boot loop before userspace is reachable.

## Fixes Already Tested

- Added Samsung `SEANDROIDENFORCE` trailer marker to test boot images. This removed the visible `Kernel is not SEANDROID enforcing` warning.
- Matched boot image load addresses against the known-working LineageOS 14.1 boot image:
  - kernel load: `0x00008000`
  - ramdisk load: `0x01000000`
  - second load: `0x00f00000`
  - tags: `0x00000100`
  - page size: `2048`
- Restored the missing Spreadtrum boot DT table. The original Android 10 boot image had `dt_size=0`; the known-working boot image has `dt_size=505856`.
- Recreated the `SPRD` DT table from six generated `j3xnlte` DTBs. The generated DT table SHA256 matched the DT section extracted from the known-working LineageOS 14.1 boot image.
- Tested a hybrid boot image using the known-working 14.1 kernel payload plus the Android 10 ramdisk. ADB still did not appear.
- Removed stale Android 7/Halium init payloads from `/system/system/etc/init`:
  - removed `halium.rc`
  - replaced `j3xln.rc` with a minimal Android 10-safe file
- Tested a no-encryption/storage variant:
  - ramdisk fstab first-stage mounts only `SYSTEM`
  - system fstab changes `/data` from `encryptable=footer` to `formattable`

## Current Hypothesis

The remaining blocker is probably not just the Samsung bootloader warning, missing DT table, or basic fstab encryption handling. The next likely class is an Android 10 userspace expectation that the old Spreadtrum kernel tree does not satisfy, or a very early init/kernel watchdog reset before ADB can start.

## Next Debug Step

Build a deliberately debug-heavy boot image that reduces automatic reboot behavior and maximizes early logging. The goal is to stop the loop long enough to capture a useful kernel panic, init failure, or watchdog/reset reason.
