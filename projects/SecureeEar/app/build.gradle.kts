plugins {
    id("com.android.application")
}

android {
    namespace = "com.kennethchoinfosec.secureeear"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.kennethchoinfosec.secureeear"
        minSdk = 23
        targetSdk = 34
        versionCode = 1
        versionName = "1.0.0"

        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

dependencies {
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.activity:activity:1.9.3")
    implementation("androidx.annotation:annotation:1.9.1")
}
