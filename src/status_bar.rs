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
        io::Result,
        time::{
            Duration,
            Instant,
        },
    },
};

pub type ModuleCallback = fn(menu_handler: &MenuHandler) -> String;

pub struct StatusBar {
    module_handlers: Vec<ModuleHandler>,
}
impl StatusBar {
    pub fn new(menu_handler: &MenuHandler) -> StatusBar {
        let mut module_handlers: Vec<ModuleHandler> = config::init_status_bar_module_handlers();
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
    update_callback: ModuleCallback,

    pub print_string: String,
    last_update: Instant,
}
impl ModuleHandler {
    pub fn new(foreground: Color, background: Color, update_interval: Duration, update_callback: ModuleCallback) -> ModuleHandler {
        return ModuleHandler {
            foreground: foreground,
            background: background,

            update_interval: update_interval,
            update_callback: update_callback,

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
        self.print_string = (self.update_callback)(menu_handler);
    }
}
