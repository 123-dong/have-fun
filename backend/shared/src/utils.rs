use tracing::info;

pub async fn graceful_shutdown() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.unwrap();
    }

    info!("Shutting down...");
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
}

pub fn init_logging() {
    tracing_subscriber::fmt().compact().pretty().init();
}

#[macro_export]
macro_rules! init_reflection {
    ($desc_set:expr) => {{
        tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set($desc_set)
            .build_v1()
    }};
}
