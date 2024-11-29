use std::io::{
    self,
    Result
};

use crossterm::{
    style,
    QueueableCommand,
};

pub fn set_color(stdout: &mut io::Stdout, foreground: style::Color, background: style::Color) -> Result<()> {
    stdout
        .queue(style::SetForegroundColor(foreground))?
        .queue(style::SetBackgroundColor(background))?;

    return Result::Ok(());
}
pub fn reset_color(stdout: &mut io::Stdout) -> Result<()> {
    stdout
        .queue(style::ResetColor)?;

    return Result::Ok(());
}
