// backend/server/src/cache/mod.rs
pub mod keys;

use anyhow::Result;
use redis::AsyncCommands;

pub type RedisPool = redis::Client;

pub async fn get<T: serde::de::DeserializeOwned>(
    client: &RedisPool,
    key: &str,
) -> Result<Option<T>> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let val: Option<String> = conn.get(key).await?;
    Ok(val.and_then(|s| serde_json::from_str(&s).ok()))
}

pub async fn set<T: serde::Serialize>(
    client: &RedisPool,
    key: &str,
    value: &T,
    ttl_secs: u64,
) -> Result<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let serialized = serde_json::to_string(value)?;
    conn.set_ex(key, serialized, ttl_secs).await?;
    Ok(())
}

pub async fn del(client: &RedisPool, key: &str) -> Result<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.del(key).await?;
    Ok(())
}

pub async fn flush_pattern(client: &RedisPool, pattern: &str) -> Result<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let keys: Vec<String> = conn.keys(pattern).await?;
    if !keys.is_empty() {
        conn.del(keys).await?;
    }
    Ok(())
}
