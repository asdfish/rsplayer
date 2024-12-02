use std::io::{
    self,
    Result
};

use crossterm::{
    style,
    QueueableCommand,
};

#[inline(always)]
pub fn set_foreground(foreground: style::Color) -> Result<()> {
    return set_foreground_borrow(&foreground);
}
#[inline(always)]
pub fn set_background(background: style::Color) -> Result<()> {
    return set_background_borrow(&background);
}
#[inline(always)]
pub fn set_foreground_borrow(foreground: &style::Color) -> Result<()> {
    io::stdout()
        .queue(style::SetForegroundColor(*foreground))?;
    return Result::Ok(());
}
#[inline(always)]
pub fn set_background_borrow(background: &style::Color) -> Result<()> {
    io::stdout()
        .queue(style::SetBackgroundColor(*background))?;
    return Result::Ok(());
}
#[inline(always)]
pub fn set_color(foreground: style::Color, background: style::Color) -> Result<()> {
    set_background(background)?;
    set_foreground(foreground)?;
    return Result::Ok(());
}

#[inline(always)]
pub fn reset_color() -> Result<()> {
    io::stdout()
        .queue(style::ResetColor)?;
    return Result::Ok(());
}
