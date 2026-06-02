use crate::error::AppError;
use common::Setting;
use sqlx::SqlitePool;

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Setting>, AppError> {
    Ok(sqlx::query_as!(Setting, "SELECT key, value FROM settings ORDER BY key")
        .fetch_all(pool)
        .await?)
}

pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<Setting, AppError> {
    Ok(sqlx::query_as!(Setting,
        "INSERT INTO settings(key, value) VALUES(?,?) ON CONFLICT(key) DO UPDATE SET value=excluded.value RETURNING key, value",
        key, value
    )
    .fetch_one(pool)
    .await?)
}
