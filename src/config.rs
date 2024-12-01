use {
    crate::{
        bind_callback,
        event_handler::keys::Binding,
        menu_handler::MenuHandler,
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
        style::Color,
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
                move |duration: Duration, menu_handler: &MenuHandler| {
                    const PLAY_PHASES: [&str; 10] = [
                        "[=         ]",
                        "[==        ]",
                        "[===       ]",
                        "[====      ]",
                        "[=====     ]",
                        "[======    ]",
                        "[=======   ]",
                        "[========  ]",
                        "[========= ]",
                        "[==========]",
                    ];

                    let current_duration: u64 = duration.as_secs();

                    let current_source_duration: Option<Duration> = menu_handler.audio_handler.current_source_duration.clone();
                    let play_percentage: f32 = if current_source_duration.is_some() && current_duration != 0 {
                        let current_source_duration: Duration = current_source_duration.unwrap();
                        let current_source_duration: u64 = current_source_duration.as_secs();

                        if current_source_duration != 0 {
                            (current_duration / current_source_duration) as f32
                        } else {
                            0.
                        }
                    } else {
                        0.
                    };

                    let play_percentage: usize = if PLAY_PHASES.len() == 0 { 0 } else {
                        (((PLAY_PHASES.len() - 1) as f32) * play_percentage) as usize
                    };
                    return PLAY_PHASES[play_percentage].to_string();

                    //use crate::cast;
                    //let play_percentage: f32 = if menu_handler.audio_handler.current_source_duration.is_some() {
                    //    let current_source_duration: Duration = menu_handler.audio_handler.current_source_duration.unwrap();
                    //}

                    //fn pad_usage(num: usize) -> String {
                    //    return if num < 10 {
                    //        "0".to_string() + &num.to_string()
                    //    } else {
                    //        num.to_string()
                    //    }
                    //}
                    //
                    //let seconds: usize = cast!(duration.as_secs());
                    //let minutes: usize = if seconds == 0 { 0 } else { seconds / 60 };
                    //
                    //let seconds: String = pad_usize(seconds);
                    //let minutes: String = pad_usize(minutes);
                    //
                    //return minutes + ":" + &seconds.to_string();
                }
            ),
        )),
    ];
}
