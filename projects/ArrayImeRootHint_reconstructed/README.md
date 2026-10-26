# Array IME Root Hint Reconstruction

This is a reconstructed Android Studio project generated from:

`C:\Users\kenneth\Downloads\行列輸入法 -- 字根提示版_0.0.8_1 (24.08.20.11.51)_APKPure.apk`

## What Is Included

- Kotlin Android project scaffold using Gradle Kotlin DSL.
- Kotlin rewrites for small app-facing classes:
  - `BaseInputView`
  - `HzimSettingsActivity`
  - `HzimKeyboardView`
- Decompiled Java for the larger recovered IME engine and keyboard implementation.
- Decoded Android resources and `assets/ImeData` dictionary tables.
- Debug build output verified with `./gradlew assembleDebug`.

## Notes

The APK was obfuscated and several Jadx outputs needed manual repair before the code would compile. The project is buildable, but some repaired decompiler artifacts should be reviewed before treating this as source-equivalent production code, especially the custom keyboard parser/view code.

Jadx also exported compiled nine-patch files in a form that AAPT could not recompile. Those images were kept under the same resource names but converted to regular PNG filenames so the app can build.

## Build

From this directory:

```powershell
.\gradlew.bat assembleDebug
```

The debug APK is generated at:

`app\build\outputs\apk\debug\app-debug.apk`
