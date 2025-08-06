use std::io;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize the tracing subscriber with optional file logging.
///
/// This function sets up structured logging with two outputs:
/// 1. Human-readable logs to stderr for development
/// 2. JSON logs to rotating files in user's home directory when filesystem is writable
///
/// If file logging fails (e.g., read-only filesystem), it gracefully
/// falls back to stderr-only logging without panicking.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stderr)
                .with_target(false),
        )
        .with(EnvFilter::from_default_env().add_directive("ghcid_mcp=debug".parse()?));

    // Try to add file logging in user's home directory
    if let Some(home_dir) = std::env::var_os("HOME") {
        let logs_dir = std::path::Path::new(&home_dir)
            .join(".ghcid-mcp")
            .join("logs");

        match std::fs::create_dir_all(&logs_dir) {
            Ok(_) => {
                let file_appender = tracing_appender::rolling::daily(&logs_dir, "ghcid-mcp.log");
                let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

                subscriber
                    .with(
                        tracing_subscriber::fmt::layer()
                            .json()
                            .with_writer(non_blocking)
                            .with_target(true)
                            .with_thread_ids(true)
                            .with_thread_names(true),
                    )
                    .init();

                eprintln!(
                    "Logging initialized with file output to {}",
                    logs_dir.display()
                );

                // Keep guard alive for the duration of the program
                // This prevents the background thread from shutting down
                std::mem::forget(_guard);
            }
            Err(e) => {
                subscriber.init();
                eprintln!(
                    "Failed to create logs directory at {}, using stderr only: {}",
                    logs_dir.display(),
                    e
                );
            }
        }
    } else {
        subscriber.init();
        eprintln!("HOME environment variable not found, using stderr only logging");
    }

    Ok(())
}
