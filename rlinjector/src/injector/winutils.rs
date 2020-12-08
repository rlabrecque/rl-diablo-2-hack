pub fn get_process_ids_from_name(process_name: &str) -> Vec<u32> {
    use std::os::windows::ffi::OsStringExt;

    let mut process_entry = winapi::um::tlhelp32::PROCESSENTRY32W {
        dwSize: std::mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32W>() as _,
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
        rlwindows::create_tool_help32_snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0);

    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::print_get_last_err();
        println!("create_tool_help32_snapshot failed for process id: 0");
        return vec![];
    }

    let mut process_ids: Vec<u32> = Vec::new();

    let mut result = rlwindows::process32_first(snapshot, &mut process_entry);
    if !result {
        rlwindows::print_get_last_err();
        println!("process32_first failed for process id: 0");
    }

    while result {
        let filename: std::ffi::OsString = std::ffi::OsString::from_wide(&process_entry.szExeFile);
        let filename: &str = filename.to_str().unwrap();
        let filename: widestring::WideCString = widestring::WideCString::from_str_with_nul(filename).unwrap();
        let filename: String = filename.to_string_lossy();

        if filename == *process_name {
            process_ids.push(process_entry.th32ProcessID);
        }

        result = rlwindows::process32_next(snapshot, &mut process_entry)
    }

    if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::close_handle(snapshot);
    }

    return process_ids;
}

pub fn is_process_elevated(process_handle: winapi::um::winnt::HANDLE) -> bool {
    let mut is_elevated = false;
    let mut token_handle: winapi::um::winnt::HANDLE = std::ptr::null_mut();

    if rlwindows::open_process_token(process_handle, winapi::um::winnt::TOKEN_QUERY, &mut token_handle) {
        let mut elevation = winapi::um::winnt::TOKEN_ELEVATION::default();
        let size = std::mem::size_of::<winapi::um::winnt::TOKEN_ELEVATION>() as u32;
        let mut ret_size = size;
        if rlwindows::get_token_information(
            token_handle,
            winapi::um::winnt::TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut ret_size,
        ) {
            is_elevated = elevation.TokenIsElevated != 0;
        }
    }

    if !token_handle.is_null() {
        rlwindows::close_handle(token_handle);
    }

    return is_elevated;
}

pub fn enable_debug_privilege() -> bool {
    let mut token_handle: winapi::um::winnt::HANDLE = std::ptr::null_mut();
    let mut se_debugname_value = winapi::um::winnt::LUID::default();
    let mut token_privileges = winapi::um::winnt::TOKEN_PRIVILEGES::default();

    if rlwindows::open_process_token(
        rlwindows::get_current_process(),
        winapi::um::winnt::TOKEN_ADJUST_PRIVILEGES | winapi::um::winnt::TOKEN_QUERY,
        &mut token_handle,
    ) {
        if rlwindows::lookup_privilege_value(
            std::ptr::null(),
            widestring::WideCString::from_str(winapi::um::winnt::SE_DEBUG_NAME)
                .unwrap()
                .into_raw(),
            &mut se_debugname_value,
        ) {
            token_privileges.PrivilegeCount = 1;
            token_privileges.Privileges[0].Luid = se_debugname_value;
            token_privileges.Privileges[0].Attributes = winapi::um::winnt::SE_PRIVILEGE_ENABLED;

            if rlwindows::adjust_token_privileges(
                token_handle,
                winapi::shared::minwindef::FALSE,
                &mut token_privileges,
                std::mem::size_of::<winapi::um::winnt::TOKEN_PRIVILEGES>() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ) {
                rlwindows::close_handle(token_handle);
                return true;
            }
        }
    }

    rlwindows::close_handle(token_handle);

    return false;
}

