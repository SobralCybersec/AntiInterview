use crate::application::WindowService;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, info, warn};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

pub struct ProcessMonitor {
    window_service: Arc<WindowService>,
    enabled_hooks: Arc<Mutex<Vec<String>>>,
    last_taskmgr_pid: Arc<Mutex<Option<u32>>>,
    running: Arc<Mutex<bool>>,
}

impl ProcessMonitor {
    pub fn new(window_service: Arc<WindowService>) -> Self {
        Self {
            window_service,
            enabled_hooks: Arc::new(Mutex::new(Vec::new())),
            last_taskmgr_pid: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn update_enabled_hooks(&self, hooks: Vec<String>) {
        debug!("Updated enabled hooks: {:?}", hooks);
        *self.enabled_hooks.lock().unwrap() = hooks;
    }

    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
            return;
        }
        *running = true;
        drop(running);

        let window_service = self.window_service.clone();
        let enabled_hooks = self.enabled_hooks.clone();
        let last_taskmgr_pid = self.last_taskmgr_pid.clone();
        let running = self.running.clone();

        std::thread::spawn(move || {
            info!("Process monitor started");
            
            loop {
                if !*running.lock().unwrap() {
                    break;
                }

                std::thread::sleep(Duration::from_secs(2));

                if let Some(current_pid) = Self::find_taskmgr_pid() {
                    let mut last_pid = last_taskmgr_pid.lock().unwrap();
                    
                    if *last_pid != Some(current_pid) {
                        info!("New Task Manager detected (PID: {})", current_pid);
                        *last_pid = Some(current_pid);
                        drop(last_pid);

                        std::thread::sleep(Duration::from_millis(500));

                        let hooks = enabled_hooks.lock().unwrap().clone();
                        for hook_dll in hooks {
                            match window_service.inject_hook_dll(&hook_dll) {
                                Ok(_) => info!("Auto-injected {} into Task Manager", hook_dll),
                                Err(e) => warn!("Failed to auto-inject {}: {:?}", hook_dll, e),
                            }
                        }
                    }
                } else {
                    let mut last_pid = last_taskmgr_pid.lock().unwrap();
                    if last_pid.is_some() {
                        debug!("Task Manager closed");
                        *last_pid = None;
                    }
                }
            }

            info!("Process monitor stopped");
        });
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    fn find_taskmgr_pid() -> Option<u32> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;

            let mut entry = PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };

            if Process32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let name_len = entry
                        .szExeFile
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry.szExeFile.len());
                    let name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);

                    if name.to_lowercase() == "taskmgr.exe" {
                        return Some(entry.th32ProcessID);
                    }

                    if Process32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }
        }

        None
    }
}

impl Drop for ProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
