use std::io::Result;
use crate::{
    cast,
    Menu,
};
use crossterm::terminal;

pub fn resize_menus(menu: &mut Menu, sub_menu: &mut Menu) -> Result<()> {
    let (width, height) = terminal::size()?;

    resize_main_menu(menu, width, height);
    resize_sub_menu(sub_menu, width, height);

    return Result::Ok(());
}
fn resize_main_menu(main_menu: &mut Menu, width: u16, height: u16) {
    main_menu.x = 0;
    main_menu.y = 0;
    match width {
        0 => main_menu.width = 0,
        _ => main_menu.width = cast!(width / 2),
    }
    main_menu.height = cast!(height);
}
fn resize_sub_menu(sub_menu: &mut Menu, width: u16, height: u16) {
    let x: usize = match width {
        0 => 1,
        _ => cast!(width / 2),
    };
    let width: usize = match width {
        0 => 1,
        _ => cast!(width / 2),
    };

    sub_menu.x = x;
    sub_menu.width = width;
    sub_menu.y = 0;
    sub_menu.height = cast!(height);
}
