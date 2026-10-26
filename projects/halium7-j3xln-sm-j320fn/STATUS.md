# Current Bring-Up Status

This file records the current known-good work, the failed attempts, and the present blocker for the Samsung Galaxy J3 2016 `SM-J320FN` (`j3xnlte`) Halium 7.1 / Ubuntu Touch bring-up.

## What Was Confirmed

- Halium 7.1 source tree sync was complete before build work started.
- `halium-boot.img` and `system.img` were built successfully from the `j3xln` tree.
- The live device partition layout was checked over ADB and corrected in the device tree:
  - `/dev/block/platform/sdio_emmc/by-name/KERNEL -> /dev/block/mmcblk0p20`
  - `/dev/block/platform/sdio_emmc/by-name/RECOVERY -> /dev/block/mmcblk0p21`
  - `/dev/block/platform/sdio_emmc/by-name/CACHE -> /dev/block/mmcblk0p24`
  - `/dev/block/platform/sdio_emmc/by-name/SYSTEM -> /dev/block/mmcblk0p25`
  - `/dev/block/platform/sdio_emmc/by-name/HIDDEN -> /dev/block/mmcblk0p26`
  - `/dev/block/platform/sdio_emmc/by-name/userdata -> /dev/block/mmcblk0p27`
  - `/dev/block/platform/sdio_emmc/by-name/efs -> /dev/block/mmcblk0p17`
- The original device-tree NAND assumptions were wrong and were patched to the actual eMMC layout.
- A clean LineageOS 14.1 base was reflashed later and is the correct Android-side base for further Halium work.

## What Was Tried

1. Built `halium-boot.img` and `system.img`.
2. Flashed boot and system directly in early tests.
3. Built a first TWRP flashable package that wrote:
   - `boot.img` to `KERNEL`
   - raw Android `system` image to the real `SYSTEM` partition
4. That package booted only to the Samsung splash / custom-kernel warning and did not reach Ubuntu Touch.
5. Switched to the proper `halium-install` model for Ubuntu Touch:
   - generate `rootfs.img`
   - shrink `system.img`
   - place both on `/data`
6. Built a new TWRP package for that layout.
7. The first attempt failed while writing `rootfs.img` because the image was still a full `3.0G` file.
8. Shrunk `rootfs.img` to its minimum size, rebuilt the package, and flashed again.
9. That second package flashed successfully but the phone still stalls at the Samsung splash after `Kernel is not SEANDROID enforcing`.

## Why The Current Stall Matters

- The Samsung text `Kernel is not SEANDROID enforcing` is very commonly only a custom-kernel warning.
- The actual blocker is that the device still does not progress into a bootable Halium / Ubuntu Touch userspace after a successful flash.
- At this stage, more blind flashing is lower value than early-boot diagnostics.

## Current Best Hypothesis

The remaining issue is now likely in one of these layers:

- early initramfs logic
- mount / handoff to `/data/rootfs.img` and `/data/system.img`
- early kernel / initramfs service bring-up
- Android container or later Halium service startup

## Latest Useful Code Change

`hybris-recovery` originally failed to build because `halium/hybris-boot/fixup-mountpoints` had no mapping for `j3xln`.

That was fixed by adding mountpoint translation for:

- `KERNEL -> mmcblk0p20`
- `RECOVERY -> mmcblk0p21`
- `CACHE -> mmcblk0p24`
- `SYSTEM -> mmcblk0p25`
- `HIDDEN -> mmcblk0p26`
- `userdata -> mmcblk0p27`
- `efs -> mmcblk0p17`

The patched file is tracked here:

- `source-tree/halium/hybris-boot/fixup-mountpoints`

## Current Blocker

- `hybris-recovery.img` is now built successfully.
- A debug boot image exists, but flashing through Odin is still blocked by unreliable Windows USB enumeration on this machine.
- The next technical step is to boot `hybris-recovery` and collect `diagnosis.log` / early-init clues.

## Useful Artifacts In This Repo

- `artifacts/halium-boot.img`
- `artifacts/boot-odin.tar`
- `artifacts/hybris-recovery.img`
- `artifacts/hybris-recovery-boot-odin.tar`
- `artifacts/system.img.xz`
- `artifacts/halium-j3xnlte-twrp-flashable.zip`

## Useful Artifacts Not Committed

These exist locally but are too large for normal GitHub storage:

- `rootfs.img`
- `j3-ut-halium-install-smallrootfs.zip`

## Recommended Next Step

1. Flash `hybris-recovery.img` to the boot partition without overwriting TWRP.
2. Boot the debug image.
3. Capture Halium early boot diagnostics such as `diagnosis.log`.
4. Patch the exact failing layer based on those logs.
