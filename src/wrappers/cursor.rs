use std::io::{
    self,
    Result
};

use crossterm::{
    cursor,
    ExecutableCommand,
};

pub fn move_to(x: u16, y: u16) -> Result<()> {
    io::stdout()
        .execute(cursor::MoveTo(x, y))?;

    return Ok(());
}
