use crate::application::{ConfigurationRepository, WindowService};
use crate::domain::{Configuration, ProcessId, Window, WindowId};
use crate::infrastructure::{CaptureManager, TrayEvent, TrayManager, ProcessMonitor};
use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{ColorImage, Context, TextureHandle};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info};
use windows_capture::monitor::Monitor;

pub enum AppCommand {
    RefreshWindows,
    ToggleVisibility {
        process_id: ProcessId,
        window_id: WindowId,
        hidden: bool,
        hide_from_taskbar: Option<bool>,
    },
    SetProcessStealth {
        process_id: ProcessId,
    },
}

pub struct AppState {
    windows: Arc<Mutex<Vec<Window>>>,
    window_service: Arc<WindowService>,
    config_repo: Arc<dyn ConfigurationRepository + Send + Sync>,
    config: Configuration,
    command_tx: Sender<AppCommand>,
    capture_manager: CaptureManager,
    tray_manager: Option<TrayManager>,
    process_monitor: ProcessMonitor,
    monitors: Vec<Monitor>,
    active_monitor: usize,
    capture_texture: Option<TextureHandle>,
    icon_cache: HashMap<u32, Option<TextureHandle>>,
    filter_text: String,
    frame_counter: u32,
    window_visible: bool,
}

impl AppState {
    pub fn set_process_stealth(&self, process_id: ProcessId) {
        let _ = self.command_tx.send(AppCommand::SetProcessStealth { process_id });
    }
    pub fn new(
        window_service: Arc<WindowService>,
        config_repo: Arc<dyn ConfigurationRepository + Send + Sync>,
    ) -> Result<Self> {
        let config = config_repo.load()?;
        let windows = Arc::new(Mutex::new(Vec::new()));
        let (command_tx, command_rx) = crossbeam_channel::unbounded();

        Self::spawn_command_processor(command_rx, windows.clone(), window_service.clone());

        let monitors = Monitor::enumerate().unwrap_or_default();
        let capture_manager = CaptureManager::new();

        if config.ui.show_preview && !monitors.is_empty() {
            capture_manager.start_capture(monitors[0]);
        }

        let tray_manager = if config.behavior.minimize_to_tray {
            TrayManager::new().ok()
        } else {
            None
        };

        let process_monitor = ProcessMonitor::new(window_service.clone());
        
        let mut enabled_hooks = Vec::new();
        if config.ui.hook_notepad {
            enabled_hooks.push("hook_notepad.dll".to_string());
        }
        if config.ui.hook_firefox {
            enabled_hooks.push("hook_firefox.dll".to_string());
        }
        if config.ui.hook_edge {
            enabled_hooks.push("hook_edge.dll".to_string());
        }
        if config.ui.hook_chrome {
            enabled_hooks.push("hook_chrome.dll".to_string());
        }
        if config.ui.hook_vscode {
            enabled_hooks.push("hook_vscode.dll".to_string());
        }
        if config.ui.hook_visualstudio {
            enabled_hooks.push("hook_visualstudio.dll".to_string());
        }
        if config.ui.hook_antiinterview {
            enabled_hooks.push("hook_antiinterview.dll".to_string());
        }
        
        process_monitor.update_enabled_hooks(enabled_hooks);
        process_monitor.start();

        let state = Self {
            windows,
            window_service,
            config_repo,
            config,
            command_tx,
            capture_manager,
            tray_manager,
            process_monitor,
            monitors,
            active_monitor: 0,
            capture_texture: None,
            icon_cache: HashMap::new(),
            filter_text: String::new(),
            frame_counter: 0,
            window_visible: true,
        };
        
        state.refresh_windows();
        
        Ok(state)
    }

