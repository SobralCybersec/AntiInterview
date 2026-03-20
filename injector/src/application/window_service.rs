use crate::application::WindowRepository;
use crate::domain::{ProcessId, Window, WindowId};
use anyhow::Result;
use std::sync::Arc;

pub struct WindowService {
    repository: Arc<dyn WindowRepository + Send + Sync>,
}

impl WindowService {
    pub fn new(repository: Arc<dyn WindowRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub fn inject_hook_dll(&self, dll_name: &str) -> Result<()> {
        self.repository.inject_hook_dll(dll_name)
    }

    pub fn list_windows(&self) -> Result<Vec<Window>> {
        self.repository.find_all()
    }

    pub fn filter_windows(&self, windows: &[Window], filter: &str) -> Vec<Window> {
        windows
            .iter()
            .filter(|w| w.matches_filter(filter))
            .cloned()
            .collect()
    }

    pub fn toggle_visibility(
        &self,
        process_id: &ProcessId,
        window_id: &WindowId,
        hidden: bool,
        hide_from_taskbar: Option<bool>,
    ) -> Result<()> {
        self.repository
            .set_visibility(process_id, window_id, hidden, hide_from_taskbar)
    }

    pub fn get_icon(&self, window_id: &WindowId) -> Option<(usize, usize, Vec<u8>)> {
        self.repository.get_icon(window_id)
    }

    pub fn set_process_stealth(&self, process_id: &ProcessId) -> Result<()> {
        self.repository.set_process_stealth(process_id)
    }
}
