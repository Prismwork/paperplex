use tao::window::Window;

#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[cfg(target_os = "windows")]
#[allow(dead_code)]
unsafe extern "system" fn enum_windows_proc(top_handle: HWND, _: LPARAM) -> BOOL {
    let mut class_name = [0u16; 256];
    if GetClassNameW(top_handle, &mut class_name) != 0 {
        let s1 = String::from_utf16_lossy(&class_name);

        if s1.contains("WorkerW") {
            let def = FindWindowExW(top_handle, None, w!("SHELLDLL_DefView"), None);
            if def.0 == 0 {
                let _ = ShowWindow(top_handle, SW_HIDE);
            }
        }
    }
    true.into()
}

#[cfg(target_os = "windows")]
pub fn setup_window_win(window: &Window) {
    let hwnd = HWND(window.hwnd() as isize);

    unsafe {
        let progman_hwnd = FindWindowW(w!("Progman"), w!("Program Manager"));

        SendMessageTimeoutW(
            progman_hwnd,
            0x052c,
            WPARAM(0),
            LPARAM(0),
            SMTO_NORMAL,
            1000,
            None,
        );

        EnumWindows(Some(enum_windows_proc), LPARAM(0)).unwrap();

        SetParent(hwnd, progman_hwnd);

        let ex_style = WS_EX_LAYERED
                // | WS_EX_TRANSPARENT
                | WS_EX_NOACTIVATE
                | WS_EX_TOOLWINDOW;
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style.0 as isize);

        SetWindowPos(
            hwnd,
            HWND_BOTTOM,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        ).unwrap();
    }
}
