#[cfg(not(target_os = "windows"))]
compile_error!("This only works on Windows.");

#[cfg(not(target_arch = "x86"))]
compile_error!("This only works on 32bit.");

pub mod d2;
pub mod library;

use d2::d2core::D2Core;
use library::Library;


fn dll_attach(base: winapi::shared::minwindef::LPVOID) {
    println!("Attach!");


    let game = Library::new("Game.exe".to_owned());
    let d2core = D2Core::new(game.clone());

    for _ in 0..15 {
        println!("");
        println!(
            "ScreenSize: {}x{}",
            d2::variables::get_screensize_x(&game),
            d2::variables::get_screensize_y(&game)
        ));
        println!(
            "Cursor Hover: ({}, {})",
            d2::variables::get_cursor_hover_x(&game),
            d2::variables::get_cursor_hover_y(&game)
        ));
        println!(
            "Mouse Pos: ({}, {})",
            d2::variables::get_mouse_pos_x(&game),
            d2::variables::get_mouse_pos_y(&game)
        ));
        println!(
            "Mouse Offset: ({}, {}, {})",
            d2::variables::get_mouse_offset_y(&game),
            d2::variables::get_mouse_offset_z(&game),
            d2::variables::get_mouse_offset_x(&game)
        ));
        println!(
            "Automap: On: {} Mode: {} Offset: {}",
            d2::variables::get_automap_on(&game),
            d2::variables::get_automap_mode(&game),
            d2::variables::get_automap_offset(&game)
        ));
        println!(
            "Viewport: ({}, {})",
            d2::variables::get_viewport_x(&game),
            d2::variables::get_viewport_y(&game)
        ));
        println!(
            "Gold Dialog: Action: {} Amount: {}",
            d2::variables::get_gold_dialog_action(&game),
            d2::variables::get_gold_dialog_amount(&game)
        ));
        println!(
            "NPC Menu Amount: {}",
            d2::variables::get_npc_menu_amount(&game)
        ));
        println!(
            "Regular Cursor Type: {}",
            d2::variables::get_regular_cursor_type(&game)
        ));
        println!(
            "Shop Cursor Type: {}",
            d2::variables::get_shop_cursor_type(&game)
        ));
        println!("FPS: {}", d2::variables::get_fps(&game)));
        println!("Skip: {}", d2::variables::get_skip(&game)));
        println!("Ping: {}", d2::variables::get_ping(&game)));
        println!("Lang: {}", d2::variables::get_lang(&game)));
        println!("Divisor: {}", d2::variables::get_divisor(&game)));
        println!(
            "Overhead Trigger: {}",
            d2::variables::get_overhead_trigger(&game)
        ));
        println!(
            "Recent Interact Id: {}",
            d2::variables::get_recent_interact_id(&game)
        ));
        println!(
            "Item Price List: {}",
            d2::variables::get_item_price_list(&game)
        ));
        println!("Waypoint Table: {}", d2::variables::get_waypoint_table(&game)));
        println!(
            "Is Weapon Swapped: {}",
            d2::variables::get_is_weapon_swapped(&game)
        ));
        println!(
            "Trade: Accepted: {} Blocked: {} Recent Trade Id: {}",
            d2::variables::get_is_trade_accepted(&game),
            d2::variables::get_is_trade_block(&game),
            d2::variables::get_recent_trade_id(&game)
        ));
        println!("Exp Char Flag: {}", d2::variables::get_exp_char_flag(&game)));
        println!("Map Id: {}", d2::variables::get_map_id(&game)));
        println!("Always Run: {}", d2::variables::get_always_run(&game)));
        println!("No Pickup: {}", d2::variables::get_no_pickup(&game)));
        println!("Chat Message: {}", d2::variables::get_chat_message(&game)));
        println!("Orfice Id: {}", d2::variables::get_orifice_id(&game)));
        println!(
            "Cursor Item Mode: {}",
            d2::variables::get_cursor_item_mode(&game)
        ));

        println!("");

        println!("Automap Size: {}", d2::functions::get_automap_size(&game)));
        println!("Difficulty: {}", d2::functions::get_difficulty(&game)));
        println!(
            "Game Language Code: {}",
            d2::functions::get_game_language_code(&game)
        ));
        println!(
            "Mouse Offset: ({}, {})",
            d2::functions::get_mouse_x_offset(&game),
            d2::functions::get_mouse_y_offset(&game)
        ));
        d2::functions::print_game_string(&d2core, "I love you", 0);
        //d2::functions::print_party_string(&game, "Spam", 1);

        //d2::functions::close_npc_interact(&game);
        //d2::functions::close_interact(&game);
        //d2::functions::exit_game(&d2core);

        println!("Running!");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

unsafe extern "system" fn dll_attach_wrapper(
    base: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::DWORD {
    match std::panic::catch_unwind(|| dll_attach(base)) {
        Err(e) => {
            println!("`dll_attach` has panicked: {:#?}", e);
        }
        Ok(_) => {}
    }

    rlwindows::free_library_and_exit_thread(base as _, 1);

    // This won't be executed because the
    unreachable!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    hinst_dll: winapi::shared::minwindef::HINSTANCE,
    fdw_reason: winapi::shared::minwindef::DWORD,
    _lpv_reserved: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::BOOL {

    match fdw_reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => {
            let mut thread_id: winapi::shared::minwindef::DWORD = 0;
            let thread_id_ptr: *mut winapi::shared::minwindef::DWORD =
                &mut thread_id as *mut _ as *mut winapi::shared::minwindef::DWORD;

            rlwindows::create_thread(
                std::ptr::null_mut(),
                0,
                Some(dll_attach_wrapper),
                hinst_dll as _,
                0,
                thread_id_ptr,
            );
        }
            }
        _ => {}
    }

    return winapi::shared::minwindef::TRUE;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn UnloadModule(_base: winapi::shared::minwindef::LPVOID) -> winapi::shared::minwindef::DWORD {
    println!("UnloadModule");

    return winapi::shared::minwindef::TRUE as _;
}
