use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    #[serde(default)]
    pub banner_url: Option<String>,
    #[serde(default)]
    pub ignore_mouse: bool,
    #[serde(default)]
    pub hide_from_task_manager: bool,
    #[serde(default)]
    pub hidden_windows: HashSet<u32>,
    #[serde(default)]
    pub browser_firefox_hidden: bool,
    #[serde(default)]
    pub browser_edge_hidden: bool,
    #[serde(default)]
    pub browser_chrome_hidden: bool,
    #[serde(default)]
    pub ide_vscode_hidden: bool,
    #[serde(default)]
    pub ide_vs_hidden: bool,
    #[serde(default)]
    pub hook_notepad: bool,
    #[serde(default)]
    pub hook_firefox: bool,
    #[serde(default)]
    pub hook_edge: bool,
    #[serde(default)]
    pub hook_chrome: bool,
    #[serde(default)]
    pub hook_vscode: bool,
    #[serde(default)]
    pub hook_visualstudio: bool,
    #[serde(default)]
    pub hook_antiinterview: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            dark_theme: true,
            show_preview: false,
            hide_from_taskbar: false,
            window_size: (320.0, 540.0),
            banner_url: None,
            ignore_mouse: false,
            hide_from_task_manager: false,
            hidden_windows: HashSet::new(),
            browser_firefox_hidden: false,
            browser_edge_hidden: false,
            browser_chrome_hidden: false,
            ide_vscode_hidden: false,
            ide_vs_hidden: false,
            hook_notepad: false,
            hook_firefox: false,
            hook_edge: false,
            hook_chrome: false,
            hook_vscode: false,
            hook_visualstudio: false,
            hook_antiinterview: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkeys_default() {
        let hotkeys = Hotkeys::default();
        assert_eq!(hotkeys.screenshot, "Ctrl+Shift+S");
        assert_eq!(hotkeys.hide_window, "Ctrl+Shift+H");
        assert_eq!(hotkeys.show_gui, "Ctrl+Shift+I");
    }

    #[test]
    fn test_ui_settings_default() {
        let ui = UiSettings::default();
        assert!(ui.dark_theme);
        assert!(!ui.show_preview);
        assert!(!ui.hide_from_taskbar);
        assert_eq!(ui.window_size, (320.0, 540.0));
    }

    #[test]
    fn test_behavior_default() {
        let behavior = Behavior::default();
        assert!(behavior.auto_refresh);
        assert_eq!(behavior.refresh_interval_ms, 1000);
        assert!(!behavior.minimize_to_tray);
    }

    #[test]
    fn test_configuration_default() {
        let config = Configuration::default();
        assert_eq!(config.hotkeys.screenshot, "Ctrl+Shift+S");
        assert!(config.ui.dark_theme);
        assert!(config.behavior.auto_refresh);
    }

    #[test]
    fn test_configuration_serialization() {
        let config = Configuration::default();
        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("screenshot"));
        assert!(serialized.contains("dark_theme"));
    }

    #[test]
    fn test_configuration_deserialization() {
        let toml_str = r#"
            [hotkeys]
            screenshot = "Ctrl+S"
            hide_window = "Ctrl+H"
            show_gui = "Ctrl+I"
        "#;
        let config: Configuration = toml::from_str(toml_str).unwrap();
        assert_eq!(config.hotkeys.screenshot, "Ctrl+S");
    }
}
