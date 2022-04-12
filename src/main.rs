use anyhow::Result;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

mod cli;
use clap::Parser;

mod events;
mod wasi_outbound_http_helper_k8s;

mod kube_config;
use kube_config::ConnectionConfig;

fn main() {
    let cli = cli::Cli::parse();

    // setup logging
    let level_filter = if cli.verbose { "debug" } else { "info" };
    let filter_layer = EnvFilter::new(level_filter).add_directive("hyper=off".parse().unwrap()); // this crate generates lots of tracing events we don't care about
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt::layer().with_writer(std::io::stderr))
        .init();

    if let Err(e) = run(cli) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}

fn run(cli: cli::Cli) -> Result<()> {
    match cli.command {
        cli::Commands::Events => {
            let connection_config = ConnectionConfig::from_kube_config()?;
            let req_cfg_id = connection_config.register()?;

            events::print_kubewarden_events(&connection_config, &req_cfg_id)
        }
    }
}
