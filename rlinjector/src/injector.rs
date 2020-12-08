mod winutils;

/// Enable Debug Privileges
/// This must be called once at startup to ensure we have the appropriate privileges
/// Returns Ok
pub fn enable_debug_privilege() -> Result<(), Box<dyn std::error::Error>> {
    if !winutils::enable_debug_privilege() {
        return Err("Could not enable Debug Privileges.".into());
    }

    Ok(())
}

///
///
pub fn is_dll_loaded_in_pid(
    dll_path: &std::path::PathBuf,
    process_id: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let process_handle = rlwindows::open_process(
        process_id,
        winapi::um::winnt::PROCESS_QUERY_INFORMATION | winapi::um::winnt::PROCESS_VM_READ,
    );

    if process_handle.is_null() {
        rlwindows::print_get_last_err();
        let message = format!("Process id '{}' does not exist or is not accessible.", process_id);
        return Err(message.into());
    }

    let module = winutils::find_module(process_handle, dll_path);

    rlwindows::close_handle(process_handle);

    Ok(!module.is_null())
}

/// Injects the specified dll into all the processes with a given name.
/// Returns the process id's of the injected processes.
#[cfg(target_arch = "x86")]
pub fn inject_dll_into_process_name(
    dll_path: &std::path::PathBuf,
    process_name: &str,
) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    println!("Injecting dll '{}' into '{}'", dll_path.display(), process_name);

    if !dll_path.exists() {
        let err_msg = format!("DLL path specified does not exist: '{}'", dll_path.display());
        return Err(err_msg.into());
    }

    let process_ids: Vec<u32> = winutils::get_process_ids_from_name(&process_name);
    if process_ids.is_empty() {
        let err_msg = format!("Process '{}' does not exist.", process_name);
        return Err(err_msg.into());
    }

    inject_dll_into_process_ids(dll_path, process_ids)
}

/// Injects the specified dll into all the processes with a given name.
/// Returns the process id's of the injected processes.
#[cfg(target_arch = "x86")]
fn inject_dll_into_process_ids(
    dll_path: &std::path::PathBuf,
    process_ids: Vec<u32>,
) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut injected_process_ids: Vec<u32> = Vec::new();
    for process_id in process_ids {
        let process_handle = rlwindows::open_process(
            process_id,
            winapi::um::winnt::PROCESS_CREATE_THREAD
                | winapi::um::winnt::PROCESS_QUERY_INFORMATION
                | winapi::um::winnt::PROCESS_VM_OPERATION
                | winapi::um::winnt::PROCESS_VM_WRITE
                | winapi::um::winnt::PROCESS_VM_READ,
        );

        if process_handle.is_null() {
            rlwindows::print_get_last_err();
            println!("Process id '{}' does not exist or is not accessible.", process_id);
            continue;
        }

        println!("Process {} successfully opened.", process_id);

        let remote_module = winutils::find_remote_module_by_path(process_id, dll_path);

        if !remote_module.is_null() {
            println!("DLL already exists in process. HMODULE: {:?}.", remote_module);
            println!("Injection failed.");
        } else {
            if winutils::inject_library(process_handle, &dll_path) {
                println!("Successfully injected '{}' into '{}'.", dll_path.display(), process_id);
                injected_process_ids.push(process_id);
            } else {
                println!("Injection failed.");
            }
        }

        rlwindows::close_handle(process_handle);
    }

    Ok(injected_process_ids)
}

#[cfg(target_arch = "x86")]
pub fn unload(dll_path: &std::path::PathBuf, process_ids: Vec<u32>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Unloading Dll: {}.", dll_path.display());

    for process_id in process_ids {
        let process_handle: winapi::um::winnt::HANDLE =
            rlwindows::open_process(process_id, winapi::um::winnt::PROCESS_ALL_ACCESS);

        let remote_module: winapi::shared::minwindef::HMODULE =
            winutils::find_remote_module_by_path(process_id, dll_path);

        if remote_module != std::ptr::null_mut() {
            println!("DLL exists in process. HMODULE: {:?}.", remote_module);
            let success = winutils::unload_library(process_handle, remote_module);
            //println!("unload_library: {}", success);
        } else {
            println!("Could not find {} in process: {}", dll_path.display(), process_id);
        }

        println!("Closing process {}", process_id);
        if process_handle != std::ptr::null_mut() {
            rlwindows::close_handle(process_handle);
        }
    }

    Ok(())
}

