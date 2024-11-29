use {
    crate::{
        config,
        cast,
        menu::Menu,
        rs_player::RsPlayer,
    },
    crossterm::{
        event,
        terminal,
    },
    std::{
        io::Result,
        time::Duration,
    },
};

pub mod keys;

pub struct EventHandler {
    key_event_handler: keys::KeyEventHandler,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        return EventHandler {
            key_event_handler: keys::KeyEventHandler::new(config::init_key_bindings()),
        }
    }

    pub fn resize(rs_player: &mut RsPlayer) -> Result<()> {
        let (width, height) = terminal::size()?;

        Self::resize_main_menu(&mut rs_player.main_menu, width, height);
        Self::resize_sub_menu(&mut rs_player.sub_menu, width, height);

        return Result::Ok(());
    }
    fn resize_main_menu(main_menu: &mut Menu, width: u16, height: u16) {
        main_menu.x = 0;
        main_menu.y = 0;
        match width {
            0 => main_menu.width = 0,
            _ => main_menu.width = cast!(width / 2),
        }
        main_menu.height = cast!(height);
    }
    fn resize_sub_menu(sub_menu: &mut Menu, width: u16, height: u16) {
        let x: usize = match width {
            0 => 1,
            _ => cast!(width / 2),
        };
        let width: usize = match width {
            0 => 1,
            _ => cast!(width / 2),
        };

        sub_menu.x = x;
        sub_menu.width = width;
        sub_menu.y = 0;
        sub_menu.height = cast!(height);
    }

    pub fn update(&mut self, rs_player: &mut RsPlayer) -> Result<bool> {
        if event::poll(Duration::from_millis(config::FRAME_RATE_MS))? {
            let event: event::Event = event::read()?;

            let mut redraw: bool = false;

            match event {
                event::Event::Key(key_event) => {
                    self.key_event_handler.update(key_event);
                },
                event::Event::Resize(_, _) => {
                    Self::resize(rs_player)?;
                    redraw = true;
                }
                _ => {},
            }

            return Result::Ok(redraw);
        }

        return Result::Ok(false);
    }
}
