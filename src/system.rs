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
    return Result::Ok(());
}
fn init_hooks() {
    panic::set_hook(Box::new(|panic_info| {
        let _ = uninit();
        println!("{}", panic_info);
        process::exit(-1);
    }));

    #[cfg(windows)]
    {
        extern "system" fn ctrl_handler(ctrl_type: DWORD) -> BOOL {
            println!("Caught signal: {}", ctrl_type);
            uninit();

            return FALSE;
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
            for signal in &mut signals {
                let _ = uninit();
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

    return Result::Ok(());
}

pub fn uninit() {
    match terminal::is_raw_mode_enabled() {
        Ok(is_raw_mode_enabled) => {
            if is_raw_mode_enabled {
                let _result = terminal::disable_raw_mode();
            }
        },
        _ => {}
    }

    let _result = execute!(stdout(),
        style::ResetColor,
        terminal::LeaveAlternateScreen,
        cursor::Show);
}
