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
    let filter_expression = args.log_filters.join(",");

    let cli_filter = EnvFilter::builder()
        .with_default_directive(args.log_level.into())
        .parse(&filter_expression)
        .wrap_err(format!("Failed to parse {filter_expression:?}"))?;

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

    let builder = tracing_subscriber::fmt()
        .with_env_filter(cli_filter)
        .with_writer(writer)
        .with_filter_reloading()
        .with_ansi(true);

    let reload_handle = builder.reload_handle();

    builder.init();

    tracing::info!("CLI arguments: {args:?}");

    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");

    submod::function();

    tracing::info!("Reloading logging filter");
    let new_filter = EnvFilter::builder().parse_lossy("logging_sandbox::submod=ERROR");
    reload_handle.reload(new_filter)?;

    submod::function();

    Ok(())
}
