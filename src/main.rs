mod submod;
use clap::Parser;
use eyre::WrapErr;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[clap(about, version)]
struct CliArgs {
    #[clap(short, long, verbatim_doc_comment, default_value_t = tracing::Level::DEBUG)]
    log_level: tracing::Level,

    #[clap(
        short = 'F',
        long,
        verbatim_doc_comment,
        value_parser,
        value_delimiter = ','
    )]
    log_filters: Vec<String>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = CliArgs::parse();
    let filters = args.log_filters.join(",");

    let env_filter = EnvFilter::builder()
        .with_default_directive(args.log_level.into())
        .from_env()
        .wrap_err(format!(
            "Failed to parse {:?}",
            std::env::var("RUST_LOG").unwrap_or_default()
        ))?;

    let cli_filter = EnvFilter::builder()
        .with_default_directive(args.log_level.into())
        .parse(&filters)
        .wrap_err(format!("Failed to parse {filters:?}"))?;

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_env_filter(cli_filter)
        .with_ansi(true)
        .init();

    tracing::info!("CLI arguments: {args:?}");

    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");

    submod::function();
    Ok(())
}
