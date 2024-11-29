use crate::rs_player::RsPlayer;

pub fn quit(rs_player: &mut RsPlayer) -> bool {
    rs_player.running = false;
    return false;
}
