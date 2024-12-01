use {
    crate::{
        config,
        wrappers::{
            cursor,
            print,
            style,
        },
        menu_handler::MenuHandler,
    },
    crossterm::style::Color,
    std::{
        boxed::Box,
        io::Result,
        time::{
            Duration,
            Instant,
        },
    },
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
            module_handler.draw()?;
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
    foreground: Color,
    background: Color,

    update_interval: Duration,
    module: Box<dyn StatusBarModule>,

    pub print_string: String,
    last_update: Instant,
}
impl ModuleHandler {
    pub fn new(foreground: Color, background: Color, update_interval: Duration, module: Box<dyn StatusBarModule>) -> ModuleHandler {
        return ModuleHandler {
            foreground: foreground,
            background: background,

            update_interval: update_interval,
            module: module,

            print_string: String::new(),
            last_update: Instant::now(),
        };
    }

    pub fn draw(&self) -> Result<()> {
        style::set_color(self.foreground, self.background)?;
        print::text_borrow(&self.print_string)?;

        return Result::Ok(());
    }

    pub fn update(&mut self, menu_handler: &MenuHandler) {
        let now: Instant = Instant::now();

        if now.duration_since(self.last_update) > self.update_interval {
            self.last_update = now;
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

pub struct PlayDuration {
    format: fn(Duration, menu_handler: &MenuHandler) -> String,
}
impl PlayDuration {
    pub fn new(format: fn(Duration, &MenuHandler) -> String) -> PlayDuration {
        return PlayDuration {
            format: format, // format! does not work on strings
        };
    }
}

impl StatusBarModule for PlayDuration {
    fn output(&self, menu_handler: &MenuHandler) -> String {
        let play_duration: Duration = menu_handler.audio_handler.play_duration();

        return (self.format)(play_duration, menu_handler);
    }
}
