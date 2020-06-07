use winapi;

#[cfg(target_arch = "x86")]
pub fn inject(dll_path: &std::path::PathBuf, process_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inject {:#?} {}", dll_path, process_name);
    println!("Are we elevated: {}", is_process_elevated(get_current_process()));

    if !dll_path.exists() {
        let err_msg = format!("DLL file specified does not exist: {:#?}", dll_path);
        return Err(err_msg.into());
    }

    let process_ids: Vec<u32> = get_process_ids_from_name(&process_name);
    if process_ids.is_empty() {
        let err_msg = format!("Process '{}' does not exist.", process_name);
        return Err(err_msg.into());
    }

    for pid in &process_ids {
        println!("{}", pid);

        let process_handle: winapi::um::winnt::HANDLE = open_process(
            *pid,
            winapi::um::winnt::PROCESS_CREATE_THREAD
                | winapi::um::winnt::PROCESS_QUERY_INFORMATION
                | winapi::um::winnt::PROCESS_VM_OPERATION
                | winapi::um::winnt::PROCESS_VM_WRITE
                | winapi::um::winnt::PROCESS_VM_READ,
        );

        if process_handle == std::ptr::null_mut() {
            print_get_last_err();
            println!("Process with id {:?} does not exist or is not accessible.", pid);
            continue;
        }

        println!("Process {} successfully opened.", pid);

        /*let remote_module: winapi::minwindef::HMODULE = find_remote_module_by_path(*p, dll_path_real);
        if remote_module != null_mut() {
            println!("DLL already exists in process. HMODULE: {:?}.", remote_module);
            println!("Injection failed.");
        } else {
            if inject_library(process_handle, &dll_path_real) {
                println!("Successfully injected {:?} into {:?}.", dll_path, p);
            } else {
                println!("Injection failed.");
            }
        }*/

        println!("Closing process {}", pid);
        if process_handle != std::ptr::null_mut() {
            close_handle(process_handle);
        }
    }

    Ok(())
}

fn get_process_ids_from_name(process_name: &str) -> Vec<u32> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use widestring::WideCString;

    let mut process_entry = winapi::um::tlhelp32::PROCESSENTRY32W {
        dwSize: std::mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32W>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; winapi::shared::minwindef::MAX_PATH],
    };

    let snapshot: winapi::um::winnt::HANDLE;
    unsafe {
        snapshot = winapi::um::tlhelp32::CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0);
    }

    let mut process_ids: Vec<u32> = Vec::new();

    unsafe {
        if winapi::um::tlhelp32::Process32FirstW(snapshot, &mut process_entry) == winapi::shared::minwindef::TRUE {
            // TODO: Bug where we skip this first process??
            while winapi::um::tlhelp32::Process32NextW(snapshot, &mut process_entry) == winapi::shared::minwindef::TRUE
            {
                let filename: OsString = OsString::from_wide(&process_entry.szExeFile);
                let filename: &str = filename.to_str().unwrap();
                let filename: WideCString = WideCString::from_str_with_nul(filename).unwrap();
                let filename: String = filename.to_string_lossy();
                println!("Process name: {:#?}", filename);
                if filename == *process_name {
                    process_ids.push(process_entry.th32ProcessID);
                }
            }
        }
    }

    if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
        close_handle(snapshot);
    }

    return process_ids;
}

fn open_process(process_id: u32, desired_access: winapi::shared::minwindef::DWORD) -> winapi::um::winnt::HANDLE {
    unsafe {
        winapi::um::processthreadsapi::OpenProcess(desired_access, winapi::shared::minwindef::FALSE, process_id)
    }
}

fn open_process_token(process_handle: winapi::um::winnt::HANDLE, desired_access: winapi::shared::minwindef::DWORD, token_handle: winapi::um::winnt::PHANDLE) -> bool {
    unsafe {
        let ret = winapi::um::processthreadsapi::OpenProcessToken(process_handle, desired_access, token_handle);
        return ret == winapi::shared::minwindef::TRUE;
    }
}

fn get_current_process() -> winapi::um::winnt::HANDLE {
    unsafe {
        winapi::um::processthreadsapi::GetCurrentProcess()
    }
}

fn close_handle(handle: winapi::um::winnt::HANDLE) -> bool {
    assert_ne!(handle, winapi::um::handleapi::INVALID_HANDLE_VALUE);

    unsafe {
        let ret = winapi::um::handleapi::CloseHandle(handle);
        return ret == winapi::shared::minwindef::TRUE;
    }
}

fn get_token_information(token_handle: winapi::um::winnt::HANDLE,
    token_information_class: winapi::um::winnt::TOKEN_INFORMATION_CLASS,
    token_information: winapi::shared::minwindef::LPVOID,
    token_information_length: winapi::shared::minwindef::DWORD,
    return_length: winapi::shared::minwindef::PDWORD) -> bool {
    unsafe {
        let ret = winapi::um::securitybaseapi::GetTokenInformation(token_handle,token_information_class, token_information, token_information_length, return_length);
        return ret == winapi::shared::minwindef::TRUE;
    }
}

fn print_get_last_err() {
    unsafe {
        let err_code = winapi::um::errhandlingapi::GetLastError();
        println!("GetLastError: {}", err_code);
    }
}

fn is_process_elevated(process_handle: winapi::um::winnt::HANDLE) -> bool {
    let mut is_elevated = false;
    let mut token: winapi::um::winnt::HANDLE = std::ptr::null_mut();

    if open_process_token(process_handle, winapi::um::winnt::TOKEN_QUERY, &mut token) {
        let mut elevation = winapi::um::winnt::TOKEN_ELEVATION::default();
        let size = std::mem::size_of::<winapi::um::winnt::TOKEN_ELEVATION>() as u32;
        let mut ret_size = size;
        if get_token_information(token, winapi::um::winnt::TokenElevation, &mut elevation as *mut _ as *mut _, size, &mut ret_size) {
            is_elevated = elevation.TokenIsElevated != 0;
        }
    }

    if !token.is_null() {
        close_handle(token);
    }

    return is_elevated;
}