use windows::core::{Error as WinError, Result as WinResult};

pub fn main() -> WinResult<()> {
    let focus_hwnd = imectl::get_focus_window()?;
    if focus_hwnd.0 == 0 {
        return Err(WinError::new(
            windows::core::HRESULT(0),
            "Failed to obtain focus window handle.",
        ));
    }

    let ime_hwnd = imectl::get_default_ime_wnd(focus_hwnd);
    if ime_hwnd.0 == 0 {
        return Err(WinError::new(
            windows::core::HRESULT(0),
            "Failed to obtain IME window handle.",
        ));
    }

    let args: Vec<String> = std::env::args().collect();
    let open = imectl::parse_args(&args);
    imectl::set_ime_open_status(ime_hwnd, open);

    Ok(())
}
