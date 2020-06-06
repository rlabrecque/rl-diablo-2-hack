pub mod lib;
mod options;

use options::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Options::default();

    lib::hello_world(&opt.process_name);

    Ok(())
}
