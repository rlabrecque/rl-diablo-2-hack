pub fn get_module_base_name(
    process_handle: winapi::um::winnt::HANDLE,
    module: winapi::shared::minwindef::HMODULE,
    base_name: winapi::shared::ntdef::LPWSTR,
    size: winapi::shared::minwindef::DWORD,
) -> winapi::shared::minwindef::DWORD {
    unsafe { winapi::um::psapi::GetModuleBaseNameW(process_handle, module, base_name, size) }
}

pub fn enum_process_modules(
    process_handle: winapi::um::winnt::HANDLE,
    out_module_handles: *mut winapi::shared::minwindef::HMODULE,
    out_module_handles_size: winapi::shared::minwindef::DWORD,
    bytes_needed: winapi::shared::minwindef::LPDWORD,
) -> bool {
    unsafe {
        let ret = winapi::um::psapi::EnumProcessModules(
            process_handle,
            out_module_handles,
            out_module_handles_size,
            bytes_needed,
        );
        ret == winapi::shared::minwindef::TRUE
    }
}

pub fn wait_for_single_object(
    handle: winapi::um::winnt::HANDLE,
    milliseconds: winapi::shared::minwindef::DWORD,
) -> winapi::shared::minwindef::DWORD {
    unsafe { winapi::um::synchapi::WaitForSingleObject(handle, milliseconds) }
}

pub fn create_remote_thread(
    process_handle: winapi::um::winnt::HANDLE,
    thread_attributes: winapi::um::minwinbase::LPSECURITY_ATTRIBUTES,
    stack_size: winapi::shared::basetsd::SIZE_T,
    start_address: winapi::um::minwinbase::LPTHREAD_START_ROUTINE,
    parameter: winapi::shared::minwindef::LPVOID,
    creation_flags: winapi::shared::minwindef::DWORD,
    thread_id: winapi::shared::minwindef::LPDWORD,
) -> winapi::um::winnt::HANDLE {
    unsafe {
        winapi::um::processthreadsapi::CreateRemoteThread(
            process_handle,
            thread_attributes,
            stack_size,
            start_address,
            parameter,
            creation_flags,
            thread_id,
        )
    }
}

pub fn write_process_memory(
    process_handle: winapi::um::winnt::HANDLE,
    base_address: winapi::shared::minwindef::LPVOID,
    buffer: winapi::shared::minwindef::LPCVOID,
    size: winapi::shared::basetsd::SIZE_T,
    number_of_bytes_written: *mut winapi::shared::basetsd::SIZE_T,
) -> bool {
    unsafe {
        let ret = winapi::um::memoryapi::WriteProcessMemory(
            process_handle,
            base_address,
            buffer,
            size,
            number_of_bytes_written,
        );
        ret == winapi::shared::minwindef::TRUE
    }
}

pub fn virtual_alloc_ex(
    process_handle: winapi::um::winnt::HANDLE,
    address: winapi::shared::minwindef::LPVOID,
    size: winapi::shared::basetsd::SIZE_T,
    allocation_type: winapi::shared::minwindef::DWORD,
    protect_flags: winapi::shared::minwindef::DWORD,
) -> winapi::shared::minwindef::LPVOID {
    unsafe { winapi::um::memoryapi::VirtualAllocEx(process_handle, address, size, allocation_type, protect_flags) }
}

pub fn virtual_free_ex(
    process_handle: winapi::um::winnt::HANDLE,
    address: winapi::shared::minwindef::LPVOID,
    size: winapi::shared::basetsd::SIZE_T,
    free_type: winapi::shared::minwindef::DWORD,
) -> bool {
    unsafe {
        let ret = winapi::um::memoryapi::VirtualFreeEx(process_handle, address, size, free_type);
        ret == winapi::shared::minwindef::TRUE
    }
}

pub fn get_proc_address(
    module: winapi::shared::minwindef::HMODULE,
    proc_name: winapi::shared::ntdef::LPCSTR,
) -> winapi::shared::minwindef::FARPROC {
    unsafe { winapi::um::libloaderapi::GetProcAddress(module, proc_name) }
}

pub fn get_module_handle(module_name: winapi::shared::ntdef::LPCWSTR) -> winapi::shared::minwindef::HMODULE {
    unsafe { winapi::um::libloaderapi::GetModuleHandleW(module_name) }
}

