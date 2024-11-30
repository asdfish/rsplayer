use {
    crate::{
        config,
        wrappers::{
            cursor,
            print,
        },
        menu_handler::MenuHandler,
    },
    std::{
        boxed::Box,
        io::Result,
        time::{
            Duration,
            Instant,
        },
    },
};
#[cfg(feature = "play_position_module")]
use chrono::{
    DateTime,
    Local,
};

pub struct StatusBar {
    module_handlers: Vec<ModuleHandler>,
}
impl StatusBar {
    pub fn new(menu_handler: &MenuHandler) -> StatusBar {
        let mut module_handlers: Vec<ModuleHandler> = config::init_status_bar();
        for module_handler in &mut module_handlers {
            module_handler.update_force(menu_handler);
        }

        return StatusBar {
            module_handlers: module_handlers,
        };
    }

    pub fn draw(&self) -> Result<()> {
        cursor::move_to(0, 0)?;
        for module_handler in &self.module_handlers {
            print::text_borrow(&module_handler.print_string)?;
        }

        return Result::Ok(());
    }

    pub fn update(&mut self, menu_handler: &MenuHandler) {
        for module_handler in &mut self.module_handlers {
            module_handler.update(menu_handler);
        }
    }
}

pub struct ModuleHandler {
    update_interval: Duration,
    module: Box<dyn StatusBarModule>,

    pub print_string: String,
    last_update: Instant,
}
impl ModuleHandler {
    pub fn new(update_interval: Duration, module: Box<dyn StatusBarModule>) -> ModuleHandler {
        return ModuleHandler {
            update_interval: update_interval,
            module: module,

            print_string: String::new(),
            last_update: Instant::now(),
        };
    }

    pub fn update(&mut self, menu_handler: &MenuHandler) {
        let now: Instant = Instant::now();

        if now.duration_since(self.last_update) > self.update_interval {
            self.update_force(menu_handler);
        }
    }
    pub fn update_force(&mut self, menu_handler: &MenuHandler) {
        self.print_string = self.module.output(menu_handler);
    }
}

pub trait StatusBarModule {
    fn output(&self, menu_handler: &MenuHandler) -> String;
}

#[cfg(feature = "play_position_module")]
pub struct PlayPosition {
    format: String,
}
impl PlayPosition {
    pub fn new(format: String) -> PlayPosition {
        return PlayPosition {
            format: format,
        };
    }
}
#[cfg(feature = "play_position_module")]
#[allow(unused_variables)]
impl StatusBarModule for PlayPosition {
    fn output(&self, menu_handler: &MenuHandler) -> String {
        let now: DateTime<Local> = Local::now();

        return now.format(&self.format).to_string();
    }
}
