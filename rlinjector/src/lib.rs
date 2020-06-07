use winapi;

pub fn inject(dll_path: &std::path::PathBuf, process_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inject {:#?} {}", dll_path, process_name);

    if !dll_path.exists() {
        let err_msg = format!("DLL file specified does not exist: {:#?}", dll_path);
        return Err(err_msg.into());
    }

    let mut process_ids: Vec<u32> = get_process_ids_from_name(&process_name);
    if process_ids.is_empty() {
        let err_msg = format!("Process  '{}' does not exist.", process_name);
        return Err(err_msg.into());
    }

    for pid in &process_ids {
        /*let process_handle: winapi::HANDLE = open_process(
            *p,
            winapi::winnt::PROCESS_CREATE_THREAD
                | winapi::winnt::PROCESS_QUERY_INFORMATION
                | winapi::winnt::PROCESS_VM_OPERATION
                | winapi::winnt::PROCESS_VM_WRITE
                | winapi::winnt::PROCESS_VM_READ,
        );

        if process_handle == null_mut() {
            println!("Process with id {:?} does not exist or is not accessible.", p);
            continue;
        }

        let remote_module: winapi::minwindef::HMODULE = find_remote_module_by_path(*p, dll_path_real);
        if remote_module != null_mut() {
            println!("DLL already exists in process. HMODULE: {:?}.", remote_module);
            println!("Injection failed.");
        } else {
            if inject_library(process_handle, &dll_path_real) {
                println!("Successfully injected {:?} into {:?}.", dll_path, p);
            } else {
                println!("Injection failed.");
            }
        }

        if process_handle != null_mut() {
            unsafe {
                kernel32::CloseHandle(process_handle);
            }
        }*/
        println!("{}", pid);
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
        unsafe {
            winapi::um::handleapi::CloseHandle(snapshot);
        }
    }

    return process_ids;
}

/*fn open_process(process_id: u32, desired_access: winapi::minwindef::DWORD) -> winapi::HANDLE {
    let process_handle: winapi::HANDLE;
    unsafe {
        process_handle = kernel32::OpenProcess(desired_access, winapi::minwindef::FALSE, process_id);
    }

    return process_handle;
}*/