pub fn find_remote_module_by_path(process_id: u32, dll_path: &std::path::Path) -> winapi::shared::minwindef::HMODULE {
    use std::os::windows::ffi::OsStringExt;

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
        rlwindows::create_tool_help32_snapshot(winapi::um::tlhelp32::TH32CS_SNAPMODULE, process_id);

    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::print_get_last_err();
        println!("create_tool_help32_snapshot failed for process id: {}", process_id);
        return std::ptr::null_mut();
    }

    let mut module_handle: winapi::shared::minwindef::HMODULE = std::ptr::null_mut();

    println!("Trying to find: {} in process: {}", dll_path.display(), process_id);

    if rlwindows::module32_first(snapshot, &mut module_entry) {
        let filename: std::ffi::OsString = std::ffi::OsString::from_wide(&module_entry.szExePath);
        let filename: &str = filename.to_str().unwrap();
        let filename: widestring::WideCString = widestring::WideCString::from_str_with_nul(filename).unwrap();
        let filename: String = filename.to_string_lossy();
        let filename: std::path::PathBuf = std::path::PathBuf::from(filename);

        while rlwindows::module32_next(snapshot, &mut module_entry) {
            let filename: std::ffi::OsString = std::ffi::OsString::from_wide(&module_entry.szExePath);
            let filename: &str = filename.to_str().unwrap();
            let filename: widestring::WideCString = widestring::WideCString::from_str_with_nul(filename).unwrap();
            let filename: String = filename.to_string_lossy();
            let filename: std::path::PathBuf = std::path::PathBuf::from(filename);

            if filename == *dll_path {
                module_handle = module_entry.hModule;
                break;
            }
        }
    } else {
        rlwindows::print_get_last_err();
        println!("module32_first failed...");
    }

    if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::close_handle(snapshot);
    }

    return module_handle;
}

pub fn inject_library(process_handle: winapi::um::winnt::HANDLE, dll_path: &std::path::Path) -> bool {
    use std::os::windows::ffi::OsStrExt;

    if process_handle == std::ptr::null_mut() {
        println!("Process does not exist or is not accessible.");
        return false;
    }

    let kernel32_str = "Kernel32.dll";
    let kernel32_wide_str = widestring::WideCString::from_str(kernel32_str).unwrap();
    let kernel32_module = rlwindows::get_module_handle(kernel32_wide_str.as_ptr());
    if kernel32_module == std::ptr::null_mut() {
        println!("Failed to find {}.", kernel32_str);
        return false;
    }

    let load_library_str = "LoadLibraryW";
    let load_library_cstring = std::ffi::CString::new(load_library_str).unwrap();
    let load_library_address = rlwindows::get_proc_address(kernel32_module, load_library_cstring.as_ptr());
    if load_library_address == std::ptr::null_mut() {
        println!("Failed to find {}.", load_library_str);
        return false;
    }

    let dll_path_str = dll_path.as_os_str();
    let dll_path_size = dll_path_str.len() * std::mem::size_of::<u16>();
    let remote_string = rlwindows::virtual_alloc_ex(
        process_handle,
        std::ptr::null_mut(),
        winapi::shared::minwindef::MAX_PATH,
        winapi::um::winnt::MEM_RESERVE | winapi::um::winnt::MEM_COMMIT,
        winapi::um::winnt::PAGE_READWRITE,
    );
    if remote_string == std::ptr::null_mut() {
        println!("Failed to allocate memory in the target process.");
        return false;
    }

    let mut bytes_written = winapi::shared::basetsd::SIZE_T::default();
    let bytes_written_ptr = &mut bytes_written;
    let wpm_ret = rlwindows::write_process_memory(
        process_handle,
        remote_string,
        dll_path_str.encode_wide().collect::<Vec<_>>().as_ptr() as *const winapi::ctypes::c_void,
        dll_path_size,
        bytes_written_ptr,
    );
    if !wpm_ret || bytes_written < dll_path_size {
        println!("Failed to write memory to the target process.");
        rlwindows::virtual_free_ex(
            process_handle,
            remote_string,
            dll_path_size,
            winapi::um::winnt::MEM_RELEASE,
        );
        return false;
    }

    let start_routine = unsafe {
        Some(::std::mem::transmute::<
            winapi::shared::minwindef::FARPROC,
            unsafe extern "system" fn(
                lpThreadParameter: winapi::shared::minwindef::LPVOID,
            ) -> winapi::shared::minwindef::DWORD,
        >(load_library_address))
    };

    let mut thread_id = winapi::shared::minwindef::DWORD::default();
    let thread_id_ptr = &mut thread_id;

    let thread_handle = rlwindows::create_remote_thread(
        process_handle,
        std::ptr::null_mut(),
        0,
        start_routine,
        remote_string,
        0,
        thread_id_ptr,
    );
    if thread_handle.is_null() {
        println!("Failed to inject the dll.");
        rlwindows::virtual_free_ex(
            process_handle,
            remote_string,
            dll_path_size,
            winapi::um::winnt::MEM_RELEASE,
        );
        return false;
    }

    println!("Thread id: {}", thread_id);

    rlwindows::wait_for_single_object(thread_handle, winapi::um::winbase::INFINITE);

    rlwindows::virtual_free_ex(process_handle, remote_string, 0, winapi::um::winnt::MEM_RELEASE);

    // TODO: FreeLibrary

    let module = find_module(process_handle, dll_path);
    if module.is_null() {
        println!("Dll did not successfully inject!");
        rlwindows::close_handle(thread_handle);
        rlwindows::virtual_free_ex(
            process_handle,
            remote_string,
            dll_path_size,
            winapi::um::winnt::MEM_RELEASE,
        );
        return false;
    }

    rlwindows::close_handle(thread_handle);
    rlwindows::virtual_free_ex(
        process_handle,
        remote_string,
        dll_path_size,
        winapi::um::winnt::MEM_RELEASE,
    );

    return true;
}

