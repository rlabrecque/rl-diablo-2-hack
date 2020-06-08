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

    let snapshot: winapi::um::winnt::HANDLE =
        windows::create_tool_help32_snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0);

    let mut process_ids: Vec<u32> = Vec::new();

    if windows::process32_first(snapshot, &mut process_entry) {
        // TODO: Bug where we skip this first process??
        while windows::process32_next(snapshot, &mut process_entry) {
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

pub fn find_remote_module_by_path(process_id: u32, dll_path: &std::path::Path) -> winapi::shared::minwindef::HMODULE {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use widestring::WideCString;

    let mut module_entry = winapi::um::tlhelp32::MODULEENTRY32W {
        dwSize: std::mem::size_of::<winapi::um::tlhelp32::MODULEENTRY32W>() as u32,
        th32ModuleID: 0,
        th32ProcessID: 0,
        GlblcntUsage: 0,
        ProccntUsage: 0,
        modBaseAddr: std::ptr::null_mut(),
        modBaseSize: 0,
        hModule: std::ptr::null_mut(),
        szModule: [0; winapi::um::tlhelp32::MAX_MODULE_NAME32 + 1],
        szExePath: [0; winapi::shared::minwindef::MAX_PATH],
    };

    let snapshot: winapi::um::winnt::HANDLE =
        windows::create_tool_help32_snapshot(winapi::um::tlhelp32::TH32CS_SNAPMODULE, process_id);

    let mut module_handle: winapi::shared::minwindef::HMODULE = std::ptr::null_mut();
    if windows::module32_first(snapshot, &mut module_entry) {
        // TODO: Bug where we skip this first module??
        while windows::module32_next(snapshot, &mut module_entry) {
            let filename: OsString = OsString::from_wide(&module_entry.szExePath);
            let filename: &str = filename.to_str().unwrap();
            let filename: WideCString = WideCString::from_str_with_nul(filename).unwrap();
            let filename: String = filename.to_string_lossy();
            let filename: std::path::PathBuf = std::path::PathBuf::from(filename);

            println!("Module name: {:#?}", filename);
            if filename == *dll_path {
                module_handle = module_entry.hModule;
                break;
            }
        }
    }

    if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
        windows::close_handle(snapshot);
    }

    return module_handle;
}

pub fn inject_library(process_handle: winapi::um::winnt::HANDLE, dll_path: &std::path::Path) -> bool {
    use std::ffi::CString;
    use std::os::windows::ffi::OsStrExt;
    use widestring::WideCString;

    if process_handle == std::ptr::null_mut() {
        println!("Process does not exist or is not accessible.");
        return false;
    }

    let kernel32_str = "Kernel32.dll";
    let kernel32_wide_str = WideCString::from_str(kernel32_str).unwrap();
    let kernel32_module = windows::get_module_handle(kernel32_wide_str.as_ptr());
    if kernel32_module == std::ptr::null_mut() {
        println!("Failed to find {}.", kernel32_str);
        return false;
    }

    let load_library_str = "LoadLibraryW";
    let load_library_cstring = CString::new(load_library_str).unwrap();
    let load_library_address = windows::get_proc_address(kernel32_module, load_library_cstring.as_ptr());
    if load_library_address == std::ptr::null_mut() {
        println!("Failed to find {}.", load_library_str);
        return false;
    }

    let dll_path_str = dll_path.as_os_str();
    let dll_path_size = (dll_path_str.len() + 1) * std::mem::size_of::<u16>();
    let remote_string = windows::virtual_alloc_ex(
        process_handle,
        std::ptr::null_mut(),
        dll_path_size,
        winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT,
        winapi::um::winnt::PAGE_READWRITE,
    );
    if remote_string == std::ptr::null_mut() {
        println!("Failed to allocate memory in the target process.");
        return false;
    }

    let mut bytes_written: winapi::shared::basetsd::SIZE_T = 0;
    let bytes_written_ptr: *mut winapi::shared::basetsd::SIZE_T =
        &mut bytes_written as *mut _ as *mut winapi::shared::basetsd::SIZE_T;
    let wpm_ret = windows::write_process_memory(
        process_handle,
        remote_string,
        dll_path_str.encode_wide().collect::<Vec<_>>().as_ptr() as *const winapi::ctypes::c_void,
        dll_path_size,
        bytes_written_ptr,
    );
    if !wpm_ret || bytes_written < dll_path_size {
        println!("Failed to write memory to the target process.");
        windows::virtual_free_ex(
            process_handle,
            remote_string,
            dll_path_size,
            winapi::um::winnt::MEM_RELEASE,
        );
        return false;
    }

    let start_routine =
        unsafe {
            Some(::std::mem::transmute::<
                winapi::shared::minwindef::FARPROC,
                unsafe extern "system" fn(
                    lpThreadParameter: winapi::shared::minwindef::LPVOID,
                ) -> winapi::shared::minwindef::DWORD,
            >(load_library_address))
        };

    let mut thread_id: winapi::shared::minwindef::DWORD = 0;
    let thread_id_ptr: *mut winapi::shared::minwindef::DWORD =
        &mut thread_id as *mut _ as *mut winapi::shared::minwindef::DWORD;

    let thread_handle = windows::create_remote_thread(
        process_handle,
        std::ptr::null_mut(),
        0,
        start_routine,
        remote_string,
        0,
        thread_id_ptr,
    );
    if thread_handle == std::ptr::null_mut() {
        println!("Failed to inject the dll.");
        windows::virtual_free_ex(
            process_handle,
            remote_string,
            dll_path_size,
            winapi::um::winnt::MEM_RELEASE,
        );
        return false;
    }

    println!("Thread id: {}", thread_id);

    windows::wait_for_single_object(thread_handle, winapi::um::winbase::INFINITE);

    windows::virtual_free_ex(process_handle, remote_string, 0, winapi::um::winnt::MEM_RELEASE);

    // TODO: FreeLibrary

    let module = find_module(process_handle, dll_path);
    if module.is_null() {
        println!("Dll did not successfully inject!");
    }

    windows::close_handle(thread_handle);
    windows::close_handle(process_handle);
    /*windows::virtual_free_ex(
        process_handle,
        remote_string,
        dll_path_size,
        winapi::um::winnt::MEM_RELEASE,
    );*/



    return true;
}


fn find_module(process_handle: winapi::um::winnt::HANDLE, dll_path: &std::path::Path) -> winapi::shared::minwindef::HMODULE {
    let sizeof_hmodule = std::mem::size_of::<winapi::shared::minwindef::HMODULE>();

    let mut modules = {
        let mut bytes_needed: winapi::shared::minwindef::DWORD = 0;
        windows::enum_process_modules(process_handle, std::ptr::null_mut(), 0, &mut bytes_needed);
        let num_entries_needed = bytes_needed as usize / sizeof_hmodule;
        let mut modules = Vec::<winapi::shared::minwindef::HMODULE>::with_capacity(num_entries_needed);
        modules.resize(num_entries_needed, std::ptr::null_mut());
        modules
    };

    {
        let mut bytes_fetched: winapi::shared::minwindef::DWORD = 0;
        let ret = windows::enum_process_modules(process_handle, modules.as_mut_ptr(), (modules.len() * sizeof_hmodule) as u32, &mut bytes_fetched);
        assert!(ret, "EnumProcessModules");

        let num_entries_fetched = bytes_fetched as usize / sizeof_hmodule;
        modules.resize(num_entries_fetched, std::ptr::null_mut());
    }

    let name = dll_path.file_name().unwrap().to_string_lossy().to_lowercase();

    for module in modules {
        const BUF_SIZE: usize = 1024;
        let mut buf = [0u16; BUF_SIZE];

        let n = windows::get_module_base_name(process_handle, module, buf.as_mut_ptr(), BUF_SIZE as u32);

        let mut module_name = String::from_utf16_lossy(&buf);
        module_name.truncate(n as usize);

        println!("Module Name: {}", module_name);

        if module_name.to_lowercase() == name {
            return module;
        }
    }

	return std::ptr::null_mut();
}