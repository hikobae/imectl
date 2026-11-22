use windows::Win32::Foundation::{E_FAIL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows::Win32::UI::WindowsAndMessaging::{
    GUITHREADINFO, GetGUIThreadInfo, SendMessageW, WM_IME_CONTROL,
};
use windows::core::{Error, Result};

const IMC_SETOPENSTATUS: usize = 0x06;

pub fn get_focus_window() -> Result<HWND> {
    let mut gui_info = GUITHREADINFO {
        cbSize: std::mem::size_of::<GUITHREADINFO>() as u32,
        ..Default::default()
    };

    unsafe {
        GetGUIThreadInfo(0, &mut gui_info)?;
    }

    // フォーカスなし時, hwndActive にフォールバックする
    let hwnd = if gui_info.hwndFocus.is_invalid() {
        gui_info.hwndActive
    } else {
        gui_info.hwndFocus
    };
    if hwnd.is_invalid() {
        return Err(Error::new(E_FAIL, "Failed to obtain focus window handle."));
    }
    Ok(hwnd)
}

pub fn get_default_ime_wnd(focus_hwnd: HWND) -> Result<HWND> {
    let hwnd = unsafe { ImmGetDefaultIMEWnd(focus_hwnd) };
    if hwnd.is_invalid() {
        return Err(Error::new(
            E_FAIL,
            "Failed to obtain default IME window handle.",
        ));
    }
    Ok(hwnd)
}

pub fn set_ime_open_status(hwnd: HWND, open: bool) {
    unsafe {
        SendMessageW(
            hwnd,
            WM_IME_CONTROL,
            Some(WPARAM(IMC_SETOPENSTATUS)),
            Some(LPARAM(open as isize)),
        );
    }
}

pub fn parse_args(args: &[String]) -> bool {
    if args.len() < 2 {
        return false;
    }

    args[1].parse::<i32>().unwrap_or(0) != 0
}
