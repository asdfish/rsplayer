pub mod flags;

use flags::Config;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let conf = Config::new()?;

    Ok(())
}
