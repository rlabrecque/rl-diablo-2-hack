use super::windows;

pub fn get_process_ids_from_name(process_name: &str) -> Vec<u32> {
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

    let snapshot: winapi::um::winnt::HANDLE = windows::create_tool_help32_snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0);

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
        windows::close_handle(snapshot);
    }

    return process_ids;
}

pub fn is_process_elevated(process_handle: winapi::um::winnt::HANDLE) -> bool {
    let mut is_elevated = false;
    let mut token: winapi::um::winnt::HANDLE = std::ptr::null_mut();

    if windows::open_process_token(process_handle, winapi::um::winnt::TOKEN_QUERY, &mut token) {
        let mut elevation = winapi::um::winnt::TOKEN_ELEVATION::default();
        let size = std::mem::size_of::<winapi::um::winnt::TOKEN_ELEVATION>() as u32;
        let mut ret_size = size;
        if windows::get_token_information(
            token,
            winapi::um::winnt::TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut ret_size,
        ) {
            is_elevated = elevation.TokenIsElevated != 0;
        }
    }

    if !token.is_null() {
        windows::close_handle(token);
    }

    return is_elevated;
}

/*pub fn find_remote_module_by_path(process_id: u32, dll_path: &Path) -> winapi::minwindef::HMODULE {
    let snapshot: winapi::HANDLE;
    let mut module_entry = winapi::tlhelp32::MODULEENTRY32W {
        dwSize: mem::size_of::<winapi::tlhelp32::MODULEENTRY32W>() as u32,
        th32ModuleID: 0,
        th32ProcessID: 0,
        GlblcntUsage: 0,
        ProccntUsage: 0,
        modBaseAddr: null_mut(),
        modBaseSize: 0,
        hModule: null_mut(),
        szModule: [0; winapi::tlhelp32::MAX_MODULE_NAME32 + 1],
        szExePath: [0; winapi::minwindef::MAX_PATH]
    };

    unsafe { snapshot = kernel32::CreateToolhelp32Snapshot(winapi::tlhelp32::TH32CS_SNAPMODULE, process_id); }

    let mut module_handle: winapi::minwindef::HMODULE = null_mut();
    unsafe {
             if kernel32::Module32FirstW(snapshot, &mut module_entry) == winapi::minwindef::TRUE {

                while kernel32::Module32NextW(snapshot, &mut module_entry) == winapi::minwindef::TRUE {
                    let wide_str:OsString = OsStringExt::from_wide(&module_entry.szExePath);
                    let exe_str:WideCString = WideCString::from_str_with_nul(wide_str).unwrap();
                    if exe_str.to_os_string() == dll_path.as_os_str() {
                        module_handle = module_entry.hModule;
                        break;
                    }
                }
	       }
    }

	if snapshot != winapi::INVALID_HANDLE_VALUE {
		unsafe { kernel32::CloseHandle( snapshot ); }
    }

	return module_handle;
}*/