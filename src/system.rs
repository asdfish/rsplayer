use {
    std::{
        io::{
            stdout,
            Result,
        },
        panic,
        process,
    },
    crossterm::{
        cursor,
        execute,
        style,
        terminal,
    },
};

#[cfg(windows)]
use {
    winapi::{
        shared::minwindef::{
            BOOL,
            DWORD,
            FALSE,
            TRUE,
        },
        um::consoleapi::SetConsoleCtrlHandler,
    },
};

#[cfg(unix)]
use {
    std::thread,
    signal_hook::{
        consts::SIGINT,
        consts::SIGTERM,
        iterator::Signals,
    },
};

pub fn init() -> Result<()> {
    init_hooks();
    init_terminal()?;
    Result::Ok(())
}
fn init_hooks() {
    panic::set_hook(Box::new(|panic_info| {
        uninit();
        println!("{}", panic_info);
        process::exit(-1);
    }));

    #[cfg(windows)]
    {
        extern "system" fn ctrl_handler(ctrl_type: DWORD) -> BOOL {
            println!("Caught signal: {}", ctrl_type);
            uninit();

            FALSE
        }

        if 0 == unsafe { SetConsoleCtrlHandler(Some(ctrl_handler), TRUE) } {
            println!("Failed to set ConsoleCtrlHandler");
            uninit();
        }
    }

    #[cfg(unix)]
    {
        let mut signals: Signals = Signals::new([SIGINT, SIGTERM]).unwrap();

        thread::spawn(move || {
            if let Some(signal) = (&mut signals).into_iter().next() {
                uninit();
                panic!("Caught signal: {}", signal);
            }
        });
    }
}
fn init_terminal() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide)?;

    Result::Ok(())
}

pub fn uninit() {
    let is_raw_mode_enabled: Result<bool> = terminal::is_raw_mode_enabled();
    if is_raw_mode_enabled.is_ok() && is_raw_mode_enabled.unwrap() {
        let _ = terminal::disable_raw_mode();
    }

    let _result = execute!(stdout(),
        style::ResetColor,
        terminal::LeaveAlternateScreen,
        cursor::Show);
}
