use {
    crate::{
        bind_callback,
        event_handler::keys::Binding,
        switch_song_callback::{
            self,
            SwitchSongCallback,
        },
        status_bar,
    },
    crossterm::{
        event::{
            KeyEvent,
            KeyCode,
            KeyModifiers,
        },
        style,
    },
    std::{
        boxed::Box,
        time::Duration,
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

pub const SWITCH_SONG_CALLBACKS: [SwitchSongCallback; 3] = [
    switch_song_callback::callback_random,
    switch_song_callback::callback_next,
    switch_song_callback::callback_loop,
];

pub fn init_key_bindings() -> Vec<Binding> {
    return vec![
        // quit
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('q'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::Quit {}),
        ),
        // cursor movement
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('h'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::X,
                step: -1,
            }),
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('j'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::Y,
                step: 1,
            }),
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('k'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::Y,
                step: -1,
            }),
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('l'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::X,
                step: 1,
            }),
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('G'), KeyModifiers::SHIFT ),
            ],
            Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::BOTTOM,
                step: 0,
            }),
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('g'), KeyModifiers::NONE ),
                KeyEvent::new( KeyCode::Char('g'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::TOP,
                step: 0,
            }),
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('r'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::MoveCursor {
                direction: bind_callback::CursorDirection::SELECTED,
                step: 0,
            }),
        ),
        // interaction
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Enter, KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::Select {}),
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('s'), KeyModifiers::NONE ),
            ],
            Box::new(bind_callback::SwitchSong {}),
        ),
    ];
}

pub fn init_status_bar() -> Vec<status_bar::ModuleHandler> {
    return vec![
        status_bar::ModuleHandler::new(Duration::from_secs(1), Box::new(
            status_bar::PlayDuration::new(
                |duration: Duration| {
                    return String::from("asdf")
                }
            ),
        )),
    ];
}
