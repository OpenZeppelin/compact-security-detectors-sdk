use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(short = 'm', long = "metadata", action = clap::ArgAction::SetTrue)]
    pub(crate) metadata: bool,
    #[clap(long = "scan", required = false, value_parser)]
    pub(crate) paths: Option<Vec<std::path::PathBuf>>,
    #[clap(long = "detectors", required = false, value_parser)]
    pub(crate) detectors: Option<Vec<String>>,
    #[clap(long = "project-root", required = false, value_parser)]
    pub(crate) project_root: Option<std::path::PathBuf>,
    #[clap(short = 'r', long = "recursive", action = clap::ArgAction::SetTrue)]
    pub(crate) recursive: bool,
}
