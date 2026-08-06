# Architecture

## Workspace

The Cargo workspace contains:

- `injector`: desktop application, presentation, screen capture, window
  operations, browser launcher, configuration, and hook orchestration.
- `payload`: Windows `cdylib` exporting the supported native window-control
  functions.
- `hook`: C++ Microsoft Detours DLLs used by the optional Task Manager process
  filtering features.
- `linux`: Hyprland rule engine and its shell self-test.

The application uses four Rust layers:

- `domain`: configuration and window value types.
- `application`: repository contracts and `WindowService` use cases.
- `infrastructure`: Windows, Linux, capture, tray, process-monitor, and
  file-backed repository implementations.
- `presentation`: egui state, screens, browser launcher, video rendering, and
  UI-facing tests.

`injector/src/main.rs` is the composition root. It selects the platform
repository, creates services, loads configuration, and starts the UI.

## Main flows

### Window visibility

`AntiInterviewApp` sends a request through `AppState`, `WindowService`,
and `WindowRepository`. Windows uses Win32 APIs and optional injected DLLs.
Linux resolves the Hyprland window class and invokes
`anti-interview-wayland.sh`.

### Screen preview and screenshots

`CaptureManager` owns a capture-control thread. A two-frame bounded channel
and non-blocking send provide backpressure. `AppState` updates one egui
texture and writes a PNG to the operating-system temporary directory only when
the user requests a silent screenshot.

### Commands and lifecycle

`AppState` sends typed `AppCommand` values to a worker thread. Capture uses
typed `CaptureCommand` values. Channel disconnects end worker loops, active
capture handles stop on replacement or drop, and `ProcessMonitor::drop` clears
its running flag.

### Configuration

`FileConfigurationRepository` serializes `Configuration` as TOML at
`dirs::config_dir()/AntiInterview/config.toml`. Serde defaults preserve
compatibility when newer fields are absent. `config.example.toml` documents
user-editable values.

### Browser and network

The AI browser opens fixed HTTPS provider URLs in WRY or the system browser.
The optional banner URL is loaded by egui. The project has no backend or
project-operated network service.

## Platform capabilities

| Capability | Windows | Linux |
| --- | --- | --- |
| List windows | Win32 | `hyprctl clients -j` |
| Exclude windows from capture | Win32/DLL path | Hyprland `no_screen_share` rule |
| Live preview | Windows Graphics Capture | Stubbed |
| Tray | Native tray | Stubbed |
| Process/DLL hooks | Supported, optional | Not supported |

Unsupported Linux operations return explicit errors or use narrow no-op stubs
where the shared UI requires a component.

## Contracts and state

This desktop application has no database, public HTTP API, or JavaScript IPC
contract. Rust types define internal commands, configuration, and repository
boundaries. The exported payload functions use `extern "system"` and Win32
types; changes to those signatures require coordinated Rust/C++ validation.

## Performance and observability

Tracing writes structured events to `anti-interview.log`; debug builds also
write to stdout. The `profiling` Cargo profile keeps release optimization and
debug symbols. Hot paths avoid unbounded frame queues, reuse capture padding
buffers, cache decoded textures/icons, and schedule GIF repaint at frame
deadlines.
