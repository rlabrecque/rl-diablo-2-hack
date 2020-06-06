use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rlinjector", about = "Extracts strings from files.")]
pub struct Options {
    ///
    #[structopt(parse(from_os_str))]
    pub dll_path: std::path::PathBuf,

    ///
    #[structopt(short)]
    pub process_name: String,
}

impl Default for Options {
    fn default() -> Self {
        Options::from_args()
    }
}
