mkdir debug

dx bundle --package dsot_mobile --platform android --target aarch64-linux-android

cp target/dx/dsot_mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk ./debug/dsot_mobile.debug.apk

if (adb devices | lines | get 1 | str contains "device") { adb install ./debug/dsot_mobile.debug.apk }
