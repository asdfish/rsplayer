use {
    crate::{
        config,
        filesystem,
        menu::Menu,
        audio_handler::AudioHandler,
    },
    std::{
        io::Result,
        panic,
    },
};

pub type SwitchSongCallback = fn(menu_handler: &mut MenuHandler);

pub struct MenuHandler {
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

impl MenuHandler {
    pub fn new() -> Result<MenuHandler> {
        let playlist_names: Vec<String> = filesystem::get_entries(config::PLAYLISTS_DIRECTORY, filesystem::EntryType::Directory)?;
        let mut playlists: Vec<Vec<String>> = Vec::new();

        let playlist_names_length: usize = playlist_names.len();

        for playlist_name in &playlist_names {
            playlists.push(filesystem::get_entries(&Self::get_playlist_path(playlist_name), filesystem::EntryType::File)?);
        }

        for i in 0..playlists.len() {
            if playlists[i].is_empty() {
                playlists.remove(i);
            }
        }
        if playlists.is_empty() {
            panic!("No playlists were found");
        }

        let mut menu_handler: MenuHandler = MenuHandler {
            playlist_names,
            playlists,

            main_menu: Menu::new(),
            sub_menu: Menu::new(),
            selected_menu: 0,
            
            sub_menu_selections: vec![0; playlist_names_length],

            audio_handler: AudioHandler::new(),

            redraw: true,
            running: true,

            switch_song_callback: 0,
        };
        menu_handler.sub_menu.reverse_colors = false;

        Result::Ok(menu_handler)
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

        self.redraw = false;
        Result::Ok(())
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

        self.audio_handler.play(MenuHandler::get_playlist_song_path(
                &self.playlist_names[self.main_menu.selected],
                &self.playlists[self.main_menu.selected][self.sub_menu.selected]));
    }

    pub fn get_playlist_path(playlist_name: &str) -> String {
        format!("{}/{}", config::PLAYLISTS_DIRECTORY, playlist_name)
    }
    pub fn get_playlist_song_path(playlist_name: &str, song_name: &str) -> String {
        format!("{}/{}", Self::get_playlist_path(playlist_name), song_name)
    }

    pub fn get_current_playlist(&self) -> &Vec<String> {
        &self.playlists[self.main_menu.selected]
    }
}
