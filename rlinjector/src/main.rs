mod injector;
mod options;

use options::{Command, Options};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Options::default();

    injector::enable_debug_privilege()?;

    match opt.command {
        Command::Inject { dll_path, process_name } => inject(dll_path, process_name)?,
        //Command::Create { dll_path, process_path } => create(dll_path, process_path)?,
    }

    Ok(())
}

fn inject(dll_path: std::path::PathBuf, process_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let abs_dll_path = std::fs::canonicalize(dll_path)?;

    let injected_process_ids = injector::inject_dll_into_process_name(&abs_dll_path, &process_name)?;

    let is_any_process_injected = |abs_dll_path: &std::path::PathBuf, injected_process_ids: &Vec<u32>| -> bool {
        for process_id in injected_process_ids {
            let result = injector::is_dll_loaded_in_pid(&abs_dll_path, *process_id);
            match result {
                Ok(loaded) => {
                    if loaded {
                        return true;
                    }
                }
                Err(_) => {
                    // TODO: Remove from list of injectedprocess ids'
                }
            }
        }

        false
    };

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        if !is_any_process_injected(&abs_dll_path, &injected_process_ids) {
            break;
        }

        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            println!("Shutting down, unhooking from processes.");
            for process_id in &injected_process_ids {
                injector::call_remote_function2(*process_id, &abs_dll_path, "UnloadModule");
            }

            break;
        }
    }

    Ok(())
}

// TODO:
/*
fn create(dll_path: std::path::PathBuf, process_path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let abs_dll_path = std::fs::canonicalize(dll_path)?;

    let mut create_proc_id: u32 = 0;
    let success = injector::create_process_and_inject_library(&process_path, &abs_dll_path, &mut create_proc_id);

    println!("{} - Created Process: {}", success, create_proc_id);

    std::thread::sleep(std::time::Duration::from_secs(300));

    //injector::call_remote_function2(create_proc_id, &abs_dll_path);

    Ok(())
}*/
