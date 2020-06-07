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
