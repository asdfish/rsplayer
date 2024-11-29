use std::io::{
    self,
    Result
};

use crossterm::{
    style,
    QueueableCommand,
};

pub fn bounded_text(width: usize, display_text: &String) -> Result<()> {
    if display_text.len() == width {
        text_borrow(&display_text)?;
        return Result::Ok(());
    }

    if display_text.len() < width {
        text_borrow(display_text)?;

        let x = display_text.len();
        empty_text(width - x)?;

        return Result::Ok(());
    }

    //let display_text: &str = &display_text[0..width];
    //text(display_text.to_string())?;

    let display_text: String = display_text.chars().take(width).collect();
    text(display_text)?;

    return Result::Ok(());
}
pub fn empty_text(width: usize) -> Result<()> {
    for _ in 0..width {
        text(" ".to_string())?;
    }

    return Result::Ok(());
}
pub fn text(text: String) -> Result<()> {
    return text_borrow(&text);
}
pub fn text_borrow(text: &String) -> Result<()> {
    io::stdout()
        .queue(style::Print(text))?;
    return Result::Ok(());
}
