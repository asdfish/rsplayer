mod audio_handler;
mod config;
mod event_handler;
mod filesystem;
mod macros;
mod menu;
mod menu_handler;
mod status_bar;
mod wrappers;

use {
    event_handler::EventHandler,
    menu_handler::MenuHandler,
    status_bar::StatusBar,

    std::io::{
        stdout,
        Write,
    },
};

fn main() {
    let mut menu_handler: MenuHandler = MenuHandler::new().unwrap();
    let mut status_bar: StatusBar = StatusBar::new(&menu_handler);
    let mut event_handler: EventHandler = EventHandler::new();

    menu_handler.switch_song_to(0);
    while menu_handler.running {
        let _ = menu_handler.draw();
        let _ = status_bar.draw();

        stdout()
            .flush().unwrap();

        let _ = status_bar.update(&mut menu_handler);
        let _ = event_handler.update(&mut menu_handler);

        if !menu_handler.audio_handler.is_playing() {
            menu_handler.switch_song();
        }
    }

    MenuHandler::uninit();
}
