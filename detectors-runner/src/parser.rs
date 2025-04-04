use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(short = 'd', long = "detectors", action = clap::ArgAction::SetTrue)]
    pub(crate) detectors: bool,
    #[clap(short = 'r', long = "run", required = false)]
    pub(crate) run: Option<String>,
}
