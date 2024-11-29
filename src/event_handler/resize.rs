use std::io::Result;
use crate::{
    cast,
    Menu,
};
use crossterm::terminal;

pub fn resize_menus(menu: &mut Menu, sub_menus: &mut Vec<Menu>) -> Result<()> {
    let (width, height) = terminal::size()?;

    resize_main_menu(menu, width, height);
    resize_sub_menus(sub_menus, width, height);

    return Result::Ok(());
}
fn resize_main_menu(menu: &mut Menu, width: u16, height: u16) {
    menu.x = 0;
    menu.y = 0;
    match width {
        0 => menu.width = 0,
        _ => menu.width = cast!(width / 2),
    }
    menu.height = cast!(height);
}
fn resize_sub_menus(menus: &mut Vec<Menu>, width: u16, height: u16) {
    let x: usize = match width {
        0 => 1,
        _ => cast!(width / 2),
    };
    let width: usize = match width {
        0 => 1,
        _ => cast!(width / 2),
    };

    for menu in menus {
        menu.x = x;
        menu.width = width;
        menu.y = 0;
        menu.height = cast!(height);
    }
}
