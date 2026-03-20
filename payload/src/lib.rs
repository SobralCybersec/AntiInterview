use std::os::windows::process::CommandExt;
use std::process::Command;
use windows::Win32::{
    Foundation::{COLORREF, HWND, RECT},
    UI::WindowsAndMessaging::{
        FlashWindow, GetWindowLongW, GetWindowRect, SetLayeredWindowAttributes,
        SetWindowDisplayAffinity, SetWindowLongW, SetWindowPos, ShowCursor, ShowWindow,
        GWL_EXSTYLE, HWND_NOTOPMOST, HWND_TOPMOST, LWA_ALPHA, SWP_FRAMECHANGED, SWP_NOMOVE,
        SWP_NOSIZE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, WDA_EXCLUDEFROMCAPTURE, WDA_NONE,
        WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT,
    },
};

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

static mut HIDDEN_PROCESS_NAME: Option<String> = None;

/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure that `process_name` points to a valid null-terminated UTF-16 string.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn SetHiddenProcessName(process_name: *const u16) -> bool {
    if process_name.is_null() {
        return false;
    }

    let mut len = 0;
    while *process_name.add(len) != 0 {
        len += 1;
    }
    let name = String::from_utf16_lossy(std::slice::from_raw_parts(process_name, len));
    HIDDEN_PROCESS_NAME = Some(name);
    true
}

/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure that `buffer` points to a valid buffer with at least `buffer_size` elements.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetHiddenProcessName(buffer: *mut u16, buffer_size: u32) -> u32 {
    if let Some(ref name) = HIDDEN_PROCESS_NAME {
        let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let copy_len = wide.len().min(buffer_size as usize);
        if !buffer.is_null() && buffer_size > 0 {
            std::ptr::copy_nonoverlapping(wide.as_ptr(), buffer, copy_len);
        }
        wide.len() as u32
    } else {
        0
    }
}

/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure that `exe_path` points to a valid null-terminated UTF-16 string.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn LaunchDetached(exe_path: *const u16) -> bool {
    if exe_path.is_null() {
        return false;
    }

    let exe = {
        let mut len = 0;
        while *exe_path.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(exe_path, len))
    };

    Command::new(exe)
        .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
        .spawn()
        .is_ok()
}

#[unsafe(no_mangle)]
pub extern "system" fn SetWindowVisibility(hwnd: HWND, hide: bool) -> bool {
    if hwnd.0.is_null() {
        return false;
    }

    let dwaffinity = if hide {
        WDA_EXCLUDEFROMCAPTURE
    } else {
        WDA_NONE
    };

    unsafe { SetWindowDisplayAffinity(hwnd, dwaffinity) }.is_ok()
}

#[unsafe(no_mangle)]
pub extern "system" fn HideFromTaskbar(hwnd: HWND, hide: bool) -> bool {
    if hwnd.0.is_null() {
        return false;
    }

    let mut style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) };

    if hide {
        style |= WS_EX_TOOLWINDOW.0 as i32;
        style &= !(WS_EX_APPWINDOW.0 as i32);
    } else {
        style |= WS_EX_APPWINDOW.0 as i32;
        style &= !(WS_EX_TOOLWINDOW.0 as i32);
    }

    unsafe {
        SetWindowLongW(hwnd, GWL_EXSTYLE, style);

        SetWindowPos(
            hwnd,
            None,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED,
        )
        .is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn IgnoreMouse(hwnd: HWND, ignore: bool) -> bool {
    if hwnd.0.is_null() {
        return false;
    }

    let mut style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) };

    if ignore {
        style |= WS_EX_TRANSPARENT.0 as i32;
    } else {
        style &= !(WS_EX_TRANSPARENT.0 as i32);
    }

    unsafe {
        SetWindowLongW(hwnd, GWL_EXSTYLE, style);
        SetWindowPos(hwnd, None, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED).is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn SetWindowOpacity(hwnd: HWND, opacity: u8) -> bool {
    if hwnd.0.is_null() {
        return false;
    }

    let mut style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) };
    style |= WS_EX_LAYERED.0 as i32;

    unsafe {
        SetWindowLongW(hwnd, GWL_EXSTYLE, style);
        SetLayeredWindowAttributes(hwnd, COLORREF(0), opacity, LWA_ALPHA).is_ok()
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn SetWindowAlwaysOnTop(hwnd: HWND, always_on_top: bool) -> bool {
    if hwnd.0.is_null() {
        return false;
    }

    let hwnd_insert = if always_on_top {
        Some(HWND_TOPMOST)
    } else {
        Some(HWND_NOTOPMOST)
    };

    unsafe { SetWindowPos(hwnd, hwnd_insert, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE) }.is_ok()
}

#[unsafe(no_mangle)]
pub extern "system" fn MinimizeWindow(hwnd: HWND) -> bool {
    unsafe { ShowWindow(hwnd, SW_MINIMIZE) }.as_bool()
}

#[unsafe(no_mangle)]
pub extern "system" fn MaximizeWindow(hwnd: HWND) -> bool {
    unsafe { ShowWindow(hwnd, SW_MAXIMIZE) }.as_bool()
}

#[unsafe(no_mangle)]
pub extern "system" fn RestoreWindow(hwnd: HWND) -> bool {
    unsafe { ShowWindow(hwnd, SW_RESTORE) }.as_bool()
}

#[unsafe(no_mangle)]
pub extern "system" fn SetWindowPosition(hwnd: HWND, x: i32, y: i32) -> bool {
    unsafe { SetWindowPos(hwnd, None, x, y, 0, 0, SWP_NOSIZE) }.is_ok()
}

/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure that `rect` points to a valid RECT structure.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetWindowRectangle(hwnd: HWND, rect: *mut RECT) -> bool {
    if rect.is_null() {
        return false;
    }

    GetWindowRect(hwnd, rect).is_ok()
}

#[unsafe(no_mangle)]
pub extern "system" fn FlashWindowInTaskbar(hwnd: HWND) -> bool {
    unsafe { FlashWindow(hwnd, true) }.as_bool()
}

#[unsafe(no_mangle)]
pub extern "system" fn HideCursor(hide: bool) -> bool {
    unsafe {
        if hide {
            while ShowCursor(false) >= 0 {}
        } else {
            while ShowCursor(true) < 0 {}
        }
    }
    true
}

