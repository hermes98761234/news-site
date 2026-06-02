// backend/server/src.main.rs
// Binary entry point. All shared code is in lib.rs.

use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Arc::new(server::config::Config::from_env()?);

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    sqlx::migrate!("../migrations").run(&db).await?;

    let cache = redis::Client::open(config.valkey_url.clone())?;

    let state = server::AppState { db, cache, config: config.clone() };
    let mut router = server::api::router(state);

    if let Some(ref static_dir) = config.static_dir {
        router = router.fallback_service(ServeDir::new(static_dir));
    }

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
