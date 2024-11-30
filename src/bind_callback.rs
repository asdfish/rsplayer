use crate::{
    menu::Menu,
    menu_handler::MenuHandler,
};

pub trait BindingCallback {
    fn callback(&self, menu_handler: &mut MenuHandler);
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
    fn callback(&self, menu_handler: &mut MenuHandler) {
        let menus: [&mut Menu; 2] = [&mut menu_handler.main_menu, &mut menu_handler.sub_menu];

        match self.direction {
            CursorDirection::X => {
                if self.step > 0 {
                    menu_handler.selected_menu = 1;
                } else if self.step < 0 {
                    menu_handler.selected_menu = 0;
                }
            },
            CursorDirection::Y => menus[menu_handler.selected_menu].move_cursor(self.step),

            CursorDirection::TOP => menus[menu_handler.selected_menu].cursor = 0,
            CursorDirection::BOTTOM => menus[menu_handler.selected_menu].cursor = usize::MAX,

            CursorDirection::SELECTED => menus[menu_handler.selected_menu].cursor = menus[menu_handler.selected_menu].selected,
        }

        menu_handler.redraw = true;
    }
}

pub struct SwitchSong {}
impl BindingCallback for SwitchSong {
    fn callback(&self, menu_handler: &mut MenuHandler) {
        menu_handler.switch_song();
    }
}

pub struct Select {}
impl BindingCallback for Select {
    fn callback(&self, menu_handler: &mut MenuHandler) {
        match menu_handler.selected_menu {
            0 => {
                menu_handler.change_sub_menu(menu_handler.main_menu.cursor);
            },
            1 => {
                menu_handler.switch_song_to(menu_handler.sub_menu.cursor);
            },
            _ => unreachable!(),
        }

        menu_handler.redraw = true;
    }
}

pub struct Quit {}
impl BindingCallback for Quit {
    fn callback(&self, menu_handler: &mut MenuHandler) {
        menu_handler.running = false;
    }
}
