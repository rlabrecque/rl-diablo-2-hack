#[cfg(not(target_os = "windows"))]
compile_error!("This only works on Windows.");

#[cfg(not(target_arch = "x86"))]
compile_error!("This only works on 32bit.");

mod library;

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

    std::thread::sleep(std::time::Duration::from_secs(5));

    let game = library::Library::new("Game.exe".to_owned());

    print_dbg(&format!("Offset: {:#?}", game.handle));
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
