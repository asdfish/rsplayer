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
    pub playlist_names: Vec<String>,
    pub playlists: Vec<Vec<String>>,

    pub main_menu: Menu,
    pub sub_menu: Menu,
    pub selected_menu: usize,

    pub sub_menu_selections: Vec<usize>,

    pub audio_handler: AudioHandler,

    pub redraw: bool,
    pub running: bool,

    pub switch_song_callback: usize,
}

impl RsPlayer {
    pub fn new() -> Result<RsPlayer> {
        Self::init()?;

        let playlist_names: Vec<String> = filesystem::get_entries(config::PLAYLISTS_DIRECTORY, filesystem::EntryType::DIRECTORY)?;
        let mut playlists: Vec<Vec<String>> = Vec::new();

        let playlist_names_length: usize = playlist_names.len();

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
            
            sub_menu_selections: vec![0; playlist_names_length],

            audio_handler: AudioHandler::new(),

            redraw: true,
            running: true,

            switch_song_callback: 0,
        };
        rs_player.sub_menu.reverse_colors = false;

        EventHandler::resize(&mut rs_player)?;

        return Result::Ok(rs_player);
    }

    pub fn change_sub_menu(&mut self, new_sub_menu: usize) {
        // set old selection to sub_menu_selections to save
        self.sub_menu_selections[self.main_menu.selected] = self.sub_menu.selected;
        // set sub_menu.selection to saved value
        self.sub_menu.selected = self.sub_menu_selections[new_sub_menu];

        self.main_menu.selected = new_sub_menu;
    }

    pub fn draw(&mut self) -> Result<()> {
        if !self.redraw {
            return Result::Ok(());
        }

        match self.selected_menu {
            0 => {
                self.main_menu.reverse_colors = true;
                self.sub_menu.reverse_colors = false;
            },
            1 => {
                self.main_menu.reverse_colors = false;
                self.sub_menu.reverse_colors = true;
            },
            _ => unreachable!(),
        };

        self.main_menu.draw(&self.playlist_names)?;
        self.sub_menu.draw(&self.playlists[self.main_menu.selected])?;

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

    pub fn switch_song(&mut self) {
        config::SWITCH_SONG_CALLBACKS[self.switch_song_callback](self);
        self.redraw = true;
    }
    pub fn switch_song_to(&mut self, song: usize) {
        self.sub_menu.selected = if song > self.playlists[self.main_menu.selected].len() {
            self.playlists[self.main_menu.selected].len()
        } else {
            song
        };

        self.audio_handler.play(RsPlayer::get_playlist_song_path(
                &self.playlist_names[self.main_menu.selected],
                &self.playlists[self.main_menu.selected][self.sub_menu.selected]));
    }

    pub fn get_playlist_path(playlist_name: &str) -> String {
        return format!("{}/{}", config::PLAYLISTS_DIRECTORY, playlist_name);
    }
    pub fn get_playlist_song_path(playlist_name: &str, song_name: &str) -> String {
        return format!("{}/{}", Self::get_playlist_path(playlist_name), song_name);
    }

    pub fn get_current_playlist(&self) -> &Vec<String> {
        return &self.playlists[self.main_menu.selected];
    }
}
