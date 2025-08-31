use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;

pub fn shutdown_signal() -> Arc<Notify> {
    let notify = Arc::new(Notify::new());
    let n = notify.clone();
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{SignalKind, signal};
            let mut sigterm = signal(SignalKind::terminate()).unwrap();
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {},
                _ = sigterm.recv() => {},
            }
        }
        // Non-unix: 'Ctrl+C' only
        #[cfg(not(unix))]
        {
            tokio::signal::ctrl_c().await.unwrap();
        }
        tracing::info!("Shutting down...");
        tokio::time::sleep(Duration::from_millis(200)).await;
        n.notify_waiters();
    });

    notify
}

pub fn init_logging() {
    tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_file(true)
        .pretty()
        .init();
}
