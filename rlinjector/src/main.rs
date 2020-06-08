mod injector;
mod options;

use options::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Options::default();

    let abs_dll_path = std::fs::canonicalize(opt.dll_path)?;

    injector::inject(&abs_dll_path, &opt.process_name)
}
