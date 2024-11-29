use crate::{
    cast,
    get_menu,
    menu::Menu,
    rs_player::RsPlayer,
};

pub trait Callback {
    fn callback(&self, rs_player: &mut RsPlayer);
}

pub enum CursorDirection {
    X, Y,
}
pub struct MoveCursor {
    pub direction: CursorDirection,
    pub step: isize,
}
impl Callback for MoveCursor {
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
            CursorDirection::Y => {
                menus[rs_player.selected_menu].move_cursor(self.step);
            }
        }

        rs_player.redraw = true;
    }
}

pub struct Quit {}
impl Callback for Quit {
    fn callback(&self, rs_player: &mut RsPlayer) {
        rs_player.running = false;
    }
}
