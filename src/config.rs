use {
    crate::{
        bind_callback,
        event_handler,
    },
    crossterm::{
        event,
        style,
    },
    std::boxed::Box,
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

pub const FRAME_RATE_MS: u64 = 1000 / 24;

pub fn init_key_bindings() -> Vec<event_handler::keys::Binding> {
    return vec![
        event_handler::keys::Binding {
            key_events: vec![
                event::KeyEvent::new(
                    event::KeyCode::Char('q'),
                    event::KeyModifiers::NONE
                ),
            ],
            callback: Box::new(bind_callback::Quit {}),
        },
        event_handler::keys::Binding {
            key_events: vec![
                event::KeyEvent::new(
                    event::KeyCode::Char('h'),
                    event::KeyModifiers::NONE
                ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::X,
                step: -1,
            }),
        },
        event_handler::keys::Binding {
            key_events: vec![
                event::KeyEvent::new(
                    event::KeyCode::Char('j'),
                    event::KeyModifiers::NONE
                ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::Y,
                step: 1,
            }),
        },
        event_handler::keys::Binding {
            key_events: vec![
                event::KeyEvent::new(
                    event::KeyCode::Char('k'),
                    event::KeyModifiers::NONE
                ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::Y,
                step: -1,
            }),
        },
        event_handler::keys::Binding {
            key_events: vec![
                event::KeyEvent::new(
                    event::KeyCode::Char('l'),
                    event::KeyModifiers::NONE
                ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::X,
                step: 1,
            }),
        },
    ];
}