    fn spawn_command_processor(
        command_rx: Receiver<AppCommand>,
        windows: Arc<Mutex<Vec<Window>>>,
        window_service: Arc<WindowService>,
    ) {
        std::thread::spawn(move || {
            for command in command_rx {
                match command {
                    AppCommand::RefreshWindows => {
                        debug!("Refreshing windows");
                        match window_service.list_windows() {
                            Ok(new_windows) => {
                                let mut windows_lock = windows.lock().unwrap();
                                let old_hidden: std::collections::HashMap<u32, bool> = windows_lock
                                    .iter()
                                    .map(|w| (w.id().value(), w.is_hidden()))
                                    .collect();
                                
                                *windows_lock = new_windows;
                                
                                for window in windows_lock.iter_mut() {
                                    if let Some(&was_hidden) = old_hidden.get(&window.id().value()) {
                                        window.set_hidden(was_hidden);
                                    }
                                }
                                debug!("Windows refreshed");
                            }
                            Err(e) => error!("Failed to refresh windows: {:?}", e),
                        }
                    }
                    AppCommand::SetProcessStealth { process_id } => {
                        info!("Setting process stealth for PID: {}", process_id);
                        
                        if let Err(e) = window_service.set_process_stealth(&process_id) {
                            error!("Failed to set process stealth: {:?}", e);
                        }
                    }
                    AppCommand::ToggleVisibility {
                        process_id,
                        window_id,
                        hidden,
                        hide_from_taskbar,
                    } => {
                        info!("Toggling visibility for window {:?}", window_id);
                        if let Err(e) = window_service.toggle_visibility(
                            &process_id,
                            &window_id,
                            hidden,
                            hide_from_taskbar,
                        ) {
                            error!("Failed to toggle visibility: {:?}", e);
                        }
                    }
                }
            }
        });
    }

