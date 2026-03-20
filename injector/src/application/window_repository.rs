use crate::domain::{ProcessId, Window, WindowId};
use anyhow::Result;

pub trait WindowRepository {
    fn find_all(&self) -> Result<Vec<Window>>;
    fn get_icon(&self, window_id: &WindowId) -> Option<(usize, usize, Vec<u8>)>;
    fn set_visibility(
        &self,
        process_id: &ProcessId,
        window_id: &WindowId,
        hidden: bool,
        hide_from_taskbar: Option<bool>,
    ) -> Result<()>;
    fn set_process_stealth(&self, process_id: &ProcessId) -> Result<()>;
    fn inject_hook_dll(&self, dll_name: &str) -> Result<()>;
}