pub fn module32_first(
    snapshot: winapi::um::winnt::HANDLE,
    module_entry: winapi::um::tlhelp32::LPMODULEENTRY32W,
) -> bool {
    unsafe {
        let ret = winapi::um::tlhelp32::Module32FirstW(snapshot, module_entry);
        ret == winapi::shared::minwindef::TRUE
    }
}

pub fn module32_next(
    snapshot: winapi::um::winnt::HANDLE,
    module_entry: winapi::um::tlhelp32::LPMODULEENTRY32W,
) -> bool {
    unsafe {
        let ret = winapi::um::tlhelp32::Module32NextW(snapshot, module_entry);
        ret == winapi::shared::minwindef::TRUE
    }
}
pub fn process32_first(
    snapshot: winapi::um::winnt::HANDLE,
    process_entry: winapi::um::tlhelp32::LPPROCESSENTRY32W,
) -> bool {
    unsafe {
        let ret = winapi::um::tlhelp32::Process32FirstW(snapshot, process_entry);
        ret == winapi::shared::minwindef::TRUE
    }
}

pub fn process32_next(
    snapshot: winapi::um::winnt::HANDLE,
    process_entry: winapi::um::tlhelp32::LPPROCESSENTRY32W,
) -> bool {
    unsafe {
        let ret = winapi::um::tlhelp32::Process32NextW(snapshot, process_entry);
        ret == winapi::shared::minwindef::TRUE
    }
}

pub fn create_tool_help32_snapshot(
    flags: winapi::shared::minwindef::DWORD,
    process_id: winapi::shared::minwindef::DWORD,
) -> winapi::um::winnt::HANDLE {
    unsafe { winapi::um::tlhelp32::CreateToolhelp32Snapshot(flags, process_id) }
}

pub fn get_token_information(
    token_handle: winapi::um::winnt::HANDLE,
    token_information_class: winapi::um::winnt::TOKEN_INFORMATION_CLASS,
    token_information: winapi::shared::minwindef::LPVOID,
    token_information_length: winapi::shared::minwindef::DWORD,
    return_length: winapi::shared::minwindef::PDWORD,
) -> bool {
    unsafe {
        let ret = winapi::um::securitybaseapi::GetTokenInformation(
            token_handle,
            token_information_class,
            token_information,
            token_information_length,
            return_length,
        );
        return ret == winapi::shared::minwindef::TRUE;
    }
}

pub fn print_get_last_err() {
    unsafe {
        let err_code = winapi::um::errhandlingapi::GetLastError();
        println!("GetLastError: {}", err_code);
    }
}

pub fn close_handle(handle: winapi::um::winnt::HANDLE) -> bool {
    assert_ne!(handle, winapi::um::handleapi::INVALID_HANDLE_VALUE);

    unsafe {
        let ret = winapi::um::handleapi::CloseHandle(handle);
        return ret == winapi::shared::minwindef::TRUE;
    }
}

pub fn open_process(process_id: u32, desired_access: winapi::shared::minwindef::DWORD) -> winapi::um::winnt::HANDLE {
    unsafe { winapi::um::processthreadsapi::OpenProcess(desired_access, winapi::shared::minwindef::FALSE, process_id) }
}

pub fn open_process_token(
    process_handle: winapi::um::winnt::HANDLE,
    desired_access: winapi::shared::minwindef::DWORD,
    token_handle: winapi::um::winnt::PHANDLE,
) -> bool {
    unsafe {
        let ret = winapi::um::processthreadsapi::OpenProcessToken(process_handle, desired_access, token_handle);
        return ret == winapi::shared::minwindef::TRUE;
    }
}

pub fn get_current_process() -> winapi::um::winnt::HANDLE {
    unsafe { winapi::um::processthreadsapi::GetCurrentProcess() }
}

pub fn disable_thread_library_calls(lib_module: winapi::shared::minwindef::HMODULE) -> bool {
    unsafe {
        let ret = winapi::um::libloaderapi::DisableThreadLibraryCalls(lib_module);
        ret == winapi::shared::minwindef::TRUE
    }
}

pub fn create_thread(
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
