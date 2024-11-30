use {
    crate::{
        config,
        cast,
        menu::Menu,
        menu_handler::MenuHandler,
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

    pub fn resize(menu_handler: &mut MenuHandler) -> Result<()> {
        let (width, height) = terminal::size()?;

        return Self::resize_to(menu_handler, width, height);
    }

    pub fn resize_to(menu_handler: &mut MenuHandler, width: u16, height: u16) -> Result<()> {
        Self::resize_main_menu(&mut menu_handler.main_menu, width, height);
        Self::resize_sub_menu(&mut menu_handler.sub_menu, width, height);

        menu_handler.redraw = true;
        return Result::Ok(());
    }
    fn resize_main_menu(main_menu: &mut Menu, width: u16, height: u16) {
        main_menu.x = 0;
        main_menu.y = 1;
        match width {
            0 => main_menu.width = 0,
            _ => main_menu.width = cast!(width / 2),
        }
        main_menu.height = cast!(height);

        main_menu.height -= if main_menu.height != 0 {
            1
        } else {
            0
        }
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
        sub_menu.y = 1;
        sub_menu.height = cast!(height);

        sub_menu.height -= if sub_menu.height != 0 {
            1
        } else {
            0
        }
    }

    pub fn update(&mut self, menu_handler: &mut MenuHandler) -> Result<()> {
        if event::poll(Duration::from_millis(config::FRAME_RATE_MS))? {
            let event: event::Event = event::read()?;

            match event {
                event::Event::Key(key_event) => {
                    self.key_event_handler.update(key_event, menu_handler)?;
                },
                event::Event::Resize(_, _) => {
                    Self::resize(menu_handler)?;
                }
                _ => {},
            }
        }

        return Result::Ok(());
    }
}
