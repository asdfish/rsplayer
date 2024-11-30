mod audio_handler;
mod bind_callback;
mod config;
mod event_handler;
mod filesystem;
mod macros;
mod menu;
mod rs_player;
mod switch_song_callback;
mod wrappers;

use {
    event_handler::EventHandler,
    rs_player::RsPlayer,
};

fn main() {
    let mut rs_player: RsPlayer = RsPlayer::new().unwrap();
    let mut event_handler: EventHandler = EventHandler::new();

    rs_player.switch_song_to(0);
    while rs_player.running {
        let _ = rs_player.draw();
        let _ = event_handler.update(&mut rs_player);

        if !rs_player.audio_handler.is_playing() {
            rs_player.switch_song();
        }
    }

    RsPlayer::uninit();
}
