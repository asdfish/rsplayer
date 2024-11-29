use {
    crate::{
        event_handler,
        rs_player::RsPlayer,
    },
    crossterm::{
        event,
        style,
    },
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

fn quit(rs_player: &mut RsPlayer) -> bool {
    rs_player.running = false;
    return false;
}

pub fn init_key_bindings() -> Vec<event_handler::keys::Binding> {
    return vec![
        event_handler::keys::Binding {
            key_events: vec![
                event::KeyEvent {
                    code: event::KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE,
                },
            ],
            callback: quit
        }
    ];
}
