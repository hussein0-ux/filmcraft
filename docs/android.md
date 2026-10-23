# FilmCraft on Android (experimental)

`apps/filmcraft-android` is a first-pass port: eframe's NativeActivity runner on wgpu/Vulkan driving
the same `filmcraft-engine` + `filmcraft-ui-egui` as desktop. **It has not been compiled yet**; expect
to fix build errors on the first run (crates the engine pulls in may need Android cfg-gating).

## Prerequisites
- Android Studio (or command-line tools): SDK platform 34, **NDK r26+**, set `ANDROID_HOME` and `ANDROID_NDK_ROOT`
- `rustup target add aarch64-linux-android`
- `cargo install cargo-apk` (CI uses it: `.github/workflows/android.yml`)
- Phone with USB debugging on (Vulkan 1.1+ required)

## Build and run
```sh
cargo apk run -p filmcraft-android --lib            # debug, installs and launches on the connected phone
cargo apk build -p filmcraft-android --lib --release # APK in target/release/apk/
```
Tip: use `--release`; the editor is very slow in a debug build.

## Not done yet
| Area | Desktop | Android work needed |
|---|---|---|
| File import/export | rfd dialogs | Storage Access Framework (`content://` URIs) via JNI; engine wants paths, so copy into app storage |
| Audio out / mic | cpal | cpal's Android (oboe) backend, plus RECORD_AUDIO permission |
| HW decode | `filmcraft-platform` | MediaCodec backend; software decoders are the fallback |
| Input | mouse + keyboard | touch targets, soft keyboard, pinch zoom on timeline |
| Layout | 1600x980 window | phone-sized docking / responsive panels |
| Control server | TCP | not needed |
