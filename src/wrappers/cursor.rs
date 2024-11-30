use std::io::{
    self,
    Result
};

use crossterm::{
    cursor,
    QueueableCommand,
};

pub fn move_to(x: u16, y: u16) -> Result<()> {
    io::stdout()
        .queue(cursor::MoveTo(x, y))?;

    return Ok(());
}
