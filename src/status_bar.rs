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
    force_update: bool,
    module_handlers: config::StatusBarModuleHandlersType,
}
impl StatusBar {
    pub const fn new() -> StatusBar {
        return StatusBar {
            force_update: true,
            module_handlers: config::STATUS_BAR_MODULE_HANDLERS,
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
        if self.force_update {
            for module_handler in &mut self.module_handlers {
                module_handler.update_force(menu_handler);
            }
            return;
        }

        for module_handler in &mut self.module_handlers {
            module_handler.update(menu_handler);
        }
    }
}

pub struct ModuleHandler {
    foreground: Color,
    background: Color,

    update_interval: Option<Duration>,
    update_callback: ModuleCallback,

    pub print_string: String,
    last_update: Option<Instant>,
}
impl ModuleHandler {
    pub const fn new(foreground: Color, background: Color, update_interval: Option<Duration>, update_callback: ModuleCallback) -> ModuleHandler {
        return ModuleHandler {
            foreground: foreground,
            background: background,

            update_interval: update_interval,
            update_callback: update_callback,

            print_string: String::new(),
            last_update: None,
        };
    }

    pub fn draw(&self) -> Result<()> {
        style::set_color(self.foreground, self.background)?;
        print::text_borrow(&self.print_string)?;

        return Result::Ok(());
    }

    pub fn update(&mut self, menu_handler: &MenuHandler) {
        if self.update_interval.is_none() {
            return;
        }

        let now: Instant = Instant::now();

        if self.last_update.is_none() || now.duration_since(self.last_update.unwrap()) > self.update_interval.unwrap() {
            self.last_update = Some(now);
            self.update_force(menu_handler);
        }
    }
    pub fn update_force(&mut self, menu_handler: &MenuHandler) {
        self.print_string = (self.update_callback)(menu_handler);
    }
}
