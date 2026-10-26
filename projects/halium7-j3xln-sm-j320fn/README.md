# Samsung Galaxy J3 2016 Halium 7.1 Bring-Up

This repository contains the patched source files and built test artifacts for a Halium 7.1 bring-up of the Samsung Galaxy J3 2016 `SM-J320FN` (`j3xnlte` / source tree `j3xln`) on the Spreadtrum `SC9830A` platform.

## What Is Included

- `source-tree/`
  - The exact source files patched during the build and debugging cycle.
- `artifacts/halium-boot.img`
  - Final boot image for Halium testing.
- `artifacts/boot.img`
  - Same boot payload with a generic filename for tooling that expects `boot.img`.
- `artifacts/system.img.xz`
  - Compressed copy of the sparse Android/Halium `system.img` for easier GitHub storage and download.
  - Use `xz -d system.img.xz` to recover the sparse image before flashing or testing.
- `artifacts/boot-odin.tar`
  - Odin-friendly tar containing the boot image for Samsung flashing.
- `artifacts/android-ramdisk.img`
  - Extracted ramdisk image from the built target output.
- `artifacts/halium-j3xnlte-twrp-flashable.zip`
  - TWRP flashable package that streams `boot.img` to `KERNEL` and the raw ext4 system image to `SYSTEM`.
- `artifacts/hybris-recovery.img`
  - Halium debug boot image for early init / diagnosis work.
- `artifacts/hybris-recovery-boot-odin.tar`
  - Odin-friendly tar for flashing `hybris-recovery.img` to the boot partition.
- `artifacts/SHA256SUMS`
  - Checksums for the packaged build artifacts.

## Why The Full Synced Tree Is Not In This Repo

The original synced Halium tree is about `38G`, which is too large and noisy for a practical GitHub repository. This repo contains the parts that matter for reproducing the bring-up work:

- the changed source files
- the resulting test images
- the flashing and validation guidance

## Exact Halium 7.1 Base Used

