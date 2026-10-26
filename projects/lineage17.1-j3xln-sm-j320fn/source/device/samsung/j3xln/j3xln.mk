# j3xln.mk - Product makefile for Samsung Galaxy J3 2016 (SM-J320FN)
# Halium 7.1 Compatible

# ==============================================
# Device name
# ==============================================
PRODUCT_DEVICE := j3xln

# ==============================================
# Inherit common makefiles
# ==============================================
$(call inherit-product, $(SRC_TARGET_DIR)/product/full_base.mk)

# ==============================================
# Inherit device makefile
# ==============================================
$(call inherit-product, $(LOCAL_PATH)/device.mk)

# ==============================================
# Device-specific configurations
# ==============================================
PRODUCT_NAME := j3xln
PRODUCT_DEVICE := j3xln
PRODUCT_BRAND := samsung
PRODUCT_MODEL := SM-J320FN
PRODUCT_MANUFACTURER := samsung

# ==============================================
# Build info
# ==============================================
PRODUCT_BUILD_PROP_OVERRIDES += \
    BUILD_ID=NMF26X \
    BUILD_DISPLAY_ID=NMF26X.test-keys \
    PRIVATE_BUILD_DESC="j3xln-userdebug 7.1.1 NMF26X test-keys eng.root.20170427.184818 release-keys"

# ==============================================
# OTA/SELinux - use default if files don't exist
# ==============================================