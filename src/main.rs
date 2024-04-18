mod submod;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[clap(about, version)]
struct CliArgs {
    #[clap(short, long, default_value_t = tracing::Level::DEBUG)]
    log_level: tracing::Level,
}

fn main() {
    let args = CliArgs::parse();

    let filter = EnvFilter::builder()
        .with_default_directive(args.log_level.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(true)
        .init();

    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");

    submod::function();
}
