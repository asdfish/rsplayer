mod config;
mod filesystem;

use std::io::Result;

fn get_playlist_song_path(playlist_names: &Vec<String>, playlists: &Vec<Vec<String>>, playlist: usize, song: usize) -> String {
    return format!("{}/{}/{}", config::PLAYLISTS_DIRECTORY, playlist_names[playlist], playlists[playlist][song]);
}
fn get_playlist_path(playlist_name: &str) -> String {
    return format!("{}/{}", config::PLAYLISTS_DIRECTORY, playlist_name);
}

fn main() -> Result<()>  {
    let mut playlist_names: Vec<String> = filesystem::get_entries(config::PLAYLISTS_DIRECTORY, filesystem::EntryType::DIRECTORY)?;
    let mut playlists: Vec<Vec<String>> = Vec::new();

    for i in 0..playlist_names.len() {
        playlists.push(
            filesystem::get_entries(&get_playlist_path(&playlist_names[i]), filesystem::EntryType::FILE)?
        );
    }

    for i in 0..playlists.len() {
        if playlists[i].len() == 0 {
            playlist_names.remove(i);
            playlists.remove(i);
        }
    }

    for playlist in 0..playlists.len() {
        for song in 0..playlists[playlist].len() {
            println!("{}", get_playlist_song_path(&playlist_names, &playlists, playlist, song));
        }
    }

    return Result::Ok(());
}
