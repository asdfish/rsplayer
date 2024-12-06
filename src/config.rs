use {
    crate::{
        event_handler::keys::Binding,
        menu::Menu,
        menu_handler::{MenuHandler, SwitchSongCallback},
        status_bar::{self, StatusBar},
    },
    crossterm::{
        event::{KeyCode, KeyEvent, KeyModifiers},
        style::Color,
    },
    enum_map::{Enum, EnumMap},
    std::{cmp::Ordering, time::Duration},
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

pub const STATUS_BAR_BACKGROUND: Color = Color::Black;

pub const FRAME_RATE_MS: u64 = 1000 / 12;

pub const SWITCH_SONG_CALLBACKS: [SwitchSongCallback; 3] = [
    |menu_handler: &mut MenuHandler| {
        let next_song: usize = fastrand::usize(0..menu_handler.get_current_playlist().len());

        menu_handler.switch_song_to(next_song);
    },
    |menu_handler: &mut MenuHandler| {
        let next_song: usize =
            if menu_handler.sub_menu.selected + 1 > menu_handler.get_current_playlist().len() {
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
pub const SWITCH_SONG_CALLBACK_NAMES: [&str; SWITCH_SONG_CALLBACKS.len()] =
    ["random", "next", "loop"];

#[derive(Enum)]
pub enum ModuleSignal {
    ChangedSong,
    ChangedSwitchSongCallback,
}

pub fn init_key_bindings() -> Vec<Binding> {
    enum CursorDirection {
        X,
        Y,
        Top,
        Bottom,
        Selected,
    }
    fn move_cursor(cursor_direction: CursorDirection, step: isize, menu_handler: &mut MenuHandler) {
        let menus: [&mut Menu; 2] = [&mut menu_handler.main_menu, &mut menu_handler.sub_menu];

        match cursor_direction {
            CursorDirection::X => match step.cmp(&0) {
                Ordering::Greater => menu_handler.selected_menu = 1,
                Ordering::Less => menu_handler.selected_menu = 0,
                _ => {}
            },
            CursorDirection::Y => menus[menu_handler.selected_menu].move_cursor(step),

            CursorDirection::Top => menus[menu_handler.selected_menu].cursor = 0,
            CursorDirection::Bottom => menus[menu_handler.selected_menu].cursor = usize::MAX,

            CursorDirection::Selected => {
                menus[menu_handler.selected_menu].cursor =
                    menus[menu_handler.selected_menu].selected
            }
        }

        menu_handler.redraw = true;
    }
    enum SwitchSongCallbackDirection {
        Left,
        Right,
    }
    fn switch_song(direction: SwitchSongCallbackDirection, menu_handler: &mut MenuHandler) {
        match direction {
            SwitchSongCallbackDirection::Left => {
                if menu_handler.switch_song_callback == 0 {
                    menu_handler.switch_song_callback = SWITCH_SONG_CALLBACKS.len() - 1;
                } else {
                    menu_handler.switch_song_callback -= 1;
                }
            }
            SwitchSongCallbackDirection::Right => {
                menu_handler.switch_song_callback += 1;
                if menu_handler.switch_song_callback >= SWITCH_SONG_CALLBACKS.len() {
                    menu_handler.switch_song_callback = 0;
                }
            }
        }
    }

    vec![
        // quit
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, _| {
                menu_handler.running = false;
            },
        ),
        // cursor movement
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, _| {
                move_cursor(CursorDirection::X, -1, menu_handler);
            },
        ),
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, _| {
                move_cursor(CursorDirection::Y, 1, menu_handler);
            },
        ),
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, _| {
                move_cursor(CursorDirection::Y, -1, menu_handler);
            },
        ),
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, _| {
                move_cursor(CursorDirection::X, 1, menu_handler);
            },
        ),
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT)],
            |menu_handler: &mut MenuHandler, _| {
                move_cursor(CursorDirection::Bottom, 0, menu_handler);
            },
        ),
        Binding::new(
            vec![
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
            ],
            |menu_handler: &mut MenuHandler, _| {
                move_cursor(CursorDirection::Top, 0, menu_handler);
            },
        ),
        // return to selected item
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, _| {
                move_cursor(CursorDirection::Selected, 0, menu_handler);
            },
        ),
        // switch song
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, _| {
                menu_handler.switch_song();
            },
        ),
        // select menu
        Binding::new(
            vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
            |menu_handler: &mut MenuHandler, status_bar: &mut StatusBar| {
                match menu_handler.selected_menu {
                    0 => {
                        menu_handler.change_sub_menu(menu_handler.main_menu.cursor);
                    }
                    1 => {
                        menu_handler.switch_song_to(menu_handler.sub_menu.cursor);
                    }
                    _ => unreachable!(),
                }

                status_bar.signals[ModuleSignal::ChangedSong] = true;
                menu_handler.redraw = true;
            },
        ),
        // switch switch song callback
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('H'), KeyModifiers::SHIFT)],
            |menu_handler: &mut MenuHandler, status_bar: &mut StatusBar| {
                switch_song(SwitchSongCallbackDirection::Left, menu_handler);
                status_bar.signals[ModuleSignal::ChangedSwitchSongCallback] = true;
            },
        ),
        Binding::new(
            vec![KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT)],
            |menu_handler: &mut MenuHandler, status_bar: &mut StatusBar| {
                switch_song(SwitchSongCallbackDirection::Right, menu_handler);
                status_bar.signals[ModuleSignal::ChangedSwitchSongCallback] = true;
            },
        ),
    ]
}

