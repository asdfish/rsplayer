mod audio_handler;
mod config;
mod event_handler;
mod filesystem;
mod macros;
mod menu;
mod menu_handler;
mod status_bar;
mod system;
mod wrappers;

use {
    event_handler::EventHandler,
    menu_handler::MenuHandler,
    status_bar::StatusBar,
    std::io::{
        stdout,
        Write,
    },
    system::{
        init,
        uninit,
    },
};

fn main() {
    init().unwrap();

    let mut menu_handler: MenuHandler = MenuHandler::new().unwrap();
    let mut status_bar: StatusBar = StatusBar::new();
    let mut event_handler: EventHandler = EventHandler::new();

    event_handler.resize(&mut menu_handler).unwrap();
    menu_handler.switch_song_to(0);
    while menu_handler.running {
        let _ = status_bar.draw(&event_handler);
        let _ = menu_handler.draw();

        stdout()
            .flush().unwrap();

        let _ = status_bar.update(&mut menu_handler);
        let _ = event_handler.update(&mut menu_handler, &mut status_bar);

        if !menu_handler.audio_handler.is_playing() {
            menu_handler.switch_song();
        }
    }

    uninit();
}
