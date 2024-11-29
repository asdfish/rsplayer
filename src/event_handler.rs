use std::io::Result;
use crate::Menu;
use crossterm::event;

pub mod keys;
pub mod resize;

pub struct EventHandler {
    key_event_handler: keys::KeyEventHandler,
    key_bindings: Vec<keys::Binding>,
}

impl EventHandler {
    pub fn new(key_bindings: Vec<keys::Binding>) -> EventHandler {
        return EventHandler {
            key_event_handler: keys::KeyEventHandler::new(&key_bindings),
            key_bindings: key_bindings,
        }
    }

    pub fn update(&mut self, main_menu: &mut Menu, sub_menus: &mut Vec<Menu>) -> Result<()> {
        let event: event::Event = event::read()?;

        match event {
            event::Event::Key(key_event) => {
                self.key_event_handler.update(key_event, &self.key_bindings);
            },
            event::Event::Resize(_, _) => {
                resize::resize_menus(main_menu, sub_menus)?;
            }
            _ => {},
        }

        return Result::Ok(());
    }
}