- Halium manifest repo: [Halium/android](https://github.com/Halium/android)
- Halium branch synced: [`halium-7.1`](https://github.com/Halium/android/tree/halium-7.1)
- Exact default manifest baseline: [`default.xml`](https://github.com/Halium/android/blob/halium-7.1/default.xml)
- Local manifest remote in the synced tree: `https://github.com/Halium/android`
- Local manifest branch in the synced tree: `refs/heads/halium-7.1`

The synced tree used here is the standard Halium 7.1 base, which in practice means:

- AOSP remote at `android-7.1.2_r37`
- Lineage remote on `cm-14.1`
- Halium remote on `halium-7.1`

## Final Artifacts

- `halium-boot.img`
  - SHA256: `39263e0959762468db27f25c342c728a1ce23a995a4de01ef4ee25b624f9339b`
- `boot-odin.tar`
  - SHA256: `364b6e1f24ebf60149d6784b0beb16701c40d3c5dab3921bc7aa272ba1f8b1ff`
- `android-ramdisk.img`
  - SHA256: `1a4fb31d3cf08860e55e5257a060afbe6d4a4d4226a3a5f85b35edb27982ea5f`
- `halium-j3xnlte-twrp-flashable.zip`
  - SHA256: `2d57077a8cc26b6db5f4008d3c5c44551a3b144530bab30667752fa6daba8b1d`
- `hybris-recovery.img`
  - SHA256: `5b5ee318266545018fac0d1bbd4216a099e7104294e6c1cc0714d09036e780d8`
- `hybris-recovery-boot-odin.tar`
  - SHA256: `6737e5f72dacca4fc356a7f612115436c69c30e02c86025dff91647a7c0319e7`
- `system.img.xz`
  - SHA256: `dfc915196e305387564afbbb166ca9871655dc01a2844caf1a6657dd4b6ab664`

## Flashing

### Heimdall / Odin Boot Flash

Boot the device into Samsung Download Mode and flash:

```sh
heimdall flash --BOOT halium-boot.img
```

After that, boot recovery and install the Halium rootfs plus the system image.

### Odin

Use `artifacts/boot-odin.tar` in the `AP` slot. Do not repartition and do not use a PIT file.

### TWRP

Use `artifacts/halium-j3xnlte-twrp-flashable.zip` from TWRP if you want recovery-side flashing of both boot and system in one step.

This package writes directly to the live device paths discovered over ADB:

- `/dev/block/platform/sdio_emmc/by-name/KERNEL`
- `/dev/block/platform/sdio_emmc/by-name/SYSTEM`

## Rootfs / Userspace

For first hardware validation, use the Halium reference rootfs flow. Halium documentation recommends:

```sh
./halium-install -p halium <path-to-rootfs-tarball> <path-to-system.img>
```

Ubuntu Touch note:

- Current UBports documentation says Halium 7.1 is deprecated for new ports.
- This repo therefore stops at a working Halium 7.1 test package and the reference-rootfs path for first boot validation.

## Device Tree And Kernel Source Provenance

- LineageOS 14.1 device base discussed here: [XDA thread for SM-J320FN/F/G/M](https://xdaforums.com/t/rom-oms-lineageos-14-1-samsung-galaxy-j3-2016-sm-j320fn-f-g-m-unofficial.4092513/)
- Kernel source remote used in the synced tree: [djeman/android_kernel_samsung_sharkls](https://github.com/djeman/android_kernel_samsung_sharkls)
- Kernel branch tracked locally: `cm-14.1`
- Local device tree path used for this build: `device/samsung/j3xln`
- Local kernel tree path used for this build: `kernel/samsung/j3xln`

That XDA thread matters because this port is not coming from some clean officially-maintained upstream stack. The practical base for this bring-up is an old unofficial LineageOS 14.1 port lineage, plus the Halium 7.1 adaptation work captured in this repository.

## Important Patched Areas

- Device config cleanup in `device/samsung/j3xln`
- Real-device partition layout fixes for `sdio_emmc` and Samsung `KERNEL` / `SYSTEM` naming
- `hybris-recovery` mountpoint fixup mapping for `j3xln` / `j3xnlte`
- Host Python 2 to Python 3 conversions in `build/tools/`
- Nanopb / protobuf compatibility fixes in `external/nanopb-c`
- AIDL parser compatibility fixes in `system/tools/aidl`
- Releasetools compatibility fixes for Python 3.12

## Kernel Cmdline And Early Boot Notes

Current `BoardConfig.mk` cmdline:

```make
BOARD_KERNEL_CMDLINE := androidboot.hardware=sc8830 user_debug=0 root=/dev/ram0 rw init=/init
```

Important context for that boot path:

- `androidboot.hardware=sc8830` is what the early userspace expects for this Spreadtrum family.
- `root=/dev/ram0` and `init=/init` mean we are booting through the ramdisk first, not directly into a normal Android rootfs on disk.
- The current blocker is not that the kernel fails to start at all; it is that the device still stalls after the Samsung splash and the red `Kernel is not SEANDROID enforcing` warning.

That red Samsung text is usually just the stock bootloader complaining that the boot image is custom. It is noisy, dramatic, and very Samsung, but it is not by itself a useful root cause.

The useful logs for the next round are the early ones:

- `diagnosis.log` from `hybris-recovery` / early Halium init
- `/sys/fs/pstore/console-ramoops` if the kernel leaves pstore output
- `/proc/last_kmsg` on kernels or recovery setups that still expose it

The reason `hybris-recovery.img` is in this repo is exactly that: the clean next step is to boot a debug-oriented Halium image and capture early mount / init failures instead of pretending another blind flash will magically fix an ancient Samsung bring-up.

## Device

- Vendor: Samsung
- Model: Galaxy J3 2016
- Variant: `SM-J320FN`
- Live device codename: `j3xnlte`
- Source tree codename: `j3xln`
- SoC: Spreadtrum `SC9830A`
- Android base: LineageOS 14.1 / Android 7.1

## Notes

- This tree does not produce a separate `vendor.img` for this Android 7.1 layout.
- The raw ext4 `system-raw.img` used for the TWRP package is not committed because of its size; `system.img.xz` is the GitHub-friendly copy in this repository.
- The Ubuntu Touch-style `rootfs.img` and the combined `j3-ut-halium-install-smallrootfs.zip` were not committed because they are too large for normal GitHub storage.
- Target stack: Halium 7.1

## Small Rant

Spreadtrum Samsung bring-up on old Halium bases is exactly the kind of job that reminds you how much time gets burned on things that are not the actual bug.

- Samsung's `Kernel is not SEANDROID enforcing` warning looks dramatic but usually is not the real problem.
- Old Halium 7.1 docs, dead rootfs links, and Python 2 era tooling all waste time before you even reach device-specific failures.
- USB enumeration on Windows being flaky enough to break Odin and ADB in alternating ways is the kind of nonsense that makes a straightforward debug cycle take far longer than it should.

The useful part is that the repository now captures the real state of the port and the current blocker instead of pretending this was a clean or elegant path.

Honestly, I probably should have chosen a more modern phone instead of torturing myself with this one. If anyone feels like crowdfunding me for better hardware or something equally unreasonable, I would be grateful.

## Current Status

- A clean LineageOS 14.1 base was reflashed and confirmed bootable.
- A corrected Ubuntu Touch-style package was built to place `rootfs.img` and `system.img` under `/data`, matching `halium-install` behavior.
- That package flashes successfully, but the device still stalls at the Samsung splash after the custom-kernel warning `Kernel is not SEANDROID enforcing`.
- The next debug step is `hybris-recovery`, which has now been built and packaged in this repository.
- See `STATUS.md` for a chronological record of what was tried and the current blocker.
