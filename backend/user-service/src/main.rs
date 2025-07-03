use dotenvy::dotenv;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() {
    init_env_tracing();

    let cpus = num_cpus::get();
    info!("Available CPUs: {}", cpus);

    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    info!("Connecting to: {}", database_url);
}

fn init_env_tracing() {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();
}
