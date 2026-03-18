use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotkeys {
    pub screenshot: String,
    pub hide_window: String,
    pub show_gui: String,
}

impl Default for Hotkeys {
    fn default() -> Self {
        Self {
            screenshot: "Ctrl+Shift+S".to_string(),
            hide_window: "Ctrl+Shift+H".to_string(),
            show_gui: "Ctrl+Shift+I".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub dark_theme: bool,
    pub show_preview: bool,
    pub hide_from_taskbar: bool,
    pub window_size: (f32, f32),
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            dark_theme: true,
            show_preview: false,
            hide_from_taskbar: false,
            window_size: (320.0, 540.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    pub auto_refresh: bool,
    pub refresh_interval_ms: u64,
    pub minimize_to_tray: bool,
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            auto_refresh: true,
            refresh_interval_ms: 1000,
            minimize_to_tray: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(default)]
    pub hotkeys: Hotkeys,
    #[serde(default)]
    pub ui: UiSettings,
    #[serde(default)]
    pub behavior: Behavior,
}
