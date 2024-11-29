use crate::rs_player::RsPlayer;

//pub fn move_cursor_x(rs_player: &mut RsPlayer) {
//}

pub fn quit(rs_player: &mut RsPlayer) {
    rs_player.running = false;
}
