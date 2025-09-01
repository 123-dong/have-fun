pub async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm = signal(SignalKind::terminate()).expect("listen SIGTERM failed");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.expect("listen Ctrl+C failed");
    }
    tracing::info!("Shutting down...");
}

pub fn init_logging() {
    tracing_subscriber::fmt().compact().init();
}
