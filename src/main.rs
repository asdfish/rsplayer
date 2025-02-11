pub mod flags;

use std::{
    boxed::Box,
    error::Error,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    use flags::Config;
    let conf = Config::new()?;
    Ok(())
}
