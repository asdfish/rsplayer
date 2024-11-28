mod config;
mod filesystem;

use std::{
    boxed::Box,
    io::{
        self,
        Result,
    },
    panic,
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
    consts::SIGINT,
    iterator::Signals,
};

fn get_playlist_song_path(playlist_names: &Vec<String>, playlists: &Vec<Vec<String>>, playlist: usize, song: usize) -> String {
    return format!("{}/{}/{}", config::PLAYLISTS_DIRECTORY, playlist_names[playlist], playlists[playlist][song]);
}
fn get_playlist_path(playlist_name: &str) -> String {
    return format!("{}/{}", config::PLAYLISTS_DIRECTORY, playlist_name);
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

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    io::stdout()
        .execute(terminal::EnterAlternateScreen)?
        .execute(cursor::Hide)?;

    #[cfg(unix)]
    let mut signals: Signals = Signals::new([SIGINT])?;
    #[cfg(unix)]
    thread::spawn(move || {
        for signal in &mut signals {
            let _ = uninit();
            panic!("Caught signal: {:?}", signal);
        }
    });
    panic::set_hook(Box::new(|panic_info| {
        let _ = uninit();
        println!("{}", panic_info);
    }));

    let mut playlist_names: Vec<String> = filesystem::get_entries(config::PLAYLISTS_DIRECTORY, filesystem::EntryType::DIRECTORY).unwrap();
    let mut playlists: Vec<Vec<String>> = Vec::new();

    for i in 0..playlist_names.len() {
        playlists.push(
            filesystem::get_entries(&get_playlist_path(&playlist_names[i]), filesystem::EntryType::FILE).unwrap()
        );
    }

    for i in 0..playlists.len() {
        if playlists[i].len() == 0 {
            playlist_names.remove(i);
            playlists.remove(i);
        }
    }

    uninit()?;
    return Ok(());
}
