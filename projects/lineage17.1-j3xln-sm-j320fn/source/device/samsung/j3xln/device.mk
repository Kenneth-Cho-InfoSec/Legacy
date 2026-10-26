# device.mk for Samsung Galaxy J3 2016 (SM-J320FN)
# First-pass LineageOS 17.1 product configuration

PRODUCT_DEVICE := j3xln
PRODUCT_NAME := lineage_j3xln
PRODUCT_BRAND := samsung
PRODUCT_MODEL := SM-J320FN
PRODUCT_MANUFACTURER := samsung
PRODUCT_SHIPPING_API_LEVEL := 22

PRODUCT_PROPERTY_OVERRIDES += \
    ro.adb.secure=0 \
    ro.zygote=zygote32 \
    ro.debuggable=1 \
    ro.hardware=sc9830 \
    ro.product.first_api_level=22 \
    ro.product.device=j3xln \
    ro.product.model=SM-J320FN \
    ro.product.brand=samsung \
    ro.product.name=j3xln \
    ro.product.manufacturer=samsung \
    dalvik.vm.dex2oat-Xms=64m \
    dalvik.vm.dex2oat-Xmx=512m \
    dalvik.vm.heapgrowthlimit=192m \
    dalvik.vm.heapmaxfree=8m \
    dalvik.vm.heapminfree=512k \
    dalvik.vm.heapsize=512m \
    dalvik.vm.heapstartsize=16m \
    dalvik.vm.heaptargetfree=64m \
    ro.security.keystore.keymanager=com.android.keychain

PRODUCT_SYSTEM_PROPERTIES += \
    persist.sys.usb.config=mtp,adb \
    ro.telephony.default_network=9 \
    persist.radio.multisim.config=dsds \
    ro.multisim.simslotcount=2

PRODUCT_COPY_FILES += \
    device/samsung/j3xln/fstab.sc8830.ramdisk:$(TARGET_COPY_OUT_RAMDISK)/fstab.sc8830 \
    device/samsung/j3xln/fstab.sc8830.ramdisk:$(TARGET_COPY_OUT_RAMDISK)/fstab.sc9830 \
    device/samsung/j3xln/fstab.sc9830:system/etc/fstab.sc9830 \
    device/samsung/j3xln/init.j3xln.rc:system/etc/init/j3xln.rc \
    device/samsung/j3xln/ueventd.rc:system/etc/ueventd.rc

PRODUCT_PACKAGES += \
    audio.a2dp.default \
    audio.r_submix.default \
    audio.usb.default \
    bluetooth.default \
    libbt-vendor \
    libcamera \
    libgps \
    libGLESv2 \
    wpa_supplicant \
    wpa_supplicant.conf

TARGET_BOOTANIMATION_FULL_RES := true
