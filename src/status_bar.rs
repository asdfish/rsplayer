use {
    crate::{
        cast,
        config,
        wrappers,
        event_handler::EventHandler,
        menu_handler::MenuHandler,
    },
    crossterm::{
        self,
        style::Color,
    },
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
    redraw: bool,
    pub force_update: bool,
    module_handlers: config::StatusBarModuleHandlersType,
}
impl StatusBar {
    pub const fn new() -> StatusBar {
        return StatusBar {
            redraw: true,
            force_update: true,
            module_handlers: config::STATUS_BAR_MODULE_HANDLERS,
        };
    }

    pub fn draw(&mut self, event_handler: &EventHandler) -> Result<()> {
        if !self.redraw {
            return Result::Ok(());
        }

        wrappers::cursor::move_to(0, 0)?;
        for module_handler in &self.module_handlers {
            let (x, _) = crossterm::cursor::position()?;
            let x: usize = cast!(x);
            if x + module_handler.print_string.len() >= cast!(event_handler.width) {
                let bounds: usize = (((event_handler.width - 1) as usize) - x).into();
                module_handler.draw_bounded(bounds)?;
                break;
            }
            module_handler.draw()?;
        }

        let (x, y) = crossterm::cursor::position()?;
        if y == 0 {
            let undrawn: u16 = event_handler.width - x;
            wrappers::style::set_background_borrow(&config::STATUS_BAR_BACKGROUND)?;
            wrappers::print::empty_text(cast!(undrawn))?;
        }

        self.redraw = false;
        return Result::Ok(());
    }

    pub fn update(&mut self, menu_handler: &MenuHandler) {
        if self.force_update {
            self.redraw = true;
            for module_handler in &mut self.module_handlers {
                module_handler.update_force(menu_handler);
            }
            return;
        }

        for module_handler in &mut self.module_handlers {
            let redraw: bool = module_handler.update(menu_handler);
            if redraw {
                self.redraw = redraw;
            }
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
        wrappers::style::set_color(self.foreground, self.background)?;
        wrappers::print::text_borrow(&self.print_string)?;

        return Result::Ok(());
    }

    pub fn draw_bounded(&self, bounds: usize) -> Result<()> {
        wrappers::style::set_color(self.foreground, self.background)?;
        wrappers::print::bounded_text(bounds, &self.print_string)?;

        return Result::Ok(());
    }

    pub fn update(&mut self, menu_handler: &MenuHandler) -> bool {
        if self.update_interval.is_none() {
            return false;
        }

        let now: Instant = Instant::now();

        if self.last_update.is_none() || now.duration_since(self.last_update.unwrap()) > self.update_interval.unwrap() {
            self.last_update = Some(now);
            self.update_force(menu_handler);
            return true;
        }

        return false;
    }
    pub fn update_force(&mut self, menu_handler: &MenuHandler) {
        self.print_string = (self.update_callback)(menu_handler);
    }
}
