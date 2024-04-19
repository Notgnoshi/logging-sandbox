mod submod;

use std::path::PathBuf;

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

    let log_file = PathBuf::from("./log/example.log");
    let should_rotate_on_startup = log_file.exists();
    let max_files = 4;
    let mut rolling_file = file_rotate::FileRotate::new(
        log_file,
        file_rotate::suffix::AppendCount::new(max_files),
        file_rotate::ContentLimit::None,
        file_rotate::compression::Compression::None,
        None,
    );
    // Rotate the file now, on startup, before we start logging to it.
    if should_rotate_on_startup {
        rolling_file.rotate()?;
    }
    // TODO: There's a builder to set the thread name and queue size
    let (async_rolling_file, _guard) = tracing_appender::non_blocking(rolling_file);
    let writer = tracing_subscriber::fmt::writer::Tee::new(async_rolling_file, std::io::stdout);

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_env_filter(cli_filter)
        .with_ansi(true) // This logs to the file in color
        .with_writer(writer)
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
