use {
    crate::{
        cast,
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
        style::{
            self,
            Color,
        },
    },
    std::{
        boxed::Box,
        time::Duration,
    },
};

pub const PLAYLISTS_DIRECTORY: &str = "/home/andre/files/music";

pub const NORMAL_FOREGROUND: Color = Color::White;
pub const NORMAL_FOREGROUND_REVERSED: Color = Color::Black;
pub const NORMAL_BACKGROUND: Color = Color::Black;
pub const NORMAL_BACKGROUND_REVERSED: Color = Color::White;
pub const SELECTED_FOREGROUND: Color = Color::Red;
pub const SELECTED_FOREGROUND_REVERSED: Color = Color::Red;
pub const SELECTED_BACKGROUND: Color = Color::Black;
pub const SELECTED_BACKGROUND_REVERSED: Color = Color::White;

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
        status_bar::ModuleHandler::new(Color::White, Color::Black, Duration::from_secs(1), Box::new(
            status_bar::PlayDuration::new(
                move |duration: Duration| {
                    fn pad_usize(num: usize) -> String {
                        return if num < 10 {
                            "0".to_string() + &num.to_string()
                        } else {
                            num.to_string()
                        }
                    }

                    let seconds: usize = cast!(duration.as_secs());
                    let minutes: usize = if seconds == 0 { 0 } else { seconds / 60 };

                    let seconds: String = pad_usize(seconds);
                    let minutes: String = pad_usize(minutes);

                    return minutes + ":" + &seconds.to_string();
                }
            ),
        )),
    ];
}
