# 🆕 What's new

Newest first. Everything below is part of the Android Pen & Touch layer.

---

## v3.0 · Updates, press counts and the pen pointer
* 🔔 **Update checker.** The app looks for a newer build on this repository's Releases page at start (at most every 6 hours, silent offline) and offers **Download / Later / Skip this version**. The app now declares the Internet permission for this.
* 🔢 **Press-count shortcuts.** Each pen button has *Press*, *Press twice* and *Press 3 times* actions, each a key, a combination, a tool or *Switch between recent tools*. Doing the same tool press again returns to the previous tool, so double press can pick Eraser and double press again goes back. A *Next press within* setting (default 300 ms) controls the window. A hold cancels a pending count.
* ⭕ **Pen pointer ring.** Preferences → Tools → Pen & Touch → *Pen pointer*: show a ring where the pen hovers and touches, and pick its size (8–120 pt). Off by default.
* 👋 **Discord link replaced.** The start page button, the About dialog and the Help menu entry now invite people to follow the developer on GitHub (https://github.com/HansaraKehan) instead of upstream's Discord.
* 🧪 More tests for the button state machine (double and triple presses, repeat-to-go-back, hold cancelling a count).

---

## v2.2 · Scrolling fix
* 🐛 **Touching panels and windows no longer changes your tool.** On active-stylus devices a finger used to switch to the Move tool wherever it touched, which could rebuild the Brush or Layers window mid-scroll. Now it only happens for a finger that lands on the canvas.

## v2.1 · One-finger canvas pan
* ✋ **Pen only mode: one finger pans the canvas.** Two fingers still pan, zoom and rotate.
* The "pan" switch in Preferences now covers both one and two fingers.

## v2.0 · Soft keys, recent tools, and the finger fix
* ⌨️ **Soft keys moved into the top bar**, to the right of Layers / Options, instead of a third bar. They contract with «.
* 🔁 **Pen button 1 now switches between recent tools by default.** Press once to go to the previous tool, press again to return. Available for any pen button or soft key as *Switch between recent tools*.
* 🧹 **Undo and Redo removed from the strip** (the top bar already has them). Your saved settings are cleaned up once; keys you added yourself stay.
* 🐛 **Fingers are no longer mistaken for pens on Android.** Every finger reports pressure on Android, which made the app treat fingers as pens. That caused two bugs:
  * Pen only mode did not block fingers.
  * A held finger never right-clicked, even with the option off.
  Both are fixed. A held finger right-clicks again unless *Holding a finger stays a held press* is on.
* 🐛 Fixed a build error: simulated Ctrl/Alt/Shift now reach the editor through `ModifiersChanged` events.
* 🔧 The layer now targets the **current upstream** (0.11.1) instead of an October 15 snapshot, so the build no longer needs `UPSTREAM_REF`.
* 🧪 The hook script is anchor-based: if upstream moves the lines it edits, the build stops with a message naming the missing line instead of silently dropping a feature.

## v1.0 · First release
* 🖊️ **Pen barrel buttons.** Press and hold are separate actions, each a key or a combination. Modifier combinations latch (press to hold, press again to release).
* ⌨️ **Soft-key strip** with Ctrl, Alt, Shift, Enter and Esc, editable in Preferences, with Record and Pick helpers and a limit of 24 keys.
* 🔀 **Finger ⇄ Pen only** button for quick switching.
* 🧭 **Two-finger canvas navigation** in Pen only mode: pan, zoom and rotate about the midpoint, each with its own switch.
* 👆 **Active-stylus devices:** a finger drags items with the Move tool while the pen keeps painting; a held finger can stay a held press instead of right-clicking.
* 🧰 **Device question at start-up** (*with / without an active stylus*) with a **Don't ask again** button, also in Preferences → Tools → Pen & Touch. Devices without an active stylus never get Enter pressed for you.
* 🤖 **Android stylus support** in the window layer: barrel buttons arrive as pen buttons, and stylus tip and eraser contacts are told apart from fingers.
* 💾 All settings are saved with your preferences.

---

### Known limits
* Written without a local compiler: each build is the first real test.
* Rotating or transforming a selection by touch gestures was designed and then dropped at your request.
* Labels in the new screens are English only for now.