pub fn create_process_and_inject_library(
    exe_path: &std::path::Path,
    dll_path: &std::path::Path,
    process_id: &mut u32,
) -> bool {
    if !exe_path.exists() {
        println!("Executable {:?} does not exist.", exe_path);
        return false;
    }

    if !dll_path.exists() {
        println!("DLL {:?} does not exist.", dll_path);
        return false;
    }

    if !winutils::enable_debug_privilege() {
        println!("enable_debug_privilege failed.");
        return false;
    }

    let exe_str: widestring::WideCString = widestring::WideCString::from_str(exe_path.to_str().unwrap()).unwrap();
    let working_directory_str: widestring::WideCString =
        widestring::WideCString::from_str(exe_path.parent().unwrap().to_str().unwrap()).unwrap();
    let working_dir_opt: winapi::um::winnt::LPCWSTR = if working_directory_str.len() > 0 {
        working_directory_str.as_ptr()
    } else {
        std::ptr::null_mut()
    };

    let mut startup_info = winapi::um::processthreadsapi::STARTUPINFOW {
        cb: std::mem::size_of::<winapi::um::processthreadsapi::STARTUPINFOW>() as u32,
        lpReserved: std::ptr::null_mut(),
        lpDesktop: std::ptr::null_mut(),
        lpTitle: std::ptr::null_mut(),
        dwX: 0,
        dwY: 0,
        dwXSize: 0,
        dwYSize: 0,
        dwXCountChars: 0,
        dwYCountChars: 0,
        dwFillAttribute: 0,
        dwFlags: 0,
        wShowWindow: 0,
        cbReserved2: 0,
        lpReserved2: std::ptr::null_mut(),
        hStdInput: std::ptr::null_mut(),
        hStdOutput: std::ptr::null_mut(),
        hStdError: std::ptr::null_mut(),
    };

    let mut process_info = winapi::um::processthreadsapi::PROCESS_INFORMATION {
        hProcess: std::ptr::null_mut(),
        hThread: std::ptr::null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    };

    let command_line = widestring::WideCString::from_str("Game.exe -w").unwrap();

    let success: winapi::shared::minwindef::BOOL = unsafe {
        winapi::um::processthreadsapi::CreateProcessW(
            exe_str.as_ptr(),
            command_line.as_ptr() as _,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            winapi::shared::minwindef::FALSE,
            winapi::um::winbase::NORMAL_PRIORITY_CLASS | winapi::um::winbase::CREATE_SUSPENDED,
            std::ptr::null_mut(),
            working_dir_opt,
            &mut startup_info,
            &mut process_info,
        )
    };

    if success == winapi::shared::minwindef::TRUE {
        *process_id = process_info.dwProcessId;
        let injected = winutils::inject_library(process_info.hProcess, &dll_path);

        unsafe {
            winapi::um::processthreadsapi::ResumeThread(process_info.hThread);
        }

        return injected;
    } else {
        println!("Failed to create the process.");
    }

    *process_id = 0;
    return false;
}

pub fn call_remote_function2(process_id: u32, dll_path: &std::path::PathBuf, func_name: &str) -> bool {
    let process_handle: winapi::um::winnt::HANDLE = rlwindows::open_process(
        process_id,
        winapi::um::winnt::PROCESS_CREATE_THREAD
            | winapi::um::winnt::PROCESS_QUERY_INFORMATION
            | winapi::um::winnt::PROCESS_VM_OPERATION
            | winapi::um::winnt::PROCESS_VM_WRITE
            | winapi::um::winnt::PROCESS_VM_READ,
    );
    if process_handle.is_null() {
        rlwindows::print_get_last_err();
        println!("Process id '{}' does not exist or is not accessible.", process_id);
        return false;
    }

    let remote_module = winutils::find_remote_module_by_path(process_id, dll_path);
    if remote_module.is_null() {
        rlwindows::close_handle(process_handle);
        println!(
            "Could not find module {:?} in process id: {}",
            dll_path.display(),
            process_id
        );
        return false;
    }

    let function_name_cstr: std::ffi::CString = std::ffi::CString::new(func_name).unwrap();

    /*let argument_arg: Vec<std::ffi::OsString> = Vec::new();
    let argument_ptr: winapi::shared::minwindef::LPVOID;
    let argument_size: u64;
    if argument_arg.len() > 0 {
        argument_ptr = argument_arg.as_ptr() as *mut winapi::ctypes::c_void;
        argument_size = ((argument_arg.len() + 1) * std::mem::size_of::<u16>()) as u64;
    } else {
        argument_ptr = std::ptr::null_mut();
        argument_size = 0;
    }*/

    let result = call_remote_function(
        process_handle,
        remote_module,
        &function_name_cstr,
        std::ptr::null_mut(),
        0,
    );

    rlwindows::close_handle(process_handle);
    // TODO: free remote_module?

    result
}

