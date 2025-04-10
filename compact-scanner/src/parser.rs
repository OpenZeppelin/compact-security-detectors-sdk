use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(short = 'm', long = "metadata", action = clap::ArgAction::SetTrue)]
    pub(crate) metadata: bool,
    #[clap(long = "code", required = false, value_parser, num_args = 1..)]
    pub(crate) code: Option<Vec<std::path::PathBuf>>,
    #[clap(long = "detectors", required = false, value_parser, num_args = 1..)]
    pub(crate) detectors: Option<Vec<String>>,
    #[clap(long = "project-root", required = false, value_parser)]
    pub(crate) project_root: Option<std::path::PathBuf>,
    #[clap(short = 'r', long = "recursive", action = clap::ArgAction::SetTrue)]
    pub(crate) recursive: bool,
}
