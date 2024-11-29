mod audio_handler;
mod config;
mod filesystem;
mod event_handler;
mod macros;
mod menu;
mod wrappers;
mod rs_player;

use {
    std::io::{
        stdout,
        Write,
    },
    event_handler::EventHandler,
    menu::Menu,
    rs_player::RsPlayer,
};

//fn draw_menus(main_menu: &mut Menu, main_menu_items: &Vec<String>, sub_menu: &mut Menu, sub_menu_items: &Vec<Vec<String>>) -> Result<()> {
//    main_menu.draw(&main_menu_items)?;
//    sub_menu.draw(&sub_menu_items[main_menu.selected])?;
//
//    return Result::Ok(());
//}

fn main() {
    let mut rs_player: RsPlayer = RsPlayer::new().unwrap();
    let mut event_handler: EventHandler = EventHandler::new();

    while rs_player.running {
        let _ = rs_player.draw();
        let _ = event_handler.update(&mut rs_player);
    }

    RsPlayer::uninit();
}

//fn main() {
//    let mut redraw: bool = true;
//    loop {
//        if redraw {
//            let _ = draw_menus(&mut main_menu, &playlist_names, &mut sub_menu, &playlists);
//            let _ = io::stdout()
//                .flush();
//        }
//
//        redraw = event_handler.update(&mut main_menu, &mut sub_menu).unwrap();
//    }
//
//    uninit().unwrap();
//}
