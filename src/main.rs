mod submod;

use tracing_subscriber::EnvFilter;

fn main() {
    let filter = EnvFilter::builder()
        // placeholder for CLI argument
        .with_default_directive(tracing::Level::DEBUG.into())
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
