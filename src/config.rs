use crate::event_handler;
use crossterm::{
    event,
    style,
};

pub const PLAYLISTS_DIRECTORY: &str = "/home/andre/files/music";

pub const NORMAL_FOREGROUND: style::Color = style::Color::White;
pub const NORMAL_FOREGROUND_REVERSED: style::Color = style::Color::Black;
pub const NORMAL_BACKGROUND: style::Color = style::Color::Black;
pub const NORMAL_BACKGROUND_REVERSED: style::Color = style::Color::White;
pub const SELECTED_FOREGROUND: style::Color = style::Color::Red;
pub const SELECTED_FOREGROUND_REVERSED: style::Color = style::Color::Red;
pub const SELECTED_BACKGROUND: style::Color = style::Color::Black;
pub const SELECTED_BACKGROUND_REVERSED: style::Color = style::Color::White;

fn cb() {
    println!("hello from cb");
}

pub fn init_key_bindings() -> Vec<event_handler::keys::Event> {
    return vec![
        event_handler::keys::Event {
            keys: vec![
                event::KeyEvent {
                    code: event::KeyCode::Char('d'),
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE,
                },
            ],
            callback: cb
        }
    ];
}

//static key_bindings: Vec<event_handler::keys::Event> = vec![
//    event_handler::keys::Event {
//        keys: vec![
//            event::KeyEvent {
//                code: event::KeyCode::Char('d'),
//                modifiers: event::KeyModifiers::NONE,
//                kind: event::KeyEventKind::Press,
//                state: event::KeyEventState::NONE,
//            },
//        ],
//        callback: cb,
//    }
//];
