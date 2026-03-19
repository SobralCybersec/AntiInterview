use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        GWL_EXSTYLE, GetWindowLongW, SetWindowDisplayAffinity, SetWindowLongW,
        WDA_EXCLUDEFROMCAPTURE, WDA_NONE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
        WS_EX_TRANSPARENT,
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
