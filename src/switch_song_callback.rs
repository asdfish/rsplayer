use {
    crate::rs_player::RsPlayer,
    std::time::Duration,
};

pub type SwitchSongCallback = fn(rs_player: &mut RsPlayer);

pub fn callback_loop(rs_player: &mut RsPlayer) {
    rs_player.switch_song_to(rs_player.sub_menu.selected);
}
