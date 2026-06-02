// backend/server/tests/common/mod.rs
use axum_test::TestServer;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

/// Create a test server with an in-memory SQLite database and run migrations.
pub async fn spawn_server() -> (TestServer, sqlx::SqlitePool) {
    std::env::set_var("CLI_TOKEN", "test-token");

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to create in-memory sqlite pool");

    // Run migrations from the backend/migrations directory
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let cache = redis::Client::open("redis://127.0.0.1:6379").expect("failed to open redis");
    let config = Arc::new(server::config::Config {
        database_url: "sqlite::memory:".to_string(),
        valkey_url: "redis://127.0.0.1:6379".to_string(),
        cli_token: "test-token".to_string(),
        port: 0,
        static_dir: None,
    });
    let state = server::AppState { db: pool.clone(), cache, config };
    let app = server::api::router(state);
    let server = TestServer::new(app).expect("failed to create test server");
    (server, pool)
}

pub const TEST_TOKEN: &str = "test-token";
