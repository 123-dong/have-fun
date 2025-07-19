use std::sync::Arc;
use tracing::info;

// Local crates import
use user_service::{config::Config, db, routes, state};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().pretty().init();
    let config = Config::from_env();

    // info!("CPUs: {}", num_cpus::get());
    info!("DB pool max connections: {}", config.max_connections);
    info!("Connecting to PostgreSQL...");

    // Initialize database
    let pool = db::init_pool(&config.database_url, config.max_connections).await?;
    info!("Connected to PostgreSQL");

    // Set up Axum router with state & routes
    let app_state = state::AppState {
        database: Arc::new(pool),
    };
    let app = routes::create_router(app_state.into());

    // Bind & serve
    let addr: std::net::SocketAddr = config.bind_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
