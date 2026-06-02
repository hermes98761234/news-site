use axum::{extract::State, routing::post, Json, Router};
use crate::{cache, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/cache/flush", post(flush))
}

async fn flush(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let pattern = body["pattern"].as_str().unwrap_or("*");
    cache::flush_pattern(&state.cache, pattern).await
        .map_err(|e| AppError::Internal(e))?;
    Ok(Json(serde_json::json!({ "flushed": true })))
}
