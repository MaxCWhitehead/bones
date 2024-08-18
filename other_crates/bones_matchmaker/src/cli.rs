use clap::Parser;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub async fn start() {
    configure_logging();

    let args = crate::Config::parse();
    if let Err(e) = super::server(args).await {
        eprintln!("Error: {e}");
    }
}

fn configure_logging() {
    let console_layer = console_subscriber::spawn();
    tracing_subscriber::registry()
        .with(console_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_filter(
                    tracing_subscriber::EnvFilter::builder()
                        .with_default_directive(LevelFilter::INFO.into())
                        .from_env_lossy(),
                ),
        )
        .init();
    // tracing::subscriber::set_global_default(
    //     tracing_subscriber::FmtSubscriber::builder()
    //         .with_env_filter(
    //             tracing_subscriber::EnvFilter::builder()
    //                 .with_default_directive(LevelFilter::INFO.into())
    //                 .from_env_lossy(),
    //         )
    //         .finish(),
    // )
    // .unwrap();

    // console_subscriber::init();
}
