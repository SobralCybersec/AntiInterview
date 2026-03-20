use crate::application::WindowRepository;
use crate::domain::{ProcessId, Window, WindowId};
use anyhow::{Context, Result};
use dll_syringe::{
    process::{BorrowedProcessModule, OwnedProcess, Process},
    rpc::{RawRpcFunctionPtr, RemoteRawProcedure},
    Syringe,
};
use std::{env, path::PathBuf};
use tracing::debug;
use windows::{
    core::BOOL,
    Win32::{
        Foundation::{HWND, LPARAM, TRUE, WPARAM},
        Graphics::{
            Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED},
            Gdi::{
                DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BI_RGB, BITMAP,
                BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS,
            },
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetClassLongPtrW, GetIconInfo, GetWindowDisplayAffinity,
            GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible, SendMessageW,
            GCLP_HICONSM, HICON, ICONINFO, ICON_SMALL2, WM_GETICON,
        },
    },
};

pub struct WindowsWindowRepository;

impl WindowsWindowRepository {
    pub fn new() -> Self {
        Self
    }

    fn get_dll_path(process: &OwnedProcess) -> Result<PathBuf> {
        let mut dll_path = env::current_exe()?;
        dll_path.pop();

        if cfg!(debug_assertions) && process.runs_under_wow64()? {
            dll_path.push("../i686-pc-windows-msvc/debug/utils.dll");
        } else if process.is_x86()? {
            dll_path.push("utils32.dll");
        } else {
            dll_path.push("utils.dll");
        }

        Ok(dll_path)
    }

    fn get_remote_proc<F: RawRpcFunctionPtr>(
        syringe: &Syringe,
        module: BorrowedProcessModule<'_>,
        proc_name: &str,
    ) -> Result<RemoteRawProcedure<F>> {
        unsafe { syringe.get_raw_procedure::<F>(module, proc_name) }?
            .context(format!("Failed to find procedure {}", proc_name))
    }
}

impl WindowRepository for WindowsWindowRepository {
    fn find_all(&self) -> Result<Vec<Window>> {
        let mut windows = Vec::new();

        unsafe {
            let param = LPARAM(&mut windows as *mut _ as isize);
            EnumWindows(Some(enum_windows_proc), param)?;
        }

        Ok(windows)
    }

    fn get_icon(&self, window_id: &WindowId) -> Option<(usize, usize, Vec<u8>)> {
        let hwnd = HWND(window_id.value() as *mut _);
        let lresult = unsafe { SendMessageW(hwnd, WM_GETICON, Some(WPARAM(ICON_SMALL2 as usize)), None) };

        let hicon = if lresult.0 == 0 {
            let uresult = unsafe { GetClassLongPtrW(hwnd, GCLP_HICONSM) };
            if uresult == 0 {
                return None;
            }
            HICON(uresult as *mut _)
        } else {
            HICON(lresult.0 as *mut _)
        };

        let mut icon_info = ICONINFO::default();
        unsafe { GetIconInfo(hicon, &mut icon_info as *mut _).ok()? };

        let hdc = unsafe { GetDC(None) };
        if hdc.is_invalid() {
            return None;
        }

        let mut bitmap = BITMAP::default();
        let object_result = unsafe {
            GetObjectW(
                icon_info.hbmColor.into(),
                std::mem::size_of::<BITMAP>() as i32,
                Some(&mut bitmap as *mut _ as *mut _),
            )
        };

        if object_result == 0 {
            return None;
        }

        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = bitmap.bmWidth;
        bmi.bmiHeader.biHeight = -bitmap.bmHeight;
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB.0;

        let pixel_count = bitmap.bmWidth * bitmap.bmHeight;
        let mut pixels: Vec<u8> = vec![0; (pixel_count * 4) as usize];
        unsafe {
            GetDIBits(
                hdc,
                icon_info.hbmColor,
                0,
                bitmap.bmHeight as u32,
                Some(pixels.as_mut_ptr() as *mut _),
                &mut bmi as *mut _,
                DIB_RGB_COLORS,
            )
        };

        for i in (0..pixels.len()).step_by(4) {
            (pixels[i], pixels[i + 2]) = (pixels[i + 2], pixels[i]);
        }

        unsafe {
            let _ = ReleaseDC(None, hdc);
            let _ = DeleteObject(icon_info.hbmColor.into());
            let _ = DeleteObject(icon_info.hbmMask.into());
        }

        Some((bitmap.bmWidth as usize, bitmap.bmHeight as usize, pixels))
    }

