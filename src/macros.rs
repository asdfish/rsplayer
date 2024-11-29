#[macro_export]
macro_rules! cast {
    ($num:expr) => {
        $num.try_into().unwrap()
    }
}

#[macro_export]
macro_rules! get_menu {
    ($rs_player:expr) => {
        match $rs_player.selected_menu {
            0 => &mut $rs_player.main_menu,
            1 => &mut $rs_player.sub_menu,
            _ => unreachable!(),
        }
    }
}
