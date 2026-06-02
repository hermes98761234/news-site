use axum::{
    extract::State,
    routing::get, Json, Router,
};
use crate::{AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/settings", get(get_settings))
}

async fn get_settings(State(state): State<AppState>) -> Result<Json<Vec<common::Setting>>, AppError> {
    Ok(Json(crate::db::settings::get_all(&state.db).await?))
}
