mod config;
mod filesystem;
mod macros;
mod menu;
mod wrappers;

use menu::Menu;

use std::{
    boxed::Box,
    io::{
        self,
        Result,
    },
    panic,
    process,
};

use crossterm::{
    cursor,
    terminal,
    ExecutableCommand,
};

#[cfg(unix)]
use std::thread;

#[cfg(unix)]
use signal_hook::{
    consts::{
        SIGINT,
        SIGTERM,
    },
    iterator::Signals,
};

fn get_playlist_song_path(playlist_names: &Vec<String>, playlists: &Vec<Vec<String>>, playlist: usize, song: usize) -> String {
    return format!("{}/{}/{}", config::PLAYLISTS_DIRECTORY, playlist_names[playlist], playlists[playlist][song]);
}
fn get_playlist_path(playlist_name: &str) -> String {
    return format!("{}/{}", config::PLAYLISTS_DIRECTORY, playlist_name);
}

fn resize_menus(menu: &mut Menu, sub_menus: &mut Vec<Menu>) -> Result<()> {
    let window_size: terminal::WindowSize = terminal::window_size()?;

    resize_main_menu(menu, &window_size);
    for sub_menu in sub_menus {
        resize_sub_menu(sub_menu, &window_size);
    }

    return Result::Ok(());
}
fn resize_main_menu(menu: &mut Menu, window_size: &terminal::WindowSize) {
    menu.x = 0;
    menu.y = 0;
    menu.width = cast!(window_size.width / 2);
    menu.height = cast!(window_size.height);
}
fn resize_sub_menu(menu: &mut Menu, window_size: &terminal::WindowSize) {
    menu.x = cast!(window_size.width / 2 + 1);
    menu.y = 0;
    menu.width = cast!(window_size.width / 2);
    menu.height = cast!(window_size.height);
}

fn uninit() -> Result<()> {
    if terminal::is_raw_mode_enabled()? {
        terminal::disable_raw_mode()?;
    }
    io::stdout()
        .execute(terminal::LeaveAlternateScreen)?
        .execute(cursor::Show)?;

    return Ok(());
}

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        let _ = uninit();
        println!("{}", panic_info);
        process::exit(-1);
    }));

    #[cfg(unix)]
    let mut signals: Signals = Signals::new([SIGINT, SIGTERM]).unwrap();
    #[cfg(unix)]
    thread::spawn(move || {
        for signal in &mut signals {
            let _ = uninit();
            panic!("Caught signal: {:?}", signal);
        }
    });

    terminal::enable_raw_mode().unwrap();
    io::stdout()
        .execute(terminal::EnterAlternateScreen).unwrap()
        .execute(cursor::Hide).unwrap();

    let mut playlist_names: Vec<String> = filesystem::get_entries(config::PLAYLISTS_DIRECTORY, filesystem::EntryType::DIRECTORY).unwrap();
    if playlist_names.len() == 0 {
        panic!("No playlists were found");
    }

    let mut playlists: Vec<Vec<String>> = Vec::new();

    for i in 0..playlist_names.len() {
        playlists.push(filesystem::get_entries(&get_playlist_path(&playlist_names[i]), filesystem::EntryType::FILE).unwrap());
    }

    for i in 0..playlists.len() {
        if playlists[i].len() == 0 {
            playlist_names.remove(i);
            playlists.remove(i);
        }
    }

    let mut main_menu: Menu = Menu::new(config::FOREGROUND, config::FOREGROUND_REVERSED,
        config::BACKGROUND, config::BACKGROUND_REVERSED,

        playlist_names.clone());

    let mut sub_menus: Vec<Menu> = Vec::new();
    for playlist in playlists {
        sub_menus.push(Menu::new(config::FOREGROUND, config::FOREGROUND_REVERSED,
                config::BACKGROUND, config::BACKGROUND_REVERSED,

                playlist.clone()));
    }
    resize_menus(&mut main_menu, &mut sub_menus).unwrap();

    thread::sleep(std::time::Duration::new(1, 0));

    uninit().unwrap();
}
