use std::io::{
    self,
    Result
};

use crossterm::{
    style,
    ExecutableCommand,
};

pub fn set_color(foreground: style::Color, background: style::Color) -> Result<()> {
    io::stdout()
        .execute(style::SetForegroundColor(foreground))?
        .execute(style::SetBackgroundColor(background))?;

    return Result::Ok(());
}
pub fn reset_color() -> Result<()> {
    io::stdout()
        .execute(style::ResetColor)?;

    return Result::Ok(());
}
