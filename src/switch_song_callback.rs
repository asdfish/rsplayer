use crate::menu_handler::MenuHandler;

pub type SwitchSongCallback = fn(menu_handler: &mut MenuHandler);

pub fn callback_loop(menu_handler: &mut MenuHandler) {
    menu_handler.switch_song_to(menu_handler.sub_menu.selected);
}
pub fn callback_next(menu_handler: &mut MenuHandler) {
    let next_song: usize = if menu_handler.sub_menu.selected + 1 > menu_handler.get_current_playlist().len() {
        0
    } else {
        menu_handler.sub_menu.selected + 1
    };

    menu_handler.switch_song_to(next_song);
}
pub fn callback_random(menu_handler: &mut MenuHandler) {
    let next_song: usize = fastrand::usize(0..menu_handler.get_current_playlist().len());

    menu_handler.switch_song_to(next_song);
}
