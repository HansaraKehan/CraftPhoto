<div align="center">

# 🎨 PhotoCraft · Android Pen & Touch Edition

**A Photoshop-style open-source image editor on your Android tablet, built for a stylus.**

An unofficial native Android build (egui + wgpu) of [PhotoCraft](https://github.com/storytold/photocraft). It runs full-screen with native touch, active pen, and keyboard/mouse support.

[What's new](WHATS-NEW.md) · [Get the app](#-get-the-app) · [Settings guide](#-settings-guide) · [Troubleshooting](#-troubleshooting) · [App Suite Downloads](#-app-downloads)

<img width="2304" height="1440" alt="Screenshot_20261016_160858" src="https://github.com/user-attachments/assets/f083d83d-0111-4d27-a9e5-06b2fbf065dd" />

</div>

---

## ✨ Highlights

| | Feature | What it does |
|---|---|---|
| 🖊️ | **Pen barrel buttons** | Press the button for one action, hold it for another. Both can be key combinations. |
| 🔁 | **Recent-tool switch** | Press the pen button once to jump to your previous tool, press again to come back. |
| ⌨️ | **Soft keys in top bar** | Ctrl, Alt, Shift, Enter, Esc and any shortcut you choose, always one tap away. Contracts when not needed. |
| 🔒 | **Latching modifiers** | Tap Ctrl/Alt/Shift once to hold it, tap again to release. Works from the pen button too. |
| ✋ | **Finger ⇄ Pen only** | One button switches modes. In Pen only mode, fingers never draw. |
| 🧭 | **Finger navigation** | In Pen only mode: one finger pans the canvas, two fingers pan, zoom, and rotate it. |
| 👆 | **Active-stylus mode** | On devices with an active stylus, a finger drags items and the pen paints. |
| ⚙️ | **Everything is switchable** | Each behavior has its own on/off switch in Preferences. |

---

## 📲 Get the App

This repository builds an Android APK on GitHub — no computer or IDE needed.

1. Open the **Actions** tab → **Android release** (or **Build and release Android APK**) → **Run workflow**.
2. Choose the branch or tag that holds your `android-layer/android-layer.tar.gz`.
3. When the run finishes, download the APK from **Releases** (or the run's artifacts) and install it.

> 💡 **How it works:** The workflow downloads the latest upstream [PhotoCraft](https://github.com/storytold/photocraft), applies the Android layer from `android-layer/android-layer.tar.gz` and the English translation script (`translate_en.py`) on top, and builds the result. Your features live in that one file.

### Requirements
- An ARM64 phone or tablet (most devices from recent years)
- Android 9 or newer
- A GPU with Vulkan or OpenGL ES 3 support

### Installation & Download
1. Get the latest `.apk` from the [Releases](https://github.com/HansaraKehan/CraftPhoto/releases) page.
2. Open it on your Android device and allow installation when asked.
3. *Note:* If an update will not install over an older build, uninstall the old version first.

---

## 📊 App Downloads

| Application | Description | Downloads | Link |
| :--- | :--- | :--- | :--- |
| **PhotoCraft** | Image editing app | [![Downloads](https://img.shields.io/github/downloads/HansaraKehan/CraftPhoto/total?style=flat-square&color=blue)](https://github.com/HansaraKehan/CraftPhoto/releases) | [Releases](https://github.com/HansaraKehan/CraftPhoto/releases) |
| **LightCraft** | Light & theme utility | [![Downloads](https://img.shields.io/github/downloads/HansaraKehan/Craftlight/total?style=flat-square&color=orange)](https://github.com/HansaraKehan/Craftlight/releases) | [Releases](https://github.com/HansaraKehan/Craftlight/releases) |
| **WordCraft** | Text utility app | [![Downloads](https://img.shields.io/github/downloads/HansaraKehan/CraftWord/total?style=flat-square&color=green)](https://github.com/HansaraKehan/CraftWord/releases) | [Releases](https://github.com/HansaraKehan/CraftWord/releases) |

---

## 🚀 How Builds & Updates Work

- **Daily Auto-Updates**: Every day the workflow checks upstream `storytold/photocraft` for a new commit. When detected, it builds a new APK and publishes a release named `android-v<version>-<commit>`.
- **Manual Trigger**: Open **Actions → Build and release Android APK → Run workflow**. You can enter an upstream branch or tag, or tick *force* to rebuild a version that already has a release.
- **Staying Up to Date / Pinning Upstream**:
  - The layer follows upstream's newest code. If upstream changes the same lines the layer edits, the run stops with **"The Android patch no longer applies"**. Send the log and the layer gets ported again.
  - To freeze a known-good version, add a repository variable `UPSTREAM_REF` (Settings → Secrets and variables → Actions → Variables) set to an upstream commit hash or branch. Delete it to follow the latest upstream again.

### Signing
Builds are signed with a key the workflow creates and caches. For a permanent key, add the secrets `ANDROID_KEYSTORE_B64` (a PKCS12 keystore, base64-encoded, with alias `photocraft`) and `ANDROID_KEYSTORE_PASSWORD`. APKs signed with different keys cannot update each other.

---

## 🖐️ How it feels to use

### Pen button
| You do | Default action |
|---|---|
| **Press** pen button 1 | Switch to the previous tool. Press again to go back. |
| **Hold** pen button 1 | Holds **Space** (pan the canvas) for as long as you hold it. |
| Pen button 2 | Off by default. Press = `Ctrl+Z`, hold = `Alt` once enabled. |

Any action can be a single key (`E`), a combination (`Ctrl+Shift+Z`), a modifier latch (`Ctrl`), or the special **Switch between recent tools** (`@recent`).

### Top-bar strip
```
« │ Finger │ Ctrl │ Alt │ Shift │ Enter │ Esc │ …
```
* **«** contracts the strip; **»** brings it back. The Finger / Pen only button always stays.
* **Ctrl / Alt / Shift** latch: lit = held.
* Other keys are pressed while your finger is on them (hold Space to pan, quick tap for one press).
* **…** opens the soft-key editor.

### Finger vs. Pen only
| Mode | Pen | One finger | Two fingers |
|---|---|---|---|
| **Finger** | Draws | Draws | Tool-dependent |
| **Pen only** | Draws | **Pans** the canvas | **Pan, zoom, rotate** the canvas |

Fingers on panels, menus and windows always work normally in both modes.

---

## ⚙️ Settings guide

Open **Preferences → Tools → Pen & Touch**. Changes apply immediately.

| Setting | Meaning |
|---|---|
| **Device** | *With an active stylus* or *Without*. Asked at start-up (with **Don't ask again**). Fingers-move behaviour and the held-finger option need an active stylus. |
| **Pen button 1 / 2** | Enable, choose the press action and the hold action, set the hold delay. |
| **Input mode** | Finger or Pen only (same as the top-bar button). |
| **Two fingers zoom / rotate / pan** | Turn each canvas gesture on or off. |
| **A finger tap clicks and a drag moves the item** | Active-stylus devices: fingers borrow the Move tool, the pen keeps painting. |
| **Holding a finger stays a held press** | Active-stylus devices: no right-click on a held finger. Off = a held finger right-clicks. |
| **Soft keys** | Show/hide the strip; add, rename, remove or reset keys. Each key accepts a key combination. |

### Key combination cheat sheet
| Write | Result |
|---|---|
| `E` | Press E |
| `Ctrl+Shift+Z` | Redo |
| `Ctrl` or `Alt+Shift` | Latch: first press holds, next press releases |
| `Space` | Pressed while the button is held |
| `@recent` | Switch between the last two tools |

Use **Record** to capture a key by pressing it, or **Pick** to choose from the list of commands.

---

## 🛠️ Troubleshooting

| Problem | Try this |
|---|---|
| **Build says the patch no longer applies** | Upstream changed. Set `UPSTREAM_REF` to a good commit, or send the log to have the layer ported. |
| **Build fails with a Rust error** | Copy the first red `error[...]` lines from the failed run and send them. |
| **Pen button does nothing** | Check *Use this button* is on for that button in Preferences → Tools → Pen & Touch. |
| **Fingers still draw in Pen only mode** | Confirm the strip shows **Pen only** (highlighted). Fingers on panels are meant to work. |
| **A window won't scroll by touch** | Turn off *A finger tap clicks and a drag moves the item* and test again. Report which windows are affected. |
| **Wrong device question at start** | Change it under Preferences → Tools → Pen & Touch → Device. |

---

## 🧱 What's inside `android-layer/`

`android-layer.tar.gz` holds everything added on top of upstream:

```
android-layer/
└── android-layer.tar.gz
    ├── android-layer.patch       Android edits to upstream (layout, touch UI, dialogs)
    ├── translate_en.py           Runs after the patch; turns Chinese UI text into English at build time
    └── overlay/
        ├── crates/ui-egui/src/pen_touch/   Pen buttons, soft keys, modes, settings
        ├── vendor/winit/…                  Stylus support & patched winit for hardware mouse/keyboard
        └── apps/photocraft-android/…       The Android app shell, launcher, top bar, and file-manager bridge
```

More detail: [`docs/pen-touch.md`](docs/pen-touch.md).

---

## ℹ️ Status

This edition is under active testing. The code was written without a local compiler, so each build is the first real test — expect a few fix-up rounds. Bug reports with build logs or a short description of what happened are very welcome.

---

## 📜 Credits & Licence

- PhotoCraft is © its authors, dual-licensed under **MIT** / **Apache-2.0** — see `LICENSE-MIT`, `LICENSE-APACHE`, and `ATTRIBUTION.md`.
- The native Android port this build is based on is [SnowOwlLeader/photocraft-for-android](https://github.com/SnowOwlLeader/photocraft-for-android).
- The Android layer and the Pen & Touch module are provided under the same terms. Licences are bundled inside the APK; see `ATTRIBUTION-ANDROID.md` in the built source for the third-party notice.

*This project is an unofficial community build and is not affiliated with or supported by the PhotoCraft or ArtCraft team.*
