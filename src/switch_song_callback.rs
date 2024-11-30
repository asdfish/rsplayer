use crate::rs_player::RsPlayer;

pub type SwitchSongCallback = fn(rs_player: &mut RsPlayer);

pub fn callback_loop(rs_player: &mut RsPlayer) {
    rs_player.switch_song_to(rs_player.sub_menu.selected);
}

pub fn callback_next(rs_player: &mut RsPlayer) {
    rs_player.sub_menu.selected += 1;

    let current_playlist_length: usize = rs_player.playlists[rs_player.main_menu.selected].len();
    if rs_player.sub_menu.selected >= current_playlist_length {
        rs_player.sub_menu.selected = 0;
    }

    rs_player.switch_song_to(rs_player.sub_menu.selected);
}
