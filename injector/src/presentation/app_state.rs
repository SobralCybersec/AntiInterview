use crate::application::{ConfigurationRepository, WindowService};
use crate::domain::{Configuration, ProcessId, Window, WindowId};
use crate::infrastructure::{CaptureManager, TrayEvent, TrayManager};
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
}

pub struct AppState {
    windows: Arc<Mutex<Vec<Window>>>,
    window_service: Arc<WindowService>,
    config_repo: Arc<dyn ConfigurationRepository + Send + Sync>,
    config: Configuration,
    command_tx: Sender<AppCommand>,
    capture_manager: CaptureManager,
    tray_manager: Option<TrayManager>,
    monitors: Vec<Monitor>,
    active_monitor: usize,
    capture_texture: Option<TextureHandle>,
    icon_cache: HashMap<u32, Option<TextureHandle>>,
    filter_text: String,
    frame_counter: u32,
    window_visible: bool,
}

impl AppState {
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

        Ok(Self {
            windows,
            window_service,
            config_repo,
            config,
            command_tx,
            capture_manager,
            tray_manager,
            monitors,
            active_monitor: 0,
            capture_texture: None,
            icon_cache: HashMap::new(),
            filter_text: String::new(),
            frame_counter: 0,
            window_visible: true,
        })
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
                                *windows.lock().unwrap() = new_windows;
                                debug!("Windows refreshed");
                            }
                            Err(e) => error!("Failed to refresh windows: {:?}", e),
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
        self.config_repo.save(&self.config)
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
        self.windows.lock().unwrap().clone()
    }

    pub fn update_window_hidden(&self, window_id: &WindowId, hidden: bool) {
        if let Some(window) = self
            .windows
            .lock()
            .unwrap()
            .iter_mut()
            .find(|w| w.id() == window_id)
        {
            window.set_hidden(hidden);
        }
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
        if self.frame_counter % 2 != 0 {
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
}
