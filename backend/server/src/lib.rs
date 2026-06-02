// backend/server/src/lib.rs
// Library crate for integration tests.
// Re-export everything that tests need.
pub mod api;
pub mod cache;
pub mod config;
pub mod db;
pub mod error;

pub use api::router;
pub use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub cache: cache::RedisPool,
    pub config: std::sync::Arc<Config>,
}