const COLORS: [Color; 4] = [
    Color::Rgb {
        r: 0xdd,
        g: 0x2d,
        b: 0x3a,
    },
    Color::Rgb {
        r: 0x9b,
        g: 0x13,
        b: 0x26,
    },
    Color::Rgb {
        r: 0x88,
        g: 0x0d,
        b: 0x1e,
    },
    STATUS_BAR_BACKGROUND,
];

const fn separator(color_index: usize) -> status_bar::ModuleHandler {
    status_bar::ModuleHandler::new(
        COLORS[color_index],
        COLORS[color_index + 1],
        None,
        |_| "".to_string(),
        None,
    )
}

const fn module_handler(
    color_index: usize,
    update_interval: Option<Duration>,
    update_callback: status_bar::ModuleCallback,
    signal_callback: Option<status_bar::SignalCallback>,
) -> status_bar::ModuleHandler {
    status_bar::ModuleHandler::new(
        Color::White,
        COLORS[color_index],
        update_interval,
        update_callback,
        signal_callback,
    )
}

pub type ModuleHandlersType = [status_bar::ModuleHandler; 6];
pub const STATUS_BAR_MODULE_HANDLERS: ModuleHandlersType = [
    module_handler(
        0,
        Some(Duration::from_secs(1)),
        |menu_handler: &MenuHandler| {
            const PHASES: [&str; 10] = [
                "[         ]",
                "[=        ]",
                "[==       ]",
                "[===      ]",
                "[====     ]",
                "[=====    ]",
                "[======   ]",
                "[=======  ]",
                "[======== ]",
                "[=========]",
            ];

            let current_duration: Duration = menu_handler.audio_handler.play_duration();
            let current_duration: u64 = current_duration.as_secs();

            let current_source_duration: Option<Duration> =
                menu_handler.audio_handler.current_source_duration;
            let play_percentage: f32 = if current_source_duration.is_some() && current_duration != 0
            {
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

            let play_percentage: usize = ((PHASES.len() as f32) * play_percentage) as usize;
            format!(" {} ",
                if play_percentage >= PHASES.len() {
                    PHASES[PHASES.len() - 1].to_string()
                } else {
                    PHASES[play_percentage].to_string()
                })
        },
        None,
    ),
    separator(0),
    // play change song callback name
    module_handler(
        1,
        None,
        |menu_handler: &MenuHandler| {
            SWITCH_SONG_CALLBACK_NAMES[menu_handler.switch_song_callback].to_string()
        },
        Some(
            |menu_handler: &MenuHandler, signals: &EnumMap<ModuleSignal, bool>| {
                if signals[ModuleSignal::ChangedSwitchSongCallback] {
                    return Some(
                        format!(" {} ",
                            &SWITCH_SONG_CALLBACK_NAMES[menu_handler.switch_song_callback]));
                }

                None
            },
        ),
    ),
    // current playlist
    separator(1),
    module_handler(
        2,
        None,
        |menu_handler: &MenuHandler| {
            format!(" {}: {} ",
                &menu_handler.playlist_names[menu_handler.main_menu.selected],
                &menu_handler.playlists[menu_handler.main_menu.selected][menu_handler.sub_menu.selected])
        },
        Some(
            |menu_handler: &MenuHandler, signals: &EnumMap<ModuleSignal, bool>| {
                if !signals[ModuleSignal::ChangedSong] {
                    return None;
                }

                Some(
                    format!(" {}: {} ",
                        &menu_handler.playlist_names[menu_handler.main_menu.selected],
                        &menu_handler.playlists[menu_handler.main_menu.selected][menu_handler.sub_menu.selected])
                )
            },
        ),
    ),
    separator(2),
];
