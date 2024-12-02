use {
    crate::{
        cast,
        config,
        wrappers,
    },
    std::io::Result,
};

pub struct Menu {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,

    pub camera: usize,
    pub cursor: usize,

    pub selected: usize,

    pub reverse_colors: bool,
}

impl Menu {
    pub const fn new() -> Menu {
        Menu {
            x: 0, y: 0, width: 0, height: 0,
            camera: 0, cursor: 0,

            selected: 0,

            reverse_colors: true,
        }
    }

    pub fn draw(&mut self, items: &[String]) -> Result<()> {
        if self.width == 0 || self.height == 0 {
            return Ok(());
        }

        if !items.is_empty() && self.cursor > items.len() - 1 {
            self.cursor = items.len();

            if self.cursor != 0 {
                self.cursor -= 1;
            }
        }

        if self.cursor < self.camera {
            self.camera = self.cursor;
        }
        if self.cursor > self.camera + self.height - 1 {
            self.camera = self.cursor - self.height + 1;
        }

        for i in 0..self.height {
            let item_y = i + self.camera;
            wrappers::cursor::move_to(cast!(self.x), cast!(self.y + i))?;

            if self.selected != item_y {
                if self.reverse_colors && self.cursor == item_y {
                    wrappers::style::set_color(config::NORMAL_FOREGROUND_REVERSED, config::NORMAL_BACKGROUND_REVERSED)?;
                } else {
                    wrappers::style::set_color(config::NORMAL_FOREGROUND, config::NORMAL_BACKGROUND)?;
                }
            } else if self.reverse_colors && self.cursor == item_y {
                wrappers::style::set_color(config::SELECTED_FOREGROUND_REVERSED, config::SELECTED_BACKGROUND_REVERSED)?;
            } else {
                wrappers::style::set_color(config::SELECTED_FOREGROUND, config::SELECTED_BACKGROUND)?;
            }

            if item_y >= items.len() {
                wrappers::print::empty_text(self.width)?;
                wrappers::style::reset_color()?;
                continue;
            }

            wrappers::print::bounded_text(self.width, &items[item_y])?;
            wrappers::style::reset_color()?;
        }

        Result::Ok(())
    }
    pub fn move_cursor(&mut self, step: isize) {
        let cursor: isize = cast!(self.cursor);
        let cursor: isize = cursor + step;

        if cursor >= 0 {
            self.cursor = cast!(cursor);
        } else {
            self.cursor = 0;
        }
    }
}
