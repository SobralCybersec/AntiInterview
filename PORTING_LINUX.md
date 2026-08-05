# Anti-Interview — Linux / Wayland Port Research

Status: research complete (2026-08). Verified live on Hyprland 0.56.0 (Omarchy, Lua config provider).

## The core problem

The whole product is one Windows call: `SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE)`.
DWM enforces it at the compositor for **every** capture path, and the window becomes
**truly absent** — the capture shows whatever is behind it (desktop, other windows). No trace.

Linux has no universal equivalent, and the reason is structural, not a missing feature.

## Why Linux can't do true "show-behind" exclusion (2026)

Two independent walls:

1. **No single capture chokepoint.**
   - X11: capture is a raw read of the root-window framebuffer (`XGetImage`). Any client reads any
     pixel. **Per-window exclusion is impossible.** Only defense = isolation (run the app on a
     separate X server / Xephyr the capturer isn't connected to).
   - Wayland: capture is portal-mediated (`xdg-desktop-portal` → PipeWire). The compositor is the
     sole pixel source, so it *can* omit a window — but only per-compositor, and only as a blackout.

2. **Occlusion culling kills "show what's behind."** Wayland tells surfaces to stop drawing when
   covered, so the content behind your hidden window **isn't in any buffer**. To reveal it, a
   compositor would have to force every occluded-behind surface to keep rendering (defeating the
   power optimization) **and** run a second occlusion-free composite for the capture path. No
   shipping compositor does this. Windows works only because DWM centrally holds every window's
   content regardless of occlusion.

Consequence: even **app-side masking can't fake it** — cut the window's region out of a full-screen
capture and the pixels behind were never captured; you can only fill the hole with a fabrication
(blur, wallpaper grab, solid color), not the real desktop.

**Black rectangle is the practical ceiling for in-place exclusion on Wayland in 2026.**
The window's shape and position still leak — a black box *is* a shape leak.

## Compositor support matrix (2026, forum-verified)

| Environment | In-place hide? | Mechanism | Persist | Killer caveat |
|---|:--:|---|:--:|---|
| **Hyprland 0.50+** | ✅ blackout | `no_screen_share` window rule | ✅ | Floating-window **zoom leak** (open, Jan 2026) — force tiled |
| **niri 25.05+** | ✅ blackout | `block-out-from "screen-capture"` | ✅ | `"screencast"` scope is screenshot-bypassable; use `"screen-capture"` |
| **KWin 6.7+** | ✅ blackout | KWin script sets `window.excludeFromCapture` | ✅ | 6.6.x session-only; window-rule persistence landed in 6.7 (MR !8828) |
| **GNOME/Mutter** | ❌ | none — confirmed May 2026 | — | Extensions hide only the *indicator*, not the window |
| **Sway / river / Wayfire / labwc** | ❌ | wlroots screencopy grabs whole output | — | No exclusion concept |
| **X11 (any WM)** | ❌ | framebuffer read | — | Only escape: isolated Xephyr / 2nd X server |

Pattern: the compositors that **left wlroots** (Hyprland/aquamarine, niri/Smithay) are the only ones
that built this. There is **no shared Wayland protocol** for capture-exclusion and none emerging —
every backend is bespoke. `security-context-v1` is unrelated (it gates sandboxed-client access, not
per-window hiding). `ext-image-copy-capture-v1` standardizes *how* you capture, not exclusion.

## Architectural note: we hide someone else's window

Anti-Interview hides Chrome/VSCode/Firefox — not itself. On Windows that's fine
(`SetWindowDisplayAffinity` works on foreign HWNDs). On Wayland **no client can flag another
client's surface** (Electron `setContentProtection`, KeePassXC protection = Linux no-ops). The
saving grace: compositor rules match by **`app-id`/`class`**, so an external tool drives the
compositor ("black out any window of class X") rather than touching the foreign surface. That's the
only viable model, and it exists on exactly three compositors.

## Zero-trace alternatives (when black-box isn't acceptable)

Ranked for the anti-interview case (interviewer typically demands whole-screen share):

1. **Second physical monitor** — share monitor A, helper on B. Genuinely zero-trace, fully usable.
   Most reliable. Needs a 2nd display.
2. **Headless virtual output** — `hyprctl output create headless`, view on a phone/tablet over VNC.
   Zero-trace on the shared screen. Flaky mirroring (0.52.2 black-screen reports); needs a 2nd device.
3. **Share a single window** — zero-trace but defeats the anti-interview case (they control the share).
4. **True absence via Hyprland plugin** — write the second render-omit pass as a C++ Hyprland plugin
   hooking the screencopy path. This is the only route to real Windows-style behavior, and it's a
   major project against Hyprland internals, per-compositor.

## Verified working on this machine

Hyprland 0.56.0 / Omarchy / Lua config provider (`o.window` API, HyprMod).
Omarchy already ships the pattern for Bitwarden:

```lua
o.window("^(Bitwarden)$", { no_screen_share = true, tag = "+floating-window" })
```

Firefox hidden via one line in `~/.config/hypr/hyprland.lua`:

```lua
o.window("firefox", { no_screen_share = true })
```

`hyprctl reload` → confirmed black rectangle in OBS PipeWire capture, Firefox normal on the local
display. Firefox was native-Wayland + tiled, dodging both the XWayland uncertainty and the
floating-zoom leak.

Runtime notes for the port:
- `hyprctl keyword` is **blocked** under the new-parser / Lua config — can't inject rules at runtime
  that way. Plain-conf Hyprland setups *can* use `hyprctl keyword windowrulev2`.
- `hyprctl setprop address:0x… no_screen_share 1` returned `unknown request` on 0.56 — the token is a
  window **rule**, not a settable dynamic prop on this build (contradicts older maintainer threads;
  version-dependent — probe at runtime, don't assume).
- Token is `no_screen_share` (new parser) — confirmed in the Hyprland 0.56 binary. Legacy
  `noscreenshare` also exists. Wiki has shipped the wrong token historically.
- `getprop` is a command, not a dispatcher: `hyprctl getprop address:0x… no_screen_share`.

## Recommended integration

A `platform/` trait layer (`CaptureHider`, `WindowManager`, `Tray`, `ProcessHider`) with a runtime
compositor-detect (`$HYPRLAND_INSTANCE_SIGNATURE` / niri socket / `$XDG_CURRENT_DESKTOP` / X11
fallback). Windows `hook/*.cpp` + `dll-syringe` tree stays behind `#[cfg(windows)]`, untouched.

Backend priority: Hyprland (blackout, MVP) → niri → KWin. On GNOME/Sway/X11, **detect and tell the
user plainly it can't hide here** — offer the 2nd-monitor / headless path. A privacy tool that
silently no-ops gets someone caught; honest degradation is the product.

UI must label every Linux hide **best-effort blackout, not the Windows guarantee** — shape leaks,
zoom/screenshot tools can defeat it, and a phone camera always can.