pub fn find_module(
    process_handle: winapi::um::winnt::HANDLE,
    dll_path: &std::path::Path,
) -> winapi::shared::minwindef::HMODULE {
    let sizeof_hmodule = std::mem::size_of::<winapi::shared::minwindef::HMODULE>();

    let mut modules = {
        let mut bytes_needed = winapi::shared::minwindef::DWORD::default();
        let ret = rlwindows::enum_process_modules(process_handle, std::ptr::null_mut(), 0, &mut bytes_needed);
        if !ret {
            rlwindows::print_get_last_err();
            println!("enum_process_modules failed.");
            return std::ptr::null_mut();
        }
        let num_entries_needed = bytes_needed as usize / sizeof_hmodule;
        let mut modules = Vec::<winapi::shared::minwindef::HMODULE>::with_capacity(num_entries_needed);
        modules.resize(num_entries_needed, std::ptr::null_mut());
        modules
    };

    {
        let mut bytes_fetched = winapi::shared::minwindef::DWORD::default();
        let ret = rlwindows::enum_process_modules(
            process_handle,
            modules.as_mut_ptr(),
            (modules.len() * sizeof_hmodule) as _,
            &mut bytes_fetched,
        );
        if !ret {
            rlwindows::print_get_last_err();
            println!("enum_process_modules failed.");
            return std::ptr::null_mut();
        }

        let num_entries_fetched = bytes_fetched as usize / sizeof_hmodule;
        modules.resize(num_entries_fetched, std::ptr::null_mut());
    }

    let name = dll_path.file_name().unwrap().to_string_lossy().to_lowercase();

    const BUF_SIZE: usize = 1024;
    let mut buf = [0u16; BUF_SIZE];
    for module in modules {
        let n = rlwindows::get_module_base_name(process_handle, module, buf.as_mut_ptr(), buf.len() as u32);

        let mut module_name = String::from_utf16_lossy(&buf);
        module_name.truncate(n as usize);

        if module_name.to_lowercase() == name {
            return module;
        }
    }

    return std::ptr::null_mut();
}

pub fn unload_library(
    process_handle: winapi::um::winnt::HANDLE,
    dll_handle: winapi::shared::minwindef::HMODULE,
) -> bool {
    if process_handle == std::ptr::null_mut() {
        println!("Process does not exist or is not accessible.");
        return false;
    }

    // TODO: Leaking kernel32_module
    let kernel32_str = "Kernel32.dll";
    let kernel32_wide_str = widestring::WideCString::from_str(kernel32_str).unwrap();
    let kernel32_module = rlwindows::get_module_handle(kernel32_wide_str.as_ptr());
    if kernel32_module == std::ptr::null_mut() {
        println!("Failed to find {}.", kernel32_str);
        return false;
    }

    // TODO: Leaking free_library_address
    let free_library_str = "FreeLibrary";
    let free_library_cstring = std::ffi::CString::new(free_library_str).unwrap();
    let free_library_address = rlwindows::get_proc_address(kernel32_module, free_library_cstring.as_ptr());
    if free_library_address == std::ptr::null_mut() {
        println!("Failed to find {}.", free_library_str);
        return false;
    }

    let start_routine = unsafe {
        Some(::std::mem::transmute::<
            winapi::shared::minwindef::FARPROC,
            unsafe extern "system" fn(
                lpThreadParameter: winapi::shared::minwindef::LPVOID,
            ) -> winapi::shared::minwindef::DWORD,
        >(free_library_address))
    };

    let mut thread_id: winapi::shared::minwindef::DWORD = 0;
    let thread_id_ptr: *mut winapi::shared::minwindef::DWORD =
        &mut thread_id as *mut _ as *mut winapi::shared::minwindef::DWORD;

    let thread_handle = rlwindows::create_remote_thread(
        process_handle,
        std::ptr::null_mut(),
        0,
        start_routine,
        dll_handle as _,
        0,
        thread_id_ptr,
    );

    if thread_handle == std::ptr::null_mut() {
        println!("Failed to free the module.");
        return false;
    }

    rlwindows::wait_for_single_object(thread_handle, winapi::um::winbase::INFINITE);

    rlwindows::close_handle(thread_handle);

    true
}

