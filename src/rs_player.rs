use {
    crate::{
        audio_handler::AudioHandler,
        config,
        event_handler::EventHandler,
        filesystem,
        menu::Menu,
    },
    crossterm::{
        cursor,
        execute,
        terminal,
    },
    std::{
        boxed::Box,
        io::{
            stdout,
            Write,
            Result,
        },
        panic,
        process,
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

pub struct RsPlayer {
    playlist_names: Vec<String>,
    playlists: Vec<Vec<String>>,

    pub main_menu: Menu,
    pub sub_menu: Menu,
    pub selected_menu: u8,

    audio_handler: AudioHandler,

    pub redraw: bool,
    pub running: bool,
}

impl RsPlayer {
    pub fn new() -> Result<RsPlayer> {
        Self::init()?;

        let playlist_names: Vec<String> = filesystem::get_entries(config::PLAYLISTS_DIRECTORY, filesystem::EntryType::DIRECTORY)?;
        let mut playlists: Vec<Vec<String>> = Vec::new();

        for playlist_name in &playlist_names {
            playlists.push(filesystem::get_entries(&Self::get_playlist_path(&playlist_name), filesystem::EntryType::FILE)?);
        }

        for i in 0..playlists.len() {
            if playlists[i].len() == 0 {
                playlists.remove(i);
            }
        }
        if playlists.len() == 0 {
            panic!("No playlists were found");
        }

        let mut rs_player: RsPlayer = RsPlayer {
            playlist_names: playlist_names,
            playlists: playlists,

            main_menu: Menu::new(),
            sub_menu: Menu::new(),
            selected_menu: 0,

            audio_handler: AudioHandler::new(),

            redraw: true,
            running: true,
        };

        EventHandler::resize(&mut rs_player)?;

        return Result::Ok(rs_player);
    }

    pub fn draw(&mut self) -> Result<()> {
        if !self.redraw {
            return Result::Ok(());
        }

        self.main_menu.draw(&self.playlist_names)?;
        self.sub_menu.draw(&self.playlists[self.main_menu.selected])?;

        match self.selected_menu {
            0 => {
                self.main_menu.reverse_colors = true;
                self.sub_menu.reverse_colors = false;
            },
            1 => {
                self.main_menu.reverse_colors = false;
                self.sub_menu.reverse_colors = true;
            },
            _ => {},
        };

        stdout()
            .flush()?;

        self.redraw = false;
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
            terminal::LeaveAlternateScreen,
            cursor::Show);
    }

    fn init() -> Result<()> {
        Self::init_hooks();
        Self::init_terminal()?;
        return Result::Ok(());
    }
    fn init_hooks() {
        panic::set_hook(Box::new(|panic_info| {
            let _ = Self::uninit();
            println!("{}", panic_info);
            process::exit(-1);
        }));

        #[cfg(unix)]
        {
            let mut signals: Signals = Signals::new([SIGINT, SIGTERM]).unwrap();

            thread::spawn(move || {
                for signal in &mut signals {
                    let _ = Self::uninit();
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

    fn get_playlist_path(playlist_name: &str) -> String {
        return format!("{}/{}", config::PLAYLISTS_DIRECTORY, playlist_name);
    }
}