    pub fn config(&self) -> &Configuration {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Configuration {
        &mut self.config
    }

    pub fn save_config(&self) -> Result<()> {
        let result = self.config_repo.save(&self.config);
        
        if result.is_ok() {
            let mut enabled_hooks = Vec::new();
            if self.config.ui.hook_notepad {
                enabled_hooks.push("hook_notepad.dll".to_string());
            }
            if self.config.ui.hook_firefox {
                enabled_hooks.push("hook_firefox.dll".to_string());
            }
            if self.config.ui.hook_edge {
                enabled_hooks.push("hook_edge.dll".to_string());
            }
            if self.config.ui.hook_chrome {
                enabled_hooks.push("hook_chrome.dll".to_string());
            }
            if self.config.ui.hook_vscode {
                enabled_hooks.push("hook_vscode.dll".to_string());
            }
            if self.config.ui.hook_visualstudio {
                enabled_hooks.push("hook_visualstudio.dll".to_string());
            }
            if self.config.ui.hook_antiinterview {
                enabled_hooks.push("hook_antiinterview.dll".to_string());
            }
            
            self.process_monitor.update_enabled_hooks(enabled_hooks);
        }
        
        result
    }

    pub fn refresh_windows(&self) {
        let _ = self.command_tx.send(AppCommand::RefreshWindows);
    }

    pub fn toggle_window_visibility(
        &self,
        process_id: ProcessId,
        window_id: WindowId,
        hidden: bool,
        hide_from_taskbar: Option<bool>,
    ) {
        let _ = self.command_tx.send(AppCommand::ToggleVisibility {
            process_id,
            window_id,
            hidden,
            hide_from_taskbar,
        });
    }

    pub fn get_windows(&self) -> Vec<Window> {
        let mut windows = self.windows.lock().unwrap().clone();
        
        for window in windows.iter_mut() {
            if self.config.ui.hidden_windows.contains(&window.id().value()) {
                window.set_hidden(true);
            }
        }
        
        windows
    }

    pub fn update_window_hidden(&mut self, window_id: &WindowId, hidden: bool) {
        if let Some(window) = self
            .windows
            .lock()
            .unwrap()
            .iter_mut()
            .find(|w| w.id() == window_id)
        {
            window.set_hidden(hidden);
        }
        
        if hidden {
            self.config.ui.hidden_windows.insert(window_id.value());
        } else {
            self.config.ui.hidden_windows.remove(&window_id.value());
        }
        let _ = self.save_config();
    }

    pub fn filter_text(&self) -> &str {
        &self.filter_text
    }

    pub fn set_filter_text(&mut self, text: String) {
        self.filter_text = text;
    }

    pub fn get_filtered_windows(&self) -> Vec<Window> {
        let windows = self.get_windows();
        self.window_service
            .filter_windows(&windows, &self.filter_text)
    }

    pub fn get_icon(&mut self, ctx: &Context, window_id: &WindowId) -> Option<&TextureHandle> {
        let id_value = window_id.value();

        if !self.icon_cache.contains_key(&id_value) {
            let icon = self.window_service.get_icon(window_id).map(|(w, h, pixels)| {
                let image = ColorImage::from_rgba_unmultiplied([w, h], &pixels);
                ctx.load_texture("icon", image, eframe::egui::TextureOptions::LINEAR)
            });
            self.icon_cache.insert(id_value, icon);
        }

        self.icon_cache.get(&id_value).and_then(|opt| opt.as_ref())
    }

    pub fn update_capture_texture(&mut self, ctx: &Context) {
        self.frame_counter += 1;
        if !self.frame_counter.is_multiple_of(2) {
            return;
        }

        if let Some(img) = self.capture_manager.try_recv_image() {
            if let Some(texture) = &mut self.capture_texture {
                texture.set(img, eframe::egui::TextureOptions::NEAREST);
            } else {
                self.capture_texture = Some(ctx.load_texture(
                    "screen_capture",
                    img,
                    eframe::egui::TextureOptions::NEAREST,
                ));
            }
        }
    }

    pub fn capture_texture(&self) -> Option<&TextureHandle> {
        self.capture_texture.as_ref()
    }

    pub fn monitors(&self) -> &[Monitor] {
        &self.monitors
    }

    pub fn active_monitor(&self) -> usize {
        self.active_monitor
    }

    pub fn set_active_monitor(&mut self, index: usize) {
        if index < self.monitors.len() {
            self.active_monitor = index;
            self.capture_manager.start_capture(self.monitors[index]);
        }
    }

    pub fn start_capture(&self) {
        if !self.monitors.is_empty() {
            self.capture_manager.start_capture(self.monitors[self.active_monitor]);
        }
    }

    pub fn stop_capture(&self) {
        self.capture_manager.stop_capture();
        let _ = self.save_config();
    }

    pub fn toggle_preview(&mut self, enabled: bool) {
        self.config.ui.show_preview = enabled;
        if enabled {
            self.start_capture();
        } else {
            self.stop_capture();
            self.capture_texture = None;
        }
    }

    pub fn check_tray_events(&mut self) -> Option<TrayEvent> {
        self.tray_manager.as_ref().and_then(|tm| tm.check_events())
    }

    pub fn set_window_visible(&mut self, visible: bool) {
        self.window_visible = visible;
    }

    pub fn hide_browser_windows(&self, title_pattern: &str) {
        let windows = self.get_windows();
        let pattern_lower = title_pattern.to_lowercase();
        let mut hidden_count = 0;
        
        info!("Attempting to hide windows with pattern: {}", title_pattern);
        
        for window in &windows {
            let window_title = window.title().to_lowercase();
            let window_process = window.process_name().to_lowercase();
            
            // Match by title OR process name
            if window_title.contains(&pattern_lower) || window_process.contains(&pattern_lower) {
                
                info!("  Hiding: title='{}' process='{}' pid={}", 
                      window.title(), window.process_name(), window.process_id());
                
                let hide_taskbar = if self.config.ui.hide_from_taskbar { Some(true) } else { None };
                
                let _ = self.command_tx.send(AppCommand::ToggleVisibility {
                    process_id: window.process_id().clone(),
                    window_id: window.id().clone(),
                    hidden: true,
                    hide_from_taskbar: hide_taskbar,
                });
                
                hidden_count += 1;
            }
        }
        
        if hidden_count > 0 {
            info!("Hidden {} window(s) with pattern: {}", hidden_count, title_pattern);
        } else {
            info!("No windows found with pattern: {}", title_pattern);
        }
    }
    
    pub fn show_browser_windows(&self, title_pattern: &str) {
        let windows = self.get_windows();
        let pattern_lower = title_pattern.to_lowercase();
        let mut shown_count = 0;
        
        info!("Attempting to show windows with title containing: {}", title_pattern);
        
        for window in &windows {
            let window_title = window.title().to_lowercase();
            let window_process = window.process_name().to_lowercase();
            
            // Match by title OR process name
            if window_title.contains(&pattern_lower) || window_process.contains(&pattern_lower) {
                
                info!("  Showing: title='{}' process='{}' pid={}", 
                      window.title(), window.process_name(), window.process_id());
                
                let _ = self.command_tx.send(AppCommand::ToggleVisibility {
                    process_id: window.process_id().clone(),
                    window_id: window.id().clone(),
                    hidden: false,
                    hide_from_taskbar: Some(false),
                });
                
                shown_count += 1;
            }
        }
        
        if shown_count > 0 {
            info!("Shown {} window(s) with title containing: {}", shown_count, title_pattern);
        } else {
            info!("No windows found with title containing: {}", title_pattern);
        }
    }

    pub fn inject_hook_dll(&self, dll_name: &str) -> Result<()> {
        self.window_service.inject_hook_dll(dll_name)
    }
}
