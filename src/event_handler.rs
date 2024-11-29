use std::io::Result;
use crossterm::event;

pub mod keys;

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

    pub fn update(&mut self) -> Result<()> {
        let event: event::Event = event::read()?;

        match event {
            event::Event::Key(key_event) => {
                self.key_event_handler.update(key_event, &self.key_bindings);
            }
            _ => {}
        }

        return Result::Ok(());
    }
}
