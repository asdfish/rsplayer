use std::io::{
    self,
    Result
};

use crossterm::{
    style,
    QueueableCommand,
};

pub fn set_color(foreground: style::Color, background: style::Color) -> Result<()> {
    io::stdout()
        .queue(style::SetForegroundColor(foreground))?
        .queue(style::SetBackgroundColor(background))?;

    return Result::Ok(());
}
pub fn reset_color() -> Result<()> {
    io::stdout()
        .queue(style::ResetColor)?;

    return Result::Ok(());
}
