#[cfg(not(target_os = "windows"))]
compile_error!("This only works on Windows.");

#[cfg(not(target_arch = "x86"))]
compile_error!("This only works on 32bit.");

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

fn print_dbg(msg: &str) {
    let load_library_cstring = std::ffi::CString::new(msg).unwrap();
    unsafe {
        winapi::um::debugapi::OutputDebugStringA(load_library_cstring.as_ptr());
    }
    println!("{}", msg);
}

fn dll_attach(_base: winapi::shared::minwindef::LPVOID) {
    print_dbg("Attach!");
}

fn dll_detach() {
    print_dbg("Detatch!");
}

unsafe extern "system" fn dll_attach_wrapper(
    base: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::DWORD {
    use std::panic;

    match panic::catch_unwind(|| dll_attach(base)) {
        Err(e) => {
            print_dbg(&format!("`dll_attach` has panicked: {:#?}", e));
        }
        Ok(_) => {}
    }

    std::thread::sleep(std::time::Duration::from_secs(5));

    winapi::um::libloaderapi::FreeLibraryAndExitThread(base as _, 1);

    // This won't be executed because the
    unreachable!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    hinst_dll: winapi::shared::minwindef::HINSTANCE,
    fdw_reason: winapi::shared::minwindef::DWORD,
    lpv_reserved: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::BOOL {
    print_dbg(&format!("DllMain: {}", fdw_reason));

    match fdw_reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => {
            let mut thread_id: winapi::shared::minwindef::DWORD = 0;
            let thread_id_ptr: *mut winapi::shared::minwindef::DWORD =
                &mut thread_id as *mut _ as *mut winapi::shared::minwindef::DWORD;

            create_thread(
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

fn disable_thread_library_calls(lib_module: winapi::shared::minwindef::HMODULE) -> bool {
    unsafe {
        let ret = winapi::um::libloaderapi::DisableThreadLibraryCalls(lib_module);
        ret == winapi::shared::minwindef::TRUE
    }
}

fn create_thread(
    thread_attributes: winapi::um::minwinbase::LPSECURITY_ATTRIBUTES,
    stack_size: winapi::shared::basetsd::SIZE_T,
    start_address: winapi::um::minwinbase::LPTHREAD_START_ROUTINE,
    parameter: winapi::shared::minwindef::LPVOID,
    creation_flags: winapi::shared::minwindef::DWORD,
    thread_id: winapi::shared::minwindef::LPDWORD,
) -> winapi::um::winnt::HANDLE {
    unsafe {
        winapi::um::processthreadsapi::CreateThread(
            thread_attributes,
            stack_size,
            start_address,
            parameter,
            creation_flags,
            thread_id,
        )
    }
}
