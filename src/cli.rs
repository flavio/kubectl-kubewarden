use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = None,
)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    /// Enable verbose mode
    #[clap(short, long, env = "KREW_WASM_VERBOSE")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Run
    #[clap(arg_required_else_help = false)]
    Events,
}
