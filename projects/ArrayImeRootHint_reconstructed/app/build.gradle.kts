plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.minn.android.hanziime"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.minn.android.hanziime"
        minSdk = 16
        targetSdk = 34
        versionCode = 1904162136
        versionName = "0.0.8_1 (24.08.20.11.51)"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }
}
