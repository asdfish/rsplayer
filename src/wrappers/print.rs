use std::io::{
    self,
    Result
};

use crossterm::{
    style,
    QueueableCommand,
};

pub fn bounded_text(stdout: &mut io::Stdout, width: usize, display_text: &String) -> Result<()> {
    if display_text.len() == width {
        text_borrow(stdout, &display_text)?;
        return Result::Ok(());
    }

    if display_text.len() < width {
        text_borrow(stdout, display_text)?;

        let x = display_text.len();
        empty_text(stdout, width - x)?;

        return Result::Ok(());
    }

    let display_text: &str = &display_text[0..width];
    text(stdout, display_text.to_string())?;

    return Result::Ok(());
}
pub fn empty_text(stdout: &mut io::Stdout, width: usize) -> Result<()> {
    for _ in 0..width {
        text(stdout, " ".to_string())?;
    }

    return Result::Ok(());
}
pub fn text(stdout: &mut io::Stdout, text: String) -> Result<()> {
    return text_borrow(stdout, &text);
}
pub fn text_borrow(stdout: &mut io::Stdout, text: &String) -> Result<()> {
    stdout
        .queue(style::Print(text))?;
    return Result::Ok(());
}
