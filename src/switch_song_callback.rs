use crate::rs_player::RsPlayer;

pub type SwitchSongCallback = fn(rs_player: &mut RsPlayer);

pub fn callback_loop(rs_player: &mut RsPlayer) {
    rs_player.switch_song_to(rs_player.sub_menu.selected);
}
pub fn callback_next(rs_player: &mut RsPlayer) {
    let next_song: usize = if rs_player.sub_menu.selected + 1 > rs_player.get_current_playlist().len() {
        0
    } else {
        rs_player.sub_menu.selected + 1
    };

    rs_player.switch_song_to(next_song);
}
pub fn callback_random(rs_player: &mut RsPlayer) {
    let next_song: usize = fastrand::usize(0..rs_player.get_current_playlist().len());

    rs_player.switch_song_to(next_song);
}
