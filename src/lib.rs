use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows::Win32::UI::WindowsAndMessaging::{
    GUITHREADINFO, GetGUIThreadInfo, SendMessageW, WM_IME_CONTROL,
};
use windows::core::Result;

const IMC_SETOPENSTATUS: usize = 0x06;
const TRUE: isize = 1;
const FALSE: isize = 0;

pub fn get_focus_window() -> Result<HWND> {
    let mut gui_info = GUITHREADINFO::default();
    gui_info.cbSize = std::mem::size_of::<GUITHREADINFO>() as u32;

    unsafe {
        GetGUIThreadInfo(0, &mut gui_info)?;
        Ok(gui_info.hwndFocus)
    }
}

pub fn get_default_ime_wnd(focus_hwnd: HWND) -> HWND {
    unsafe { ImmGetDefaultIMEWnd(focus_hwnd) }
}

pub fn set_ime_open_status(hwnd: HWND, open: bool) {
    unsafe {
        SendMessageW(
            hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS),
            LPARAM(if open { TRUE } else { FALSE }),
        );
    }
}

pub fn parse_args(args: &[String]) -> bool {
    if args.len() < 2 {
        return false;
    }

    args[1].parse::<i32>().unwrap_or(0) != 0
}
