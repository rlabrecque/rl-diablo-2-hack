use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rlinjector", about = "Extracts strings from files.")]
pub struct Options {
    ///
    #[structopt(short)]
    pub process_name: String,

    ///
    #[structopt(parse(from_os_str))]
    pub dll_path: std::path::PathBuf,
}

impl Default for Options {
    fn default() -> Self {
        Options::from_args()
    }
}
