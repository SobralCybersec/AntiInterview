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
            .map_err(|e| anyhow::anyhow!("Failed to open process: {:?}", e))?;
        let dll_path = Self::get_dll_path(&target_process)?;
        let syringe = Syringe::for_process(target_process);
        let module = syringe.find_or_inject(dll_path)
            .map_err(|e| anyhow::anyhow!("Failed to inject DLL: {:?}", e))?;

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

    let windows: &mut Vec<Window> = unsafe { &mut *(lparam.0 as *mut _) };
    windows.push(Window::new(
        WindowId::new(hwnd.0 as u32),
        title,
        ProcessId::new(pid),
        affinity != 0,
    ));

    TRUE
}
