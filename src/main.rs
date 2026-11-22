use std::process::ExitCode;

fn run(open: bool) -> windows::core::Result<()> {
    let focus = imectl::get_focus_window()?;
    let ime = imectl::get_default_ime_wnd(focus)?;
    imectl::set_ime_open_status(ime, open)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let open = match imectl::parse_args(&args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("imectl: {e}\nusage: imectl [0|1]");
            return ExitCode::from(2);
        }
    };
    match run(open) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("imectl: {e}");
            ExitCode::FAILURE
        }
    }
}
