use axum::{
    extract::State,
    routing::get, Json, Router,
};
use crate::{AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/categories", get(list_categories))
}

async fn list_categories(State(state): State<AppState>) -> Result<Json<Vec<common::Category>>, AppError> {
    Ok(Json(crate::db::categories::list(&state.db).await?))
}