pub fn shutdown_library(
    process_handle: winapi::um::winnt::HANDLE,
    dll_handle: winapi::shared::minwindef::HMODULE,
) -> bool {
    if process_handle == std::ptr::null_mut() {
        println!("Process does not exist or is not accessible.");
        return false;
    }

    // TODO: Leaking free_library_address
    let free_library_str = "UnloadModule";
    let free_library_cstring = std::ffi::CString::new(free_library_str).unwrap();
    let free_library_address = rlwindows::get_proc_address(dll_handle, 2 as _);
    if free_library_address == std::ptr::null_mut() {
        println!("Failed to find '{}' in module: {:?}.", free_library_str, dll_handle);
        return false;
    }

    let start_routine = unsafe {
        Some(::std::mem::transmute::<
            winapi::shared::minwindef::FARPROC,
            unsafe extern "system" fn(
                lpThreadParameter: winapi::shared::minwindef::LPVOID,
            ) -> winapi::shared::minwindef::DWORD,
        >(free_library_address))
    };

    let mut thread_id: winapi::shared::minwindef::DWORD = 0;
    let thread_id_ptr: *mut winapi::shared::minwindef::DWORD =
        &mut thread_id as *mut _ as *mut winapi::shared::minwindef::DWORD;

    let thread_handle = rlwindows::create_remote_thread(
        process_handle,
        std::ptr::null_mut(),
        0,
        start_routine,
        dll_handle as _,
        0,
        thread_id_ptr,
    );

    if thread_handle == std::ptr::null_mut() {
        println!("Failed to free the module.");
        return false;
    }

    rlwindows::wait_for_single_object(thread_handle, winapi::um::winbase::INFINITE);

    rlwindows::close_handle(thread_handle);

    true
}

pub fn find_remote_module_base_address_by_handle(
    process_id: u32,
    module_handle: winapi::shared::minwindef::HMODULE,
) -> *mut winapi::shared::minwindef::BYTE {
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
        rlwindows::create_tool_help32_snapshot(winapi::um::tlhelp32::TH32CS_SNAPMODULE, process_id);
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::print_get_last_err();
        println!("create_tool_help32_snapshot failed for process id: {}", process_id);
        return std::ptr::null_mut();
    }

    let mut base_address: *mut winapi::shared::minwindef::BYTE = std::ptr::null_mut();
    if rlwindows::module32_first(snapshot, &mut module_entry) {
        while rlwindows::module32_next(snapshot, &mut module_entry) {
            if module_entry.hModule == module_handle {
                base_address = module_entry.modBaseAddr;
                break;
            }
        }
    } else {
        rlwindows::print_get_last_err();
        println!("module32_first failed...");
    }

    if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::close_handle(snapshot);
    }

    return base_address;
}

pub fn find_remote_module_path_by_handle(
    process_id: u32,
    module_handle: winapi::shared::minwindef::HMODULE,
) -> std::path::PathBuf {
    use std::os::windows::ffi::OsStringExt;

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
        rlwindows::create_tool_help32_snapshot(winapi::um::tlhelp32::TH32CS_SNAPMODULE, process_id);

    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::print_get_last_err();
        println!("create_tool_help32_snapshot failed for process id: {}", process_id);
        return std::path::PathBuf::new();
    }

    let mut module_path: std::path::PathBuf = std::path::PathBuf::new();

    if rlwindows::module32_first(snapshot, &mut module_entry) {
        while rlwindows::module32_next(snapshot, &mut module_entry) {
            if module_entry.hModule == module_handle {
                let filename: std::ffi::OsString = std::ffi::OsString::from_wide(&module_entry.szExePath);
                let filename: &str = filename.to_str().unwrap();
                let filename: widestring::WideCString = widestring::WideCString::from_str_with_nul(filename).unwrap();
                let filename: String = filename.to_string_lossy();

                module_path = std::path::PathBuf::from(filename);
                break;
            }
        }
    } else {
        rlwindows::print_get_last_err();
        println!("module32_first failed...");
    }

    if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
        rlwindows::close_handle(snapshot);
    }

    return module_path;
}
