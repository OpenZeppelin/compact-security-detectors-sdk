use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(short = 'm', long = "metadata", action = clap::ArgAction::SetTrue)]
    pub(crate) detectors: bool,
    #[clap(short = 'r', long = "run", required = false)]
    pub(crate) run: Option<String>,
}
