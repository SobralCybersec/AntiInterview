pub mod file_configuration_repository;
pub use file_configuration_repository::FileConfigurationRepository;

// Tray (tray-icon crate) — Windows-only; Linux stubs it (avoids GTK3/libxdo).
#[cfg(windows)]
pub mod tray_manager;
#[cfg(windows)]
pub use tray_manager::{TrayEvent, TrayManager};

// Windows-only infrastructure: Win32 window ops, Graphics.Capture preview,
// NtQuerySystemInformation hooks, ToolHelp process enumeration.
#[cfg(windows)]
pub mod process_hooks;
#[cfg(windows)]
pub mod process_monitor;
#[cfg(windows)]
pub mod screen_capture;
#[cfg(windows)]
pub mod windows_window_repository;

#[cfg(windows)]
pub use process_monitor::ProcessMonitor;
#[cfg(windows)]
pub use screen_capture::CaptureManager;
#[cfg(windows)]
pub use windows_capture::monitor::Monitor;
#[cfg(windows)]
pub use windows_window_repository::WindowsWindowRepository;

// Non-Windows: no-op stand-ins so the shared presentation layer builds unchanged.
#[cfg(not(windows))]
pub mod linux_stubs;
#[cfg(not(windows))]
pub use linux_stubs::{CaptureManager, Monitor, ProcessMonitor, TrayEvent, TrayManager};

// Linux/Hyprland window repository (drives no_screen_share via the engine script).
#[cfg(target_os = "linux")]
pub mod linux_window_repository;
#[cfg(target_os = "linux")]
pub use linux_window_repository::LinuxWindowRepository;
