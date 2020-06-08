#[cfg(not(target_os = "windows"))]
compile_error!("this only works for windows");

#[cfg(not(target_arch = "x86"))]
compile_error!("this only works for 32bit");

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

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    _dll_module: winapi::shared::minwindef::HINSTANCE,
    call_reason: winapi::shared::minwindef::DWORD,
    _reserved: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::BOOL {
    use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

    println!("TEST2");
    let load_library_cstring = std::ffi::CString::new("ScoopWoop").unwrap();
    unsafe {
        winapi::um::debugapi::OutputDebugStringA(load_library_cstring.as_ptr());
    }

    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        DLL_PROCESS_DETACH => {
            println!("Detatch2");
            let load_library_cstring = std::ffi::CString::new("Detach").unwrap();
            unsafe {
                winapi::um::debugapi::OutputDebugStringA(load_library_cstring.as_ptr());
            }

        }
        _ => {
        },
    }

    return winapi::shared::minwindef::TRUE;
}
