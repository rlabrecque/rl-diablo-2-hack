mod lib;
mod options;

use options::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Options::default();

    lib::inject(&opt.dll_path, &opt.process_name)
}
