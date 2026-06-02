use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use crate::{cache, db, error::AppError, AppState};

#[derive(Deserialize)]
struct SetSettingRequest {
    value: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/settings", get(get_settings))
        .route("/settings/:key", put(set_setting).delete(delete_setting))
        .route("/settings/flush", post(flush_cache))
}

async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<Vec<common::Setting>>, AppError> {
    let settings = db::settings::get_all(&state.db).await?;
    Ok(Json(settings))
}

async fn set_setting(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(input): Json<SetSettingRequest>,
) -> Result<Json<common::Setting>, AppError> {
    let setting = db::settings::set(&state.db, &key, &input.value).await?;
    Ok(Json(setting))
}

async fn delete_setting(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<StatusCode, AppError> {
    let rows = sqlx::query("DELETE FROM settings WHERE key = ?")
        .bind(&key)
        .execute(&state.db)
        .await?
        .rows_affected();
    if rows == 0 {
        return Err(AppError::NotFound(format!("setting '{key}' not found")));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn flush_cache(
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let _ = cache::flush_pattern(&state.cache, "articles:*").await;
    let _ = cache::flush_pattern(&state.cache, "pages:*").await;
    let _ = cache::flush_pattern(&state.cache, "tags:*").await;
    let _ = cache::flush_pattern(&state.cache, "categories:*").await;
    let _ = cache::flush_pattern(&state.cache, "feed:*").await;
    Ok(StatusCode::NO_CONTENT)
}
