use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::{CreateCategory, UpdateCategory};
use crate::{cache, db, error::AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories).post(create_category))
        .route("/categories/:slug", get(get_category).put(update_category).delete(delete_category))
}

async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<common::Category>>, AppError> {
    let categories = db::categories::list(&state.db).await?;
    Ok(Json(categories))
}

async fn create_category(
    State(state): State<AppState>,
    Json(input): Json<CreateCategory>,
) -> Result<Json<common::Category>, AppError> {
    let category = db::categories::create(&state.db, input).await?;
    let _ = cache::del(&state.cache, cache::keys::CATEGORIES_LIST).await;
    Ok(Json(category))
}

async fn get_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<common::Category>, AppError> {
    let category = db::categories::get_by_slug(&state.db, &slug).await?;
    Ok(Json(category))
}

async fn update_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<UpdateCategory>,
) -> Result<Json<common::Category>, AppError> {
    let existing = db::categories::get_by_slug(&state.db, &slug).await?;
    let category = db::categories::update(&state.db, existing.id, input).await?;
    let _ = cache::del(&state.cache, cache::keys::CATEGORIES_LIST).await;
    Ok(Json(category))
}

async fn delete_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<StatusCode, AppError> {
    db::categories::delete(&state.db, &slug).await?;
    let _ = cache::del(&state.cache, cache::keys::CATEGORIES_LIST).await;
    Ok(StatusCode::NO_CONTENT)
}
