use clap::Parser;
use html2pdf::{run, Error, Options};
use tracing::debug;

fn main() -> Result<(), Error> {
    init_tracing();

    let options = Options::parse();
    debug!(?options, "Parsed arguments");

    // Let's go
    run(&options)
}

fn init_tracing() {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::fmt::time;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_timer(time::uptime())
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .pretty()
        .init();
}
