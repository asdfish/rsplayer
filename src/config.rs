use {
    crate::{
        event_handler::keys::Binding,
        menu::Menu,
        menu_handler::{
            MenuHandler,
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
    std::time::Duration,
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
    |menu_handler: &mut MenuHandler| {
        let next_song: usize = fastrand::usize(0..menu_handler.get_current_playlist().len());

        menu_handler.switch_song_to(next_song);
    },
    |menu_handler: &mut MenuHandler| {
        let next_song: usize = if menu_handler.sub_menu.selected + 1 > menu_handler.get_current_playlist().len() {
            0
        } else {
            menu_handler.sub_menu.selected + 1
        };

        menu_handler.switch_song_to(next_song);
    },
    |menu_handler: &mut MenuHandler| {
        menu_handler.switch_song_to(menu_handler.sub_menu.selected);
    },
];

pub fn init_key_bindings() -> Vec<Binding> {
    return vec![
        // quit
        Binding::new(
            vec![
              KeyEvent::new( KeyCode::Char('q'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                menu_handler.running = false;
            },
        ),
        // cursor movement
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('h'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                move_cursor(CursorDirection::X, -1, menu_handler);
            },
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('j'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                move_cursor(CursorDirection::Y, 1, menu_handler);
            },
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('k'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                move_cursor(CursorDirection::Y, -1, menu_handler);
            },
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('l'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                move_cursor(CursorDirection::X, 1, menu_handler);
            },
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('G'), KeyModifiers::SHIFT ),
            ],
            |menu_handler: &mut MenuHandler| {
                move_cursor(CursorDirection::BOTTOM, 0, menu_handler);
            },
        ),
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('g'), KeyModifiers::NONE ),
                KeyEvent::new( KeyCode::Char('g'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                move_cursor(CursorDirection::TOP, 0, menu_handler);
            },
        ),
        // return to selected item
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('r'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                move_cursor(CursorDirection::SELECTED, 0, menu_handler);
            },
        ),
        // switch song
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Enter, KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                menu_handler.switch_song();
            }
        ),
        // select menu
        Binding::new(
            vec![
                KeyEvent::new( KeyCode::Char('s'), KeyModifiers::NONE ),
            ],
            |menu_handler: &mut MenuHandler| {
                match menu_handler.selected_menu {
                    0 => {
                        menu_handler.change_sub_menu(menu_handler.main_menu.cursor);
                    },
                    1 => {
                        menu_handler.switch_song_to(menu_handler.sub_menu.cursor);
                    },
                    _ => unreachable!(),
                }

                menu_handler.redraw = true;
            }
        ),
    ];
}

pub fn init_status_bar() -> Vec<status_bar::ModuleHandler> {
    return vec![
        status_bar::ModuleHandler::new(Color::White, Color::Black, Duration::from_secs(1), Box::new(
            status_bar::PlayDuration::new(
                |duration: Duration, menu_handler: &MenuHandler| {
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
                            0.0
                        }
                    } else {
                        0.0
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

// local
pub enum CursorDirection {
    X, Y,
    TOP, BOTTOM,
    SELECTED,
}
fn move_cursor(cursor_direction: CursorDirection, step: isize, menu_handler: &mut MenuHandler) {
    let menus: [&mut Menu; 2] = [&mut menu_handler.main_menu, &mut menu_handler.sub_menu];

    match cursor_direction {
        CursorDirection::X => {
            if step > 0 {
                menu_handler.selected_menu = 1;
            } else if step < 0 {
                menu_handler.selected_menu = 0;
            }
        },
        CursorDirection::Y => menus[menu_handler.selected_menu].move_cursor(step),

        CursorDirection::TOP => menus[menu_handler.selected_menu].cursor = 0,
        CursorDirection::BOTTOM => menus[menu_handler.selected_menu].cursor = usize::MAX,

        CursorDirection::SELECTED => menus[menu_handler.selected_menu].cursor = menus[menu_handler.selected_menu].selected,
    }

    menu_handler.redraw = true;
}
