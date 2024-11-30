use {
    crate::{
        bind_callback,
        event_handler::keys::Binding,
        switch_song_callback::{
            self,
            SwitchSongCallback,
        },
    },
    crossterm::{
        event::{
            KeyEvent,
            KeyCode,
            KeyModifiers,
        },
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

pub const SWITCH_SONG_CALLBACKS: [SwitchSongCallback; 2] = [
    switch_song_callback::callback_next,
    switch_song_callback::callback_loop,
];

pub fn init_key_bindings() -> Vec<Binding> {
    return vec![
        // quit
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('q'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::Quit {}),
        },
        // cursor movement
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('h'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::X,
                step: -1,
            }),
        },
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('j'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::Y,
                step: 1,
            }),
        },
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('k'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::Y,
                step: -1,
            }),
        },
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('l'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::X,
                step: 1,
            }),
        },
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('G'), KeyModifiers::SHIFT ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::BOTTOM,
                step: 0,
            }),
        },
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('g'), KeyModifiers::NONE ),
                KeyEvent::new( KeyCode::Char('g'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::TOP,
                step: 0,
            }),
        },
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('r'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::SELECTED,
                step: 0,
            }),
        },
        // interaction
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Enter, KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::Select {}),
        },
        Binding {
            key_events: vec![
                KeyEvent::new( KeyCode::Char('s'), KeyModifiers::NONE ),
            ],
            callback: Box::new(bind_callback::SwitchSong {}),
        },
    ];
}
