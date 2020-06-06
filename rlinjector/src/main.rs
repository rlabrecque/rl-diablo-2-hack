mod lib;
mod options;

use options::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Options::default();

    if !opt.dll_path.exists() {
        let err_msg = format!("DLL file specified does not exist: {:?}", opt.dll_path);
        return Err(err_msg.into());
    }

    lib::hello_world(&opt.process_name);

    Ok(())
}
