use crate::cast;
use crate::wrappers;

use std::io::{
    self,
    Result,
};

use crossterm::{
    ExecutableCommand,
    cursor,
    style,
};

pub struct Menu {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,

    pub camera: usize,
    pub cursor: usize,

    pub foreground: style::Color,
    pub foreground_reversed: style::Color,
    pub background: style::Color,
    pub background_reversed: style::Color,

    pub items: Vec<String>,

    pub reverse_colors: bool,
}

impl Menu {
    pub fn new(foreground: style::Color, foreground_reversed: style::Color,
        background: style::Color, background_reversed: style::Color,

        items: Vec<String>) -> Menu {

        return Menu {
            x: 0, y: 0, width: 0, height: 0,
            camera: 0, cursor: 0,

            foreground: foreground, foreground_reversed: foreground_reversed,
            background: background, background_reversed: background_reversed,

            items: items,

            reverse_colors: true,
        };
    }

    pub fn draw(&mut self) -> Result<()> {
        if self.cursor < self.camera {
            self.camera = self.cursor;
        }
        if self.cursor > self.camera + self.height - 1 {
            self.camera = self.cursor - self.height + 1;
        }

        for i in 0..self.height {
            let item_y = i + self.camera;
            wrappers::cursor::move_to(cast!(self.x), cast!(self.y + i))?;

            if self.reverse_colors && self.cursor == item_y {
                wrappers::style::set_color(self.foreground_reversed, self.background_reversed)?;
            } else {
                wrappers::style::set_color(self.foreground, self.background)?;
            }

            if item_y >= self.items.len() {
                wrappers::print::empty_text(self.width)?;
                wrappers::style::reset_color()?;
                continue;
            }

            wrappers::print::bounded_text(self.width, &self.items[item_y])?;
            wrappers::style::reset_color()?;
        }

        return Result::Ok(());
    }

    pub fn move_cursor(&mut self, step: isize) {
        let cursor: isize = cast!(self.cursor);
        let cursor: isize = cursor + step;

        if cursor >= 0 {
            self.cursor = cast!(cursor);

            if self.cursor > self.items.len() {
                self.cursor = self.items.len() - 1;
            }
        } else {
            self.cursor = 0;
        }
    }
}
