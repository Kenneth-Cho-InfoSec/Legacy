# LineageOS 17.1 product makefile for Samsung Galaxy J3 2016 SM-J320FN.

$(call inherit-product, $(SRC_TARGET_DIR)/product/full_base.mk)
$(call inherit-product, vendor/lineage/config/common_full_phone.mk)
$(call inherit-product, $(LOCAL_PATH)/device.mk)

PRODUCT_NAME := lineage_j3xln
PRODUCT_DEVICE := j3xln
PRODUCT_BRAND := samsung
PRODUCT_MODEL := SM-J320FN
PRODUCT_MANUFACTURER := samsung
