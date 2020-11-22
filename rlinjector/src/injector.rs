mod winutils;

#[cfg(target_arch = "x86")]
pub fn inject(dll_path: &std::path::PathBuf, process_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inject {} {}", dll_path.display(), process_name);
    println!(
        "Are we elevated: {}",
        winutils::is_process_elevated(rlwindows::get_current_process())
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

    for process_id in &process_ids {
        //println!("{}", process_id);

        let process_handle: winapi::um::winnt::HANDLE = rlwindows::open_process(
            *process_id,
            winapi::um::winnt::PROCESS_CREATE_THREAD
                | winapi::um::winnt::PROCESS_QUERY_INFORMATION
                | winapi::um::winnt::PROCESS_VM_OPERATION
                | winapi::um::winnt::PROCESS_VM_WRITE
                | winapi::um::winnt::PROCESS_VM_READ,
        );

        if process_handle == std::ptr::null_mut() {
            rlwindows::print_get_last_err();
            println!("Process with id {:?} does not exist or is not accessible.", process_id);
            continue;
        }

        println!("Process {} successfully opened.", process_id);

        let remote_module: winapi::shared::minwindef::HMODULE =
            winutils::find_remote_module_by_path(*process_id, dll_path);
        if remote_module != std::ptr::null_mut() {
            println!("DLL already exists in process. HMODULE: {:?}.", remote_module);
            println!("Injection failed.");
        } else {
            if winutils::inject_library(process_handle, &dll_path) {
                println!("Successfully injected {:?} into {:?}.", dll_path, process_id);
            } else {
                println!("Injection failed.");
            }
        }

        println!("Closing process {}", process_id);
        if process_handle != std::ptr::null_mut() {
            rlwindows::close_handle(process_handle);
        }
    }

    Ok(())
}
