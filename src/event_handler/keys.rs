use crossterm::event;

pub struct Event {
    pub keys: Vec<event::KeyEvent>,
    pub callback: fn(),
}

pub struct KeyEventHandler {
    pub events: Vec<event::Event>,
}

impl KeyEventHandler {
    fn new() -> KeyEventHandler {
        let mut event_handler: KeyEventHandler = KeyEventHandler {
            events: Vec::new()
        };

        event_handler.events.reserve(69);

        return event_handler;
    }

    fn update(&mut self, event: event::Event) {
        self.events.push(event);
    }
}
