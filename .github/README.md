<div align="center">

# 🎨 PhotoCraft · Android Pen & Touch Edition

**A Photcraft-style open-source image editor on your Android tablet, built for a stylus.**

An unofficial native Android build (egui + wgpu) of [PhotoCraft](https://github.com/storytold/photocraft). It runs full-screen with native touch, active pen, and keyboard/mouse support.

[What's new](WHATS-NEW.md) · [Get the app](#-get-the-app) · [Settings guide](#-settings-guide) · [App Suite Downloads](#-app-downloads)

<img width="2304" height="1440" alt="Screenshot_20261021_232623" src="https://github.com/user-attachments/assets/446f134a-1675-4b85-990c-211706215c7f" />


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
Get the latest `.apk` from the [Releases](https://github.com/HansaraKehan/CraftPhoto/releases) page, open it on your device and allow the
install when Android asks. You need:

- an ARM64 phone or tablet (most devices from the last several years)
- Android 9 or newer
- a GPU with Vulkan or OpenGL ES 3

If an update won't install over an older build, uninstall the old one first.

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

<img width="2304" height="1440" alt="Screenshot_20261021_232834" src="https://github.com/user-attachments/assets/2d63e236-7405-4d59-ab3e-c7f6afd81048" />

---

## ℹ️ Status

This edition is under active testing. The code was written without a local compiler, so each build is the first real test — expect a few fix-up rounds. Bug reports with build logs or a short description of what happened are very welcome.

---

## 📜 Credits & Licence

- PhotoCraft is © its authors, dual-licensed under **MIT** / **Apache-2.0** — see `LICENSE-MIT`, `LICENSE-APACHE`, and `ATTRIBUTION.md`.
- The native Android port this build is based on is [SnowOwlLeader/photocraft-for-android](https://github.com/SnowOwlLeader/photocraft-for-android).
- The Android layer and the Pen & Touch module are provided under the same terms. Licences are bundled inside the APK; see `ATTRIBUTION-ANDROID.md` in the built source for the third-party notice.

*This project is an unofficial community build and is not affiliated with or supported by the PhotoCraft or ArtCraft team.*
