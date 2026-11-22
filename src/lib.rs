use windows::Win32::Foundation::{E_FAIL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::{IMC_SETOPENSTATUS, ImmGetDefaultIMEWnd};
use windows::Win32::UI::WindowsAndMessaging::{
    GUITHREADINFO, GetGUIThreadInfo, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_IME_CONTROL,
};
use windows::core::Error;

const TIMEOUT_MS: u32 = 500;

pub fn get_focus_window() -> windows::core::Result<HWND> {
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

pub fn get_default_ime_wnd(focus_hwnd: HWND) -> windows::core::Result<HWND> {
    let hwnd = unsafe { ImmGetDefaultIMEWnd(focus_hwnd) };
    if hwnd.is_invalid() {
        return Err(Error::new(
            E_FAIL,
            "Failed to obtain default IME window handle.",
        ));
    }
    Ok(hwnd)
}

/// IME の ON/OFF 状態を設定するメッセージを送信する。
///
/// 相手のウィンドウが応答しない場合に固まらないよう、タイムアウト付きで送信する。
/// 次の場合は `Err` を返す。
/// - タイムアウトした、または相手が無応答だった
/// - 相手が管理者権限で動作しており、権限の低いこのプロセスからの送信が
///   UIPI (User Interface Privilege Isolation) により拒否された
pub fn set_ime_open_status(hwnd: HWND, open: bool) -> windows::core::Result<()> {
    let result = unsafe {
        SendMessageTimeoutW(
            hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as usize),
            LPARAM(open as isize),
            SMTO_ABORTIFHUNG,
            TIMEOUT_MS,
            None,
        )
    };
    if result.0 == 0 {
        return Err(Error::new(
            E_FAIL,
            "Failed to send IME message (timeout or access denied).",
        ));
    }
    Ok(())
}

pub fn parse_args(args: &[String]) -> Result<bool, String> {
    match args {
        [] | [_] => Ok(false),
        [_, value] => value
            .parse::<i64>()
            .map(|n| n != 0)
            .map_err(|_| format!("invalid argument: {value}")),
        _ => Err("too many arguments".to_string()),
    }
}
