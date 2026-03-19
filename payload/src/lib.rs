use windows::Win32::{
    Foundation::{HWND, RECT, COLORREF},
    UI::WindowsAndMessaging::{
        GWL_EXSTYLE, GetWindowLongW, SetWindowDisplayAffinity, SetWindowLongW,
        WDA_EXCLUDEFROMCAPTURE, WDA_NONE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
        WS_EX_TRANSPARENT, SetWindowPos, GetWindowRect, ShowWindow, FlashWindow,
        SetLayeredWindowAttributes, HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOMOVE, 
        SWP_NOSIZE, SW_MINIMIZE, SW_MAXIMIZE, SW_RESTORE, WS_EX_LAYERED, LWA_ALPHA,
        ShowCursor,
    },
};

#[unsafe(no_mangle)]
pub extern "system" fn SetWindowVisibility(hwnd: HWND, hide: bool) -> bool {
    let dwaffinity = if hide {
        WDA_EXCLUDEFROMCAPTURE
    } else {
        WDA_NONE
    };
    let result = unsafe { SetWindowDisplayAffinity(hwnd, dwaffinity) };
    result.is_ok()
}

#[unsafe(no_mangle)]
pub extern "system" fn HideFromTaskbar(hwnd: HWND, hide: bool) -> bool {
    let mut style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) };
    if style == 0 {
        return false;
    }
    if hide {
        style |= WS_EX_TOOLWINDOW.0 as i32;
        style &= (!WS_EX_APPWINDOW.0) as i32;
    } else {
        style |= WS_EX_APPWINDOW.0 as i32;
        style &= (!WS_EX_TOOLWINDOW.0) as i32;
    }
    unsafe { SetWindowLongW(hwnd, GWL_EXSTYLE, style) };
    true
}

#[unsafe(no_mangle)]
pub extern "system" fn IgnoreMouse(hwnd: HWND, ignore: bool) -> bool {
    let mut style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) };
    if style == 0 {
        return false;
    }
    if ignore {
        style |= WS_EX_TRANSPARENT.0 as i32;
    } else {
        style &= !(WS_EX_TRANSPARENT.0 as i32);
    }
    unsafe { SetWindowLongW(hwnd, GWL_EXSTYLE, style) };
    true
}

#[unsafe(no_mangle)]
pub extern "system" fn SetWindowOpacity(hwnd: HWND, opacity: u8) -> bool {
    let mut style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) };
    if style == 0 {
        return false;
    }
    style |= WS_EX_LAYERED.0 as i32;
    unsafe { SetWindowLongW(hwnd, GWL_EXSTYLE, style) };
    unsafe { SetLayeredWindowAttributes(hwnd, COLORREF(0), opacity, LWA_ALPHA) }.is_ok()
}

#[unsafe(no_mangle)]
pub extern "system" fn SetWindowAlwaysOnTop(hwnd: HWND, always_on_top: bool) -> bool {
    let hwnd_insert = if always_on_top { Some(HWND_TOPMOST) } else { Some(HWND_NOTOPMOST) };
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

#[unsafe(no_mangle)]
pub extern "system" fn GetWindowRectangle(hwnd: HWND, rect: *mut RECT) -> bool {
    if rect.is_null() {
        return false;
    }
    unsafe { GetWindowRect(hwnd, rect) }.is_ok()
}

#[unsafe(no_mangle)]
pub extern "system" fn FlashWindowInTaskbar(hwnd: HWND) -> bool {
    unsafe { FlashWindow(hwnd, true) }.as_bool()
}

#[unsafe(no_mangle)]
pub extern "system" fn HideCursor(hide: bool) -> bool {
    unsafe { ShowCursor(!hide) };
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_mouse_function_signature() {
        let hwnd = HWND(std::ptr::null_mut());
        let result = IgnoreMouse(hwnd, true);
        assert!(!result);
    }

    #[test]
    fn test_hide_from_taskbar_function_signature() {
        let hwnd = HWND(std::ptr::null_mut());
        let result = HideFromTaskbar(hwnd, true);
        assert!(!result);
    }

    #[test]
    fn test_set_window_visibility_function_signature() {
        let hwnd = HWND(std::ptr::null_mut());
        let result = SetWindowVisibility(hwnd, true);
        assert!(!result);
    }
}
