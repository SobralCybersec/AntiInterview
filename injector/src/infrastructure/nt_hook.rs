use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Interface to the C++ NT API hook DLL
pub struct NtHook {
    _dll: libloading::Library,
    set_hidden_fn: libloading::Symbol<'static, unsafe extern "C" fn(*const u16) -> bool>,
}

impl NtHook {
    /// Load the hook DLL and get function pointers
    pub fn load() -> Result<Self> {
        let dll_path = Self::get_hook_dll_path()?;
        
        debug!("Loading hook DLL from: {:?}", dll_path);
        
        let dll = unsafe { libloading::Library::new(&dll_path)? };
        
        let set_hidden_fn: libloading::Symbol<'static, unsafe extern "C" fn(*const u16) -> bool> = unsafe {
            let symbol: libloading::Symbol<unsafe extern "C" fn(*const u16) -> bool> = dll.get(b"SetHiddenProcessName\0")?;
            std::mem::transmute(symbol)
        };
        
        info!("NT Hook DLL loaded successfully");
        
        Ok(Self {
            _dll: dll,
            set_hidden_fn,
        })
    }
    
    /// Set the process name to hide from Task Manager
    pub fn set_hidden_process(&self, process_name: &str) -> Result<()> {
        let name_wide: Vec<u16> = process_name
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        
        let result = unsafe { (self.set_hidden_fn)(name_wide.as_ptr()) };
        
        if result {
            info!("Set hidden process name: {}", process_name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to set hidden process name"))
        }
    }
    
    /// Clear the hidden process name
    pub fn clear_hidden_process(&self) -> Result<()> {
        let result = unsafe { (self.set_hidden_fn)(std::ptr::null()) };
        
        if result {
            info!("Cleared hidden process name");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to clear hidden process name"))
        }
    }
    
    fn get_hook_dll_path() -> Result<PathBuf> {
        let mut path = std::env::current_exe()?;
        path.pop();
        path.push("hook.dll");
        
        if !path.exists() {
            warn!("hook.dll not found at {:?}", path);
            return Err(anyhow::anyhow!("hook.dll not found. Please build it using build_hook.bat"));
        }
        
        Ok(path)
    }
}

/// Check if the hook DLL is available
pub fn is_hook_available() -> bool {
    if let Ok(mut path) = std::env::current_exe() {
        path.pop();
        path.push("hook.dll");
        path.exists()
    } else {
        false
    }
}
