mod audio_handler;
mod bind_functions;
mod config;
mod filesystem;
mod event_handler;
mod macros;
mod menu;
mod wrappers;
mod rs_player;

use {
    event_handler::EventHandler,
    menu::Menu,
    rs_player::RsPlayer,
};

fn main() {
    let mut rs_player: RsPlayer = RsPlayer::new().unwrap();
    let mut event_handler: EventHandler = EventHandler::new();

    while rs_player.running {
        let _ = rs_player.draw();
        let _ = event_handler.update(&mut rs_player);
    }

    RsPlayer::uninit();
}
