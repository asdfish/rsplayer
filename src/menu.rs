use crate::cast;

use std::io::{
    self,
    Result,
};

use crossterm::{
    ExecutableCommand,
    cursor,
    style,
};

fn cursor_move_to(x: u16, y: u16) -> Result<()> {
    io::stdout()
        .execute(cursor::MoveTo(x, y))?;

    return Result::Ok(());
}

fn print_bounded_text(width: usize, text: &String) -> Result<()> {
    if text.len() == width {
        print_text_borrow(&text)?;
        return Result::Ok(());
    }

    if text.len() < width {
        print_text_borrow(text)?;

        let x = text.len();
        print_empty_text(width - x)?;

        return Result::Ok(());
    }

    let text: &str = &text[0..width];
    print_text(text.to_string())?;

    return Result::Ok(());
}
fn print_empty_text(width: usize) -> Result<()> {
    for _ in 0..width {
        print_text(" ".to_string())?;
    }

    return Result::Ok(());
}
fn print_text(text: String) -> Result<()> {
    return print_text_borrow(&text);
}
fn print_text_borrow(text: &String) -> Result<()> {
    io::stdout()
        .execute(style::Print(text))?;
    return Result::Ok(());
}

fn style_set_color(foreground: style::Color, background: style::Color) -> Result<()> {
    io::stdout()
        .execute(style::SetForegroundColor(foreground))?
        .execute(style::SetBackgroundColor(background))?;

    return Result::Ok(());
}
fn style_reset_color() -> Result<()> {
    io::stdout()
        .execute(style::ResetColor)?;

    return Result::Ok(());
}

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
            cursor_move_to(cast!(self.x), cast!(self.y + i))?;

            if self.reverse_colors && self.cursor == item_y {
                style_set_color(self.foreground_reversed, self.background_reversed)?;
            } else {
                style_set_color(self.foreground, self.background)?;
            }

            if item_y >= self.items.len() {
                print_empty_text(self.width)?;
                style_reset_color()?;
                continue;
            }

            print_bounded_text(self.width, &self.items[item_y])?;
            style_reset_color()?;
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
