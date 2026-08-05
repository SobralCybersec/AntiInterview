//! No-op Linux stand-ins for Windows-only infrastructure (Graphics.Capture screen
//! preview and the Task-Manager process monitor), so the shared presentation layer
//! compiles unchanged. These features are Windows-only; on Linux the stubs simply
//! do nothing, and the GUI's live-preview panel stays empty.
//!
//! ponytail: stubs over cfg-sprinkling every call site. When Linux preview lands
//! (PipeWire), replace CaptureManager here; the rest of the app needs no change.

use crate::application::WindowService;
use eframe::egui::ColorImage;
use std::sync::Arc;

#[derive(Clone, Copy, Default)]
pub struct Monitor;

impl Monitor {
    pub fn enumerate() -> anyhow::Result<Vec<Monitor>> {
        Ok(Vec::new())
    }
}

#[derive(Default)]
pub struct CaptureManager;

impl CaptureManager {
    pub fn new() -> Self {
        Self
    }
    pub fn start_capture(&self, _monitor: Monitor) {}
    pub fn stop_capture(&self) {}
    pub fn try_recv_image(&self) -> Option<ColorImage> {
        None
    }
}

pub struct ProcessMonitor;

impl ProcessMonitor {
    pub fn new(_window_service: Arc<WindowService>) -> Self {
        Self
    }
    pub fn update_enabled_hooks(&self, _hooks: Vec<String>) {}
    pub fn start(&self) {}
}

// Matched in the GUI but never constructed by the stub (the real tray does).
#[allow(dead_code)]
pub enum TrayEvent {
    Show,
    Quit,
}

pub struct TrayManager;

impl TrayManager {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }
    pub fn check_events(&self) -> Option<TrayEvent> {
        None
    }
}
