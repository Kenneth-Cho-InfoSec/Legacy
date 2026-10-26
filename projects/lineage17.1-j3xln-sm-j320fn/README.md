# LineageOS 17.1 Android 10 Bring-Up for Samsung Galaxy J3 2016 SM-J320FN

This repository documents a first-pass Android 10 / LineageOS 17.1 bring-up attempt for the Samsung Galaxy J3 2016 SM-J320FN.

Target device:

- Model: Samsung Galaxy J3 2016 SM-J320FN
- Device family used by the imported tree: `j3xln`
- Live Android device codename observed over ADB: `j3xnlte`
- Chipset: Spreadtrum SC9830A / SC8830 family
- Base source: LineageOS 17.1, Android 10
- Previous known-working generation used as source material: LineageOS 14.1 / Android 7.1 device and kernel tree

## Status

Build status: `boot.img` and `system.img` compile successfully.

Boot status: **not booting yet** on real SM-J320FN hardware. Current tests reach a Samsung/Spreadtrum early boot loop before ADB appears.

This does **not** mean the ROM is usable yet. This is a bring-up artifact for testing. The device tree and kernel are forward-ported from an Android 7.1-era Spreadtrum tree, so the next real work is kernel/userspace handoff, Android 10 init compatibility, display bring-up, vendor blob strategy, SELinux, RIL, Wi-Fi, audio, camera, sensors, and recovery/flash packaging.

In other words: the compiler has stopped yelling, but the phone may absolutely still choose violence.

## Images

The built images are attached to the GitHub Release, not committed to Git history.

| File | Size | SHA256 |
| --- | ---: | --- |
| `boot.img` | 6.0 MB | `141458acd63c2999c588fa01e6ca94328aa53a9da5358ffd7a680b7e1ed98e41` |
| `system.img` | 967 MB | `9f016367b8fc3ff6a8215d0f71a50a86b93ba2e740a013e9554edeb7b7de75ee` |

Newer local test boot images and TWRP ZIP checksums are documented in `artifacts/checksums/current-build-sha256.txt`.

## Source Included

This repo includes the small, directly edited bring-up source:

- `source/device/samsung/j3xln`
- `source/local_manifests`
- `logs`

The kernel source is large, so it is attached as `j3xln-kernel-source.tar.gz` in the release assets.

The device tree and log bundle are also attached as `j3xln-device-tree-lineage17.1.tar.gz`.

## Manifest

The LineageOS source checkout was initialized with:

```bash
repo init -u https://github.com/LineageOS/android.git -b lineage-17.1 --no-clone-bundle
repo sync -c -j1 --fail-fast --no-tags --no-clone-bundle
```

Manifest branch:

```text
lineage-17.1
```

## Build Commands

The successful build path was:

```bash
cd /home/kenneth/lineage17.1-j3xnlte
source build/envsetup.sh
lunch lineage_j3xln-userdebug
mka bootimage
mka systemimage
```

Final outputs:

```text
out/target/product/j3xln/boot.img
out/target/product/j3xln/system.img
```

## Important Fixes Applied

The imported Android 7.1-era device tree needed basic LineageOS 17.1 product bring-up work:

- Added `lineage_j3xln.mk`.
- Updated `AndroidProducts.mk` and `vendorsetup.sh` lunch choices.
- Removed invalid Android 7 build version overrides from `device.mk`.
- Removed Halium-only package references from the Android 10 product build.
- Added Bluetooth build config header.
- Added `BOARD_KERNEL_IMAGE_NAME := zImage`.
- Added `USE_XML_AUDIO_POLICY_CONF := 1` because Android 10 no longer accepts legacy audio policy `.conf` as the active policy format.
- Repaired Chromium WebView prebuilt by initializing Git LFS for `external/chromium-webview/prebuilt/arm`.
- Matched Samsung boot image offsets to the known-working LineageOS 14.1 boot image.
- Restored the missing Spreadtrum `SPRD` device-tree section in `boot.img`; the broken Android 10 build had `dt_size=0`, while the known-working boot image had a `505856` byte DT section.
- Appended the Samsung `SEANDROIDENFORCE` trailer marker to test boot images, which removed the bootloader warning.
- Removed stale Halium/Android 7 init payloads from the Android 10 system image and replaced `j3xln.rc` with a minimal init file.
- Simplified first-stage ramdisk fstab so it only mounts `SYSTEM`, and removed `encryptable=footer` from the system fstab during the storage/encryption test.

The WebView failure looked like this:

```text
out/target/product/j3xln/obj/APPS/webview_intermediates/package.apk: error: failed opening zip: Invalid file.
veridex E ... Expected valid zip or dex file
```

Root cause: `external/chromium-webview/prebuilt/arm/webview.apk` was only a 133-byte Git LFS pointer, not the real APK.

Fix:

```bash
cd external/chromium-webview/prebuilt/arm
git lfs install --local
git lfs pull
git lfs checkout
```

## Known Risks

- The build emitted `your 131072x1 screen size is bogus. expect trouble`; this likely points to bad inherited framebuffer/display metadata and needs investigation before expecting a usable UI.
- No complete Android 10 vendor blob integration has been proven for this Spreadtrum SM-J320FN target.
- `boot.img` and `system.img` are build artifacts only. They are not yet a signed Odin/TWRP-ready ROM package.
- Flashing raw images to the wrong partition can brick the device. Verify partitions from recovery or `heimdall print-pit` before writing anything.

## Next Debug Steps

1. Produce a debug-heavy boot image that disables panic reboot/watchdog behavior long enough to expose a useful kernel or init failure.
2. Capture serial, ramoops, or a non-overwritten normal-boot `last_kmsg` if possible.
3. Audit Android 10 init/kernel feature assumptions against the old Spreadtrum 3.x kernel tree.
4. Bring vendor blobs forward from the LineageOS 14.1 base or stock firmware.
5. Fix display dimensions and early graphics stack before chasing higher-level Android services.

## Mood

Should I have chosen a modern phone instead of torturing myself with a Spreadtrum Galaxy J3 from 2016? Probably. Someone crowdfund me a device that does not fight back like a cursed toaster.
