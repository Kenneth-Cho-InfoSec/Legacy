# Build Notes

Workspace:

```text
/home/kenneth/lineage17.1-j3xnlte
```

Successful lunch target:

```text
lineage_j3xln-userdebug
```

Successful platform output:

```text
PLATFORM_VERSION=10
LINEAGE_VERSION=17.1-20260514-UNOFFICIAL-j3xln
TARGET_ARCH=arm
TARGET_ARCH_VARIANT=armv7-a-neon
TARGET_CPU_VARIANT=cortex-a7
```

Successful images:

```text
out/target/product/j3xln/boot.img
out/target/product/j3xln/system.img
```

Build logs are included in `logs/`.

## Current Device Test Notes

Real-device boot testing is in progress and currently blocked before ADB. The latest source snapshot includes:

- Android 10-safe minimal `init.j3xln.rc`.
- No Halium init file copied into `/system/etc/init`.
- First-stage ramdisk fstab reduced to `SYSTEM` only.
- System fstab `/data` entry changed away from `encryptable=footer` for testing.
- Boot image experiments with Samsung marker and restored Spreadtrum DT table are documented in `current-boot-debug-status.md`.
