use crate::errors::AppError;

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
    tracing_subscriber::fmt()
        .compact()
        .pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%Y-%m-%d %H:%M".to_string(),
        ))
        .init();
}

pub fn init_refl(
    descriptor: &[u8],
) -> Result<
    tonic_reflection::server::ServerReflectionServer<
        impl tonic_reflection::server::ServerReflection,
    >,
    AppError,
> {
    let refl = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(descriptor)
        .build_v1()?;

    Ok(refl)
}
