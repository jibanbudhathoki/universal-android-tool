pub mod config;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod services;

use config::AppConfig;
use routes::app_router;
use services::AppState;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,uat_rust_api=debug".into()),
        )
        .init();

    let config = AppConfig::from_env();
    let state = AppState::new();
    let app = app_router(state);

    let listener = tokio::net::TcpListener::bind(config.socket_addr()).await?;
    info!("Universal Android Tool API listening on {}", config.socket_addr());

    axum::serve(listener, app).await?;
    Ok(())
}
