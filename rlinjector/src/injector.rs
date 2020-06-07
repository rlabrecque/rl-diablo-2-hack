mod windows;
mod winutils;

#[cfg(target_arch = "x86")]
pub fn inject(dll_path: &std::path::PathBuf, process_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inject {:#?} {}", dll_path, process_name);
    println!(
        "Are we elevated: {}",
        winutils::is_process_elevated(windows::get_current_process())
    );

    if !dll_path.exists() {
        let err_msg = format!("DLL file specified does not exist: {:#?}", dll_path);
        return Err(err_msg.into());
    }

    let process_ids: Vec<u32> = winutils::get_process_ids_from_name(&process_name);
    if process_ids.is_empty() {
        let err_msg = format!("Process '{}' does not exist.", process_name);
        return Err(err_msg.into());
    }

    for pid in &process_ids {
        println!("{}", pid);

        let process_handle: winapi::um::winnt::HANDLE = windows::open_process(
            *pid,
            winapi::um::winnt::PROCESS_CREATE_THREAD
                | winapi::um::winnt::PROCESS_QUERY_INFORMATION
                | winapi::um::winnt::PROCESS_VM_OPERATION
                | winapi::um::winnt::PROCESS_VM_WRITE
                | winapi::um::winnt::PROCESS_VM_READ,
        );

        if process_handle == std::ptr::null_mut() {
            windows::print_get_last_err();
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
            windows::close_handle(process_handle);
        }
    }

    Ok(())
}
