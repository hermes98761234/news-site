use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::{CreatePage, UpdatePage};
use crate::{cache, db, error::AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pages", get(list_pages).post(create_page))
        .route("/pages/:id", get(get_page).put(update_page).delete(delete_page))
        .route("/pages/:id/publish", post(publish_page))
}

async fn list_pages(
    State(state): State<AppState>,
) -> Result<Json<Vec<common::Page>>, AppError> {
    let pages = db::pages::list(&state.db).await?;
    Ok(Json(pages))
}

async fn create_page(
    State(state): State<AppState>,
    Json(input): Json<CreatePage>,
) -> Result<Json<common::Page>, AppError> {
    let page = db::pages::create(&state.db, input).await?;
    Ok(Json(page))
}

async fn get_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::Page>, AppError> {
    let page = db::pages::get_by_id(&state.db, id).await?;
    Ok(Json(page))
}

async fn update_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdatePage>,
) -> Result<Json<common::Page>, AppError> {
    let page = db::pages::update(&state.db, id, input).await?;
    let _ = cache::del(&state.cache, &cache::keys::page_slug(&page.slug)).await;
    Ok(Json(page))
}

async fn delete_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let page = db::pages::get_by_id(&state.db, id).await?;
    db::pages::delete(&state.db, id).await?;
    let _ = cache::del(&state.cache, &cache::keys::page_slug(&page.slug)).await;
    Ok(StatusCode::NO_CONTENT)
}

async fn publish_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::Page>, AppError> {
    let page = db::pages::publish(&state.db, id).await?;
    let _ = cache::del(&state.cache, &cache::keys::page_slug(&page.slug)).await;
    Ok(Json(page))
}
