# product.mk for Samsung Galaxy J3 2016 (SM-J320FN)
# Product configuration

# ==============================================
# Device product info
# ==============================================
PRODUCT_DEVICE := j3xln
PRODUCT_NAME := j3xln
PRODUCT_BRAND := samsung
PRODUCT_MODEL := SM-J320FN
PRODUCT_MANUFACTURER := samsung

# ==============================================
# SDK version
# ==============================================
PRODUCT_AAPT_PREF_CONFIG := xxhdpi
PRODUCT_AAPT_PREBUILT_DPI := 320

# ==============================================
# Packaged languages
# ==============================================
PRODUCT_PACKAGES += \
    libstrings \
    libicuuc \
    libicui18n

# ==============================================
# locales
# ==============================================
PRODUCT_LOCALES := en_US en_GB fr_FR de_DE es_ES it_IT pt_BR pt_PT ru_RU uk_UA ar_EG zh_CN zh_TW ja_JP ko_KR

# ==============================================
# Keyboard layouts
# ==============================================
PRODUCT_PACKAGES += \
    LatinIME

# ==============================================
# Fonts
# ==============================================
PRODUCT_COPY_FILES += \
    prebuilts/fonts/roboto/hinted/Roboto-Regular.ttf:system/fonts/Roboto-Regular.ttf \
    prebuilts/fonts/roboto/hinted/Roboto-Bold.ttf:system/fonts/Roboto-Bold.ttf

# ==============================================
# Default app
# ==============================================
PRODUCT_PROPERTY_OVERRIDES += \
    ro.build.build_id=NMF26X \
    ro.build.version.incremental=eng.root.20170427.184818 \
    ro.build.version.release=7.1.1

# ==============================================
# Regional settings
# ==============================================
PRODUCT_PROPERTY_OVERRIDES += \
    ro.product.locale=en-US \
    ro.gps.region_set=auto

# ==============================================
# Default ringtone
# ==============================================
PRODUCT_COPY_FILES += \
    frameworks/native/data/sounds/Alarm.ogg:system/media/audio/alarms/Alarm.ogg \
    frameworks/native/data/sounds/Notification.ogg:system/media/audio/notifications/Notification.ogg \
    frameworks/native/data/sounds/Ringtone.ogg:system/media/audio/ringtones/Ringtone.ogg