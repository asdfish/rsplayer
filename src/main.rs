pub mod audio;
pub mod flags;

use {
    audio::player::Player,
    flags::Config,
};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let conf = Config::new()?;
    let player = Player::new(conf)?;

    Ok(())
}
