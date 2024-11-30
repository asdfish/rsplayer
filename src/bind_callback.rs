use crate::{
    menu::Menu,
    rs_player::RsPlayer,
};

pub trait BindingCallback {
    fn callback(&self, rs_player: &mut RsPlayer);
}

pub enum CursorDirection {
    X, Y,
    TOP, BOTTOM,
    SELECTED,
}
pub struct MoveCursor {
    pub direction: CursorDirection,
    pub step: isize,
}
impl BindingCallback for MoveCursor {
    fn callback(&self, rs_player: &mut RsPlayer) {
        let menus: [&mut Menu; 2] = [&mut rs_player.main_menu, &mut rs_player.sub_menu];

        match self.direction {
            CursorDirection::X => {
                if self.step > 0 {
                    rs_player.selected_menu = 1;
                } else if self.step < 0 {
                    rs_player.selected_menu = 0;
                }
            },
            CursorDirection::Y => menus[rs_player.selected_menu].move_cursor(self.step),

            CursorDirection::TOP => menus[rs_player.selected_menu].cursor = 0,
            CursorDirection::BOTTOM => menus[rs_player.selected_menu].cursor = usize::MAX,

            CursorDirection::SELECTED => menus[rs_player.selected_menu].cursor = menus[rs_player.selected_menu].selected,
        }

        rs_player.redraw = true;
    }
}

pub struct SwitchSong {}
impl BindingCallback for SwitchSong {
    fn callback(&self, rs_player: &mut RsPlayer) {
        rs_player.switch_song();
    }
}

pub struct Select {}
impl BindingCallback for Select {
    fn callback(&self, rs_player: &mut RsPlayer) {
        match rs_player.selected_menu {
            0 => {
                rs_player.change_sub_menu(rs_player.main_menu.cursor);
            },
            1 => {
                rs_player.switch_song_to(rs_player.sub_menu.cursor);
            },
            _ => unreachable!(),
        }

        rs_player.redraw = true;
    }
}

pub struct Quit {}
impl BindingCallback for Quit {
    fn callback(&self, rs_player: &mut RsPlayer) {
        rs_player.running = false;
    }
}
