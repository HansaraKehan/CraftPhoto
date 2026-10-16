<img width="2304" height="1440" alt="Screenshot_20261016_160858" src="https://github.com/user-attachments/assets/f083d83d-0111-4d27-a9e5-06b2fbf065dd" />
# PhotoCraft for Android (English, auto-updated)

An unofficial Android build of [PhotoCraft](https://github.com/storytold/photocraft), the open-source
image editor written in Rust. It is a native app (egui + wgpu), not a web page in a WebView, so it
runs full-screen with touch, pen and keyboard/mouse support.

This repository does not contain PhotoCraft's source. A scheduled GitHub Actions workflow downloads the
newest upstream source, adds the Android layer and an English translation, builds the APK and publishes it.

## Download

Get the latest `.apk` from the [Releases](https://github.com/HansaraKehan/CraftPhoto/releases) page, open it on your device and allow the
install when Android asks. You need:

- an ARM64 phone or tablet (most devices from the last several years)
- Android 9 or newer
- a GPU with Vulkan or OpenGL ES 3

If an update won't install over an older build, uninstall the old one first.

## How it stays up to date

- Every day the workflow checks upstream `storytold/photocraft` for a new commit.
- When there is one, it builds a new APK and publishes a release named `android-v<version>-<commit>`.
- To build right away, open **Actions → Build and release Android APK → Run workflow**. You can enter an
  upstream branch or tag, or tick *force* to rebuild a version that already has a release.
- To follow a branch or tag other than `main`, add a repository variable named `UPSTREAM_REF`
  (Settings → Secrets and variables → Actions → Variables).

If upstream changes code the Android layer also touches, the build stops with a list of the
conflicting files and the patch in `android-layer/` needs updating.

## Signing

Builds are signed with a key the workflow creates and caches. For a permanent key, add the secrets
`ANDROID_KEYSTORE_B64` (a PKCS12 keystore, base64-encoded, with alias `photocraft`) and
`ANDROID_KEYSTORE_PASSWORD`. APKs signed with different keys can't update each other.

## What is in `android-layer/`

`android-layer.tar.gz` holds everything added on top of upstream:

- a patch to PhotoCraft's own code (touch gestures, phone layouts, menus)
- the Android app crate, launcher code and file-manager bridge
- a patched `winit` for hardware mouse and keyboard input
- `translate_en.py`, which turns the remaining Chinese interface text into English at build time

## Credits and licence

- PhotoCraft is by [storytold](https://github.com/storytold/photocraft), under MIT or Apache-2.0.
- The native Android port this build is based on is
  [SnowOwlLeader/photocraft-for-android](https://github.com/SnowOwlLeader/photocraft-for-android).
- Licences are bundled inside the APK. See `ATTRIBUTION-ANDROID.md` in the built source for the third-party notice.

This project is not affiliated with or supported by the PhotoCraft or ArtCraft team.
