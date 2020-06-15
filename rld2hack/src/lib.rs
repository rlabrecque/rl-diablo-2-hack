#[cfg(not(target_os = "windows"))]
compile_error!("This only works on Windows.");

#[cfg(not(target_arch = "x86"))]
compile_error!("This only works on 32bit.");

mod d2;
mod library;

use library::Library;

//use detour::static_detour;
//use lazy_static;

/*type createmove_fn = fn(f32, *mut UserCmd) -> bool;

struct UserCmd {
    /* dscode here */
}

struct FunctionPtrAddress {
    addy: createmove_fn
}

lazy_static! {
    static ref fn_ptrs: FunctionPtrAddress = FunctionPtrAddress {
        addy: unsafe {
            std::mem::transmute::<usize, createmove_fn>(0x10111790)
        }
    };
}

static_detour! {
    struct CreateMoveDetour: fn(f32, *mut UserCmd) -> bool;
}

fn createmove_hook(input_sample_time: f32, cmd: *mut UserCmd) -> bool {
    println!("original function");

    return (fn_ptrs.addy)(input_sample_time, cmd);
}*/

fn init() {
    //unsafe { winapi::um::consoleapi::AllocConsole() };

    let load_library_cstring = std::ffi::CString::new("InitializingDebugString").unwrap();
    unsafe {
        winapi::um::debugapi::OutputDebugStringA(load_library_cstring.as_ptr());
    }

    println!("Initializing...");

    /*let closure_for_createmove = |input_sample_time, cmd| {
        println!("heres the detour. put your code in here");

        return (fn_ptrs.addy)(input_sample_time, cmd);
    };

    let mut hook = unsafe {
        CreateMoveDetour.initialize(createmove_hook, closure_for_createmove).unwrap()
    };

    unsafe {
        hook.enable().unwrap();
    }

    createmove_hook(1.0, std::ptr::null_mut()); // call this so hook.call works
    hook.call(100.0, std::ptr::null_mut());*/
}

pub fn print_dbg(msg: &str) {
    let msg_cstring = std::ffi::CString::new(msg).unwrap();
    unsafe {
        winapi::um::debugapi::OutputDebugStringA(msg_cstring.as_ptr());
    }
    println!("{}", msg);
}

fn dll_attach(base: winapi::shared::minwindef::LPVOID) {
    rlwindows::alloc_console();
    rlwindows::disable_thread_library_calls(base as _);

    print_dbg("Attach!");

    let game = Library::new("Game.exe".to_owned());

    print_dbg(&format!("Offset: {:#?}", game.handle));

    for _ in 0..40 {
        print_dbg("");
        print_dbg(&format!("ScreenSize: {}x{}", d2::get_screensize_x(&game), d2::get_screensize_y(&game)));
        print_dbg(&format!("Cursor Hover: ({}, {})", d2::get_cursor_hover_x(&game), d2::get_cursor_hover_y(&game)));
        print_dbg(&format!("Mouse Pos: ({}, {})", d2::get_mouse_pos_x(&game), d2::get_mouse_pos_y(&game)));
        print_dbg(&format!("Mouse Offset: ({}, {}, {})", d2::get_mouse_offset_y(&game), d2::get_mouse_offset_z(&game), d2::get_mouse_offset_x(&game)));
        print_dbg(&format!("Automap: On: {} Mode: {} Offset: {}", d2::get_automap_on(&game), d2::get_automap_mode(&game), d2::get_automap_offset(&game)));
        print_dbg(&format!("Viewport: ({}, {})", d2::get_viewport_x(&game), d2::get_viewport_y(&game)));
        print_dbg(&format!("Gold Dialog: Action: {} Amount: {}", d2::get_gold_dialog_action(&game), d2::get_gold_dialog_amount(&game)));
        print_dbg(&format!("NPC Menu Amount: {}", d2::get_npc_menu_amount(&game)));
        print_dbg(&format!("Regular Cursor Type: {}", d2::get_regular_cursor_type(&game)));
        print_dbg(&format!("Shop Cursor Type: {}", d2::get_shop_cursor_type(&game)));
        print_dbg(&format!("FPS: {}", d2::get_fps(&game)));
        print_dbg(&format!("Skip: {}", d2::get_skip(&game)));
        print_dbg(&format!("Ping: {}", d2::get_ping(&game)));
        print_dbg(&format!("Lang: {}", d2::get_lang(&game)));
        print_dbg(&format!("Divisor: {}", d2::get_divisor(&game)));
        print_dbg(&format!("Overhead Trigger: {}", d2::get_overhead_trigger(&game)));
        print_dbg(&format!("Recent Interact Id: {}", d2::get_recent_interact_id(&game)));
        print_dbg(&format!("Item Price List: {}", d2::get_item_price_list(&game)));
        print_dbg(&format!("Waypoint Table: {}", d2::get_waypoint_table(&game)));
        print_dbg(&format!("Is Weapon Swapped: {}", d2::get_is_weapon_swapped(&game)));
        print_dbg(&format!("Trade: Accepted: {} Blocked: {} Recent Trade Id: {}", d2::get_is_trade_accepted(&game), d2::get_is_trade_block(&game), d2::get_recent_trade_id(&game)));
        print_dbg(&format!("Exp Char Flag: {}", d2::get_exp_char_flag(&game)));
        print_dbg(&format!("Map Id: {}", d2::get_map_id(&game)));
        print_dbg(&format!("Always Run: {}", d2::get_always_run(&game)));
        print_dbg(&format!("No Pickup: {}", d2::get_no_pickup(&game)));
        print_dbg(&format!("Chat Message: {}", d2::get_chat_message(&game)));
        print_dbg(&format!("Orfice Id: {}", d2::get_orifice_id(&game)));
        print_dbg(&format!("Cursor Item Mode: {}", d2::get_cursor_item_mode(&game)));
        print_dbg("");
        print_dbg(&format!("Difficulty: {}", d2::functions::get_difficulty(&game)));
        std::thread::sleep(std::time::Duration::from_millis(1000));

        /*if i > 3 {
            d2::functions::exit_game(&game);
        }*/
    }
}

fn dll_detach() {
    print_dbg("Detatch!");
}

unsafe extern "system" fn dll_attach_wrapper(
    base: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::DWORD {
    match std::panic::catch_unwind(|| dll_attach(base)) {
        Err(e) => {
            print_dbg(&format!("`dll_attach` has panicked: {:#?}", e));
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
    print_dbg(&format!("DllMain: {}", fdw_reason));

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
        winapi::um::winnt::DLL_PROCESS_DETACH => match std::panic::catch_unwind(|| dll_detach()) {
            Err(e) => {
                print_dbg(&format!("`dll_detach` has panicked: {:#?}", e));
            }
            Ok(_) => {}
        },
        _ => {}
    }

    return winapi::shared::minwindef::TRUE;
}
