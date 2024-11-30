use {
    crate::{
        bind_callback::BindingCallback,
        cast,
        menu_handler::MenuHandler,
    },
    crossterm::event,
    std::{
        boxed::Box,
        io::Result,
    },
};

pub struct Binding {
    pub key_events: Vec<event::KeyEvent>,
    pub callback: Box<dyn BindingCallback>,
}
impl Binding {
    pub fn new(key_events: Vec<event::KeyEvent>, callback: Box<dyn BindingCallback>) -> Binding {
        return Binding {
            key_events: key_events,
            callback: callback,
        }
    }
}

pub struct KeyEventHandler {
    pub key_bindings: Vec<Binding>,
    pub key_events: Vec<event::KeyEvent>,
}
impl KeyEventHandler {
    pub fn new(key_bindings: Vec<Binding>) -> KeyEventHandler {
        let mut event_handler: KeyEventHandler = KeyEventHandler {
            key_bindings: key_bindings,
            key_events: Vec::new()
        };

        let mut max_key_binding_length = 0;
        for key_binding in &event_handler.key_bindings {
            if key_binding.key_events.len() > max_key_binding_length {
                max_key_binding_length = key_binding.key_events.len();
            }
        }
        event_handler.key_events.reserve(max_key_binding_length);
        return event_handler;
    }

    pub fn update(&mut self, event: event::KeyEvent, rs_player: &mut MenuHandler) -> Result<()> {
        self.key_events.push(event);

        let mut same_event_id: i32 = -1;
        let mut valid_event: bool = false;
        for i in 0..self.key_bindings.len() {
            if Self::same_event(&self.key_bindings[i].key_events, &self.key_events) {
                same_event_id = cast!(i);
            }
            if Self::valid_event(&self.key_bindings[i].key_events, &self.key_events) {
                valid_event = true;
            }
        }

        if !valid_event {
            self.key_events.clear();
            return Result::Ok(());
        }

        if same_event_id != -1 {
            self.key_events.clear();
            let same_event_id: usize = cast!(same_event_id);
            self.key_bindings[same_event_id].callback.callback(rs_player);
        }

        return Result::Ok(());
    }

    fn same_event(model: &Vec<event::KeyEvent>, follower: &Vec<event::KeyEvent>) -> bool {
        if model.len() != follower.len() {
            return false;
        }

        for model_key in model {
            for follower_key in follower {
                if model_key.code != follower_key.code || model_key.modifiers != follower_key.modifiers {
                    return false;
                }
            }
        }

        return true;
    }
    fn valid_event(model: &Vec<event::KeyEvent>, follower: &Vec<event::KeyEvent>) -> bool {
        if follower.len() > model.len() {
            return false;
        }

        for follower_key in follower {
            for model_key in model {
                if follower_key.code != model_key.code || follower_key.modifiers != model_key.modifiers {
                    return false;
                }
            }
        }

        return true;
    }
}