    fn set_visibility(
        &self,
        process_id: &ProcessId,
        window_id: &WindowId,
        hidden: bool,
        hide_from_taskbar: Option<bool>,
    ) -> Result<()> {
        let target_process = OwnedProcess::from_pid(process_id.value())
            .map_err(|e| anyhow::anyhow!("Failed to open process {}: {:?}", process_id, e))?;
        
        let dll_path = Self::get_dll_path(&target_process)
            .map_err(|e| anyhow::anyhow!("Failed to get DLL path: {:?}", e))?;
        
        debug!("DLL path: {:?}", dll_path);
        
        if !dll_path.exists() {
            return Err(anyhow::anyhow!("DLL not found at {:?}", dll_path));
        }
        
        let syringe = Syringe::for_process(target_process);
        
        let module = match syringe.find_or_inject(&dll_path) {
            Ok(m) => m,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to inject DLL into process {} (PID: {}). This process may have security protections. Error: {:?}",
                    process_id,
                    process_id.value(),
                    e
                ));
            }
        };

        let visibility_proc = Self::get_remote_proc::<extern "system" fn(u32, bool) -> bool>(
            &syringe,
            module,
            "SetWindowVisibility",
        )?;

        visibility_proc.call(window_id.value(), hidden)
            .map_err(|e| anyhow::anyhow!("Failed to call SetWindowVisibility: {:?}", e))?;

        if let Some(hide_taskbar) = hide_from_taskbar {
            let taskbar_proc = Self::get_remote_proc::<extern "system" fn(u32, bool) -> bool>(
                &syringe,
                module,
                "HideFromTaskbar",
            )?;
            taskbar_proc.call(window_id.value(), hide_taskbar)
                .map_err(|e| anyhow::anyhow!("Failed to call HideFromTaskbar: {:?}", e))?;
        }

        Ok(())
    }

    fn set_process_stealth(&self, process_id: &ProcessId) -> Result<()> {
        use windows::Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
            PROCESSENTRY32W, TH32CS_SNAPPROCESS,
        };
        
        let process_name = match process_id.value() {
            1 => "firefox.exe".to_string(),
            2 => "msedge.exe".to_string(),
            3 => "chrome.exe".to_string(),
            4 => "Code.exe".to_string(),
            5 => "devenv.exe".to_string(),
            0 => return Ok(()),
            pid => {
                let _target_process = OwnedProcess::from_pid(pid)
                    .map_err(|e| anyhow::anyhow!("Failed to open target process: {:?}", e))?;
                
                let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }
                    .map_err(|e| anyhow::anyhow!("Failed to create snapshot: {:?}", e))?;
                
                let mut found_name = String::new();
                let mut entry = PROCESSENTRY32W {
                    dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                    ..Default::default()
                };
                
                if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
                    loop {
                        if entry.th32ProcessID == pid {
                            let name_len = entry.szExeFile.iter()
                                .position(|&c| c == 0)
                                .unwrap_or(entry.szExeFile.len());
                            found_name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);
                            break;
                        }
                        
                        if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                            break;
                        }
                    }
                }
                
                if found_name.is_empty() {
                    return Err(anyhow::anyhow!("Failed to get process name for PID {}", pid));
                }
                
                found_name
            }
        };
        
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }
            .map_err(|e| anyhow::anyhow!("Failed to create process snapshot: {:?}", e))?;
        
        let mut target_pids = Vec::new();
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        
        if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
            loop {
                let name_len = entry.szExeFile.iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());
                let name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);
                
                if name.to_lowercase() == process_name.to_lowercase() {
                    target_pids.push(entry.th32ProcessID);
                }
                
                if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                    break;
                }
            }
        }
        
        if target_pids.is_empty() {
            return Ok(());
        }
        
        let all_windows = self.find_all()?;
        let mut hidden_count = 0;
        
        for window in all_windows {
            if target_pids.contains(&window.process_id().value())
                && self.set_visibility(
                    window.process_id(),
                    window.id(),
                    false,
                    Some(true),
                ).is_ok() {
                    hidden_count += 1;
                }
        }
        
        debug!("Hidden {} window(s) for: {}", hidden_count, process_name);
        Ok(())
    }

    fn inject_hook_dll(&self, dll_name: &str) -> Result<()> {
        use windows::Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
            PROCESSENTRY32W, TH32CS_SNAPPROCESS,
        };
        
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }
            .map_err(|e| anyhow::anyhow!("Failed to create snapshot: {:?}", e))?;
        
        let mut taskmgr_pid = 0u32;
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        
        if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
            loop {
                let name_len = entry.szExeFile.iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());
                let name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);
                
                if name.to_lowercase() == "taskmgr.exe" {
                    taskmgr_pid = entry.th32ProcessID;
                    break;
                }
                
                if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                    break;
                }
            }
        }
        
        if taskmgr_pid == 0 {
            return Err(anyhow::anyhow!("Task Manager not running"));
        }
        
        let target_process = OwnedProcess::from_pid(taskmgr_pid)
            .map_err(|e| anyhow::anyhow!("Failed to open Task Manager: {:?}", e))?;
        
        let mut dll_path = env::current_exe()?;
        dll_path.pop();
        dll_path.push(dll_name);
        
        if !dll_path.exists() {
            return Err(anyhow::anyhow!("Hook DLL not found: {:?}", dll_path));
        }
        
        let syringe = Syringe::for_process(target_process);
        
        syringe.inject(&dll_path)
            .map_err(|e| anyhow::anyhow!("Failed to inject {} into Task Manager: {:?}", dll_name, e))?;
        
        debug!("Injected {} into Task Manager (PID: {})", dll_name, taskmgr_pid);
        Ok(())
    }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    if !unsafe { IsWindowVisible(hwnd) }.as_bool() {
        return TRUE;
    }

    let mut buf = [0u16; 128];
    let title_len = unsafe { GetWindowTextW(hwnd, &mut buf) };
    if title_len == 0 {
        return TRUE;
    }

    let title = String::from_utf16_lossy(&buf[..title_len as usize]);

    let mut cloaked: u32 = 0;
    let result = unsafe {
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut cloaked as *mut _ as _,
            std::mem::size_of::<u32>() as u32,
        )
    };

    debug!("Window {:?} cloaked={} title={}", hwnd.0, cloaked, title);

    if result.is_err() || cloaked != 0 {
        return TRUE;
    }

    let mut affinity: u32 = 0;
    if unsafe { GetWindowDisplayAffinity(hwnd, &mut affinity as *mut _) }.is_err() {
        return TRUE;
    }

    let mut pid = 0u32;
    if unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) } == 0 {
        return TRUE;
    }

    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
        PROCESSENTRY32W, TH32CS_SNAPPROCESS,
    };
    
    let process_name = if let Ok(snapshot) = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        
        let mut found_name = String::new();
        if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
            loop {
                if entry.th32ProcessID == pid {
                    let name_len = entry.szExeFile.iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry.szExeFile.len());
                    found_name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);
                    break;
                }
                
                if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                    break;
                }
            }
        }
        
        if found_name.is_empty() {
            pid.to_string()
        } else {
            found_name
        }
    } else {
        pid.to_string()
    };
    
    debug!("  -> PID={} process_name={}", pid, process_name);

    let windows: &mut Vec<Window> = unsafe { &mut *(lparam.0 as *mut _) };
    windows.push(Window::new(
        WindowId::new(hwnd.0 as u32),
        title,
        ProcessId::new(pid),
        process_name,
        affinity != 0,
    ));

    TRUE
}
