# Dependency Policy

`Cargo.lock` is committed. Runtime dependencies stay in the owning crate and
platform-only dependencies use target-specific tables. Default features are
disabled where they materially reduce codecs or backend surface.

## `injector`

| Dependency | Purpose |
| --- | --- |
| `anyhow` | Application error propagation with context |
| `crossbeam-channel` | Typed UI, capture, tray, and worker communication |
| `eframe` | egui application shell and platform renderer |
| `egui_extras` | Image-loader integration |
| `image` | ICO, GIF, PNG, and JPEG decode plus screenshot PNG output |
| `tracing` | Structured application events |
| `tracing-subscriber` | Environment filtering and log/stdout sinks |
| `serde` | Typed configuration serialization |
| `toml` | Configuration file format |
| `dirs` | Operating-system user configuration directory |
| `open` | Open selected AI provider in the system browser |
| `wry` | Integrated WebView browser |
| `tao` | Native event loop/window for the browser binaries |
| `arboard` | Optional screenshot image copy to system clipboard |
| `tray-icon` | Windows system tray |
| `dll-syringe` | Windows DLL injection and raw RPC |
| `windows` | Typed Win32 APIs |
| `windows-capture` | Windows Graphics Capture frames |
| `serde_json` | Parse `hyprctl clients -j` on Linux |
| `winresource` | Embed Windows executable resources at build time |

The Linux-specific `eframe` entry enables its Wayland and X11 backends. Cargo
unifies it with the base `eframe` feature set.

## `payload`

| Dependency | Purpose |
| --- | --- |
| `windows` | Win32 types and window operations exported by the DLL |

## Review procedure

For dependency changes:

1. Confirm the standard library or an existing dependency does not cover the
   need.
2. Limit features and platforms.
3. Update `Cargo.lock`.
4. Run `cargo tree --duplicates`, `cargo audit`, compiler, test, clippy, and
   release gates.
5. Record deferred ecosystem warnings in `PROGRESS.md`.
