use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    Inject {
        #[structopt(parse(from_os_str))]
        dll_path: std::path::PathBuf,

        ///
        #[structopt(short)]
        process_name: String,
    },
    /*Create {
        ///
        #[structopt(parse(from_os_str))]
        dll_path: std::path::PathBuf,

        ///
        #[structopt(short)]
        process_path: std::path::PathBuf,
    },*/
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rlinjector", about = "Extracts strings from files.")]
pub struct Options {
    ///
    #[structopt(subcommand)]
    pub command: Command,
}

impl Default for Options {
    fn default() -> Self {
        Options::from_args()
    }
}
