use axum::{
    extract::{Path, State},
    routing::get, Json, Router,
};
use crate::{cache, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/pages/:slug", get(get_page))
}

async fn get_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<common::Page>, AppError> {
    let key = cache::keys::page_slug(&slug);
    if let Ok(Some(cached)) = cache::get(&state.cache, &key).await {
        return Ok(Json(cached));
    }
    let page = crate::db::pages::get_by_slug(&state.db, &slug).await?;
    if page.status != "published" {
        return Err(AppError::NotFound(format!("page '{slug}' not found")));
    }
    let _ = cache::set(&state.cache, &key, &page, 3600).await;
    Ok(Json(page))
}