pub fn call_remote_function(
    process_handle: winapi::um::winnt::HANDLE,
    module_handle: winapi::shared::minwindef::HMODULE,
    function_name: &std::ffi::CString,
    argument: winapi::shared::minwindef::LPVOID,
    argument_size: usize,
) -> bool {
    use std::io::Read;

    if process_handle == std::ptr::null_mut() {
        println!("Process does not exist or is not accessible.");
        return false;
    }

    if module_handle == std::ptr::null_mut() {
        println!("Failed to find {:?}.", module_handle);
        return false;
    }

    let process_id: u32 = unsafe { winapi::um::processthreadsapi::GetProcessId(process_handle) };

    let module_path: std::path::PathBuf = winutils::find_remote_module_path_by_handle(process_id, module_handle);
    if !module_path.exists() {
        println!("Could not find the remote module.");
        return false;
    }

    let open_file_result = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .truncate(false)
        .append(false)
        .open(module_path);
    let mut file: std::fs::File;

    match open_file_result {
        Ok(f) => {
            file = f;
        }
        Err(_) => {
            println!("Failed to open the remote module.");
            return false;
        }
    }

    let mut buf = vec![];
    let read_result = file.read_to_end(&mut buf);
    match read_result {
        Ok(_) => {}
        Err(_) => {
            println!("Failed reading the remote module.");
            return false;
        }
    }

    let pe_file_result = pe::Pe::new(&buf);
    let pe_file: pe::Pe;
    match pe_file_result {
        Ok(p) => {
            pe_file = p;
        }
        Err(_) => {
            println!("Failed parsing the remote module header.");
            return false;
        }
    }

    let export_dir_result = pe_file.get_exports();
    let export_dir: pe::Exports;
    match export_dir_result {
        Ok(d) => {
            export_dir = d;
        }
        Err(_) => {
            println!("Failed parsing the remote module export directory.");
            return false;
        }
    }

    let remote_fn = export_dir.lookup_symbol(function_name.to_str().unwrap());
    let remote_fn_rva: pe::ExportAddress;

    match remote_fn {
        Ok(rva) => {
            remote_fn_rva = rva;
        }
        Err(_) => {
            println!("Could not find the remote function {:?}.", function_name);
            return false;
        }
    };

    use pe::ExportAddress as EA;
    let offset = match remote_fn_rva {
        EA::Export(rva) => rva.get(),
        EA::Forwarder(rva) => rva.get(),
    };

    let module_base_addr = winutils::find_remote_module_base_address_by_handle(process_id, module_handle);
    let resolved_fn = (module_base_addr as usize + offset as usize) as *const winapi::ctypes::c_void;

    if resolved_fn == std::ptr::null_mut() {
        println!("Failed to find {:?}.", function_name);
        return false;
    }

    let mut remote_arg: *mut winapi::ctypes::c_void = std::ptr::null_mut();
    if argument_size > 0 {
        remote_arg = rlwindows::virtual_alloc_ex(
            process_handle,
            std::ptr::null_mut(),
            argument_size,
            winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT,
            winapi::um::winnt::PAGE_READWRITE,
        );

        if remote_arg == std::ptr::null_mut() {
            println!("Failed to allocate memory in the target process.");
            return false;
        }

        let mut bytes_written: winapi::shared::basetsd::SIZE_T = 0;
        let bytes_written_ptr: *mut winapi::shared::basetsd::SIZE_T =
            &mut bytes_written as *mut _ as *mut winapi::shared::basetsd::SIZE_T;
        let wpm_ret =
            rlwindows::write_process_memory(process_handle, remote_arg, argument, argument_size, bytes_written_ptr);
        if !wpm_ret || bytes_written < argument_size {
            println!("Failed to write memory to the target process.");
            rlwindows::virtual_free_ex(
                process_handle,
                remote_arg,
                argument_size,
                winapi::um::winnt::MEM_RELEASE,
            );
            return false;
        }
    }

    let mut thread_id: winapi::shared::minwindef::DWORD = 0;
    let thread_id_ptr: *mut winapi::shared::minwindef::DWORD =
        &mut thread_id as *mut _ as *mut winapi::shared::minwindef::DWORD;

    let start_routine = if resolved_fn.is_null() {
        None
    } else {
        unsafe {
            Some(::std::mem::transmute::<
                *const winapi::ctypes::c_void,
                unsafe extern "system" fn(
                    lpParameter: winapi::shared::minwindef::LPVOID,
                ) -> winapi::shared::minwindef::DWORD,
            >(resolved_fn))
        }
    };

    let thread_handle: winapi::um::winnt::HANDLE;
    thread_handle = rlwindows::create_remote_thread(
        process_handle,
        std::ptr::null_mut(),
        0,
        start_routine,
        remote_arg,
        0,
        thread_id_ptr,
    );
    if thread_handle == std::ptr::null_mut() {
        println!("Failed to call the remote function.");
        if argument_size > 0 {
            rlwindows::virtual_free_ex(
                process_handle,
                remote_arg,
                argument_size,
                winapi::um::winnt::MEM_RELEASE,
            );
        }
        return false;
    }

    unsafe {
        winapi::um::synchapi::WaitForSingleObject(thread_handle, winapi::um::winbase::INFINITE);
        winapi::um::handleapi::CloseHandle(thread_handle);
    }

    if argument_size > 0 {
        rlwindows::virtual_free_ex(
            process_handle,
            remote_arg,
            argument_size,
            winapi::um::winnt::MEM_RELEASE,
        );
    }

    return true;
}
