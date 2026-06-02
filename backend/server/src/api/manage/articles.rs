use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::{ArticleListParams, CreateArticle, PaginatedArticles, UpdateArticle};
use crate::{cache, db, error::AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/articles", get(list_articles).post(create_article))
        .route("/articles/:id", get(get_article).put(update_article).delete(delete_article))
        .route("/articles/:id/publish", post(publish_article))
        .route("/articles/:id/archive", post(archive_article))
}

async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleListParams>,
) -> Result<Json<PaginatedArticles>, AppError> {
    let (items, total) = db::articles::list(&state.db, &params, None).await?;
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    Ok(Json(PaginatedArticles { items, total, page, limit }))
}

async fn create_article(
    State(state): State<AppState>,
    Json(input): Json<CreateArticle>,
) -> Result<Json<common::Article>, AppError> {
    let article = db::articles::create(&state.db, input).await?;
    let _ = cache::del(&state.cache, cache::keys::ARTICLES_LIST).await;
    let _ = cache::del(&state.cache, cache::keys::HOMEPAGE_FEED).await;
    Ok(Json(article))
}

async fn get_article(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::ArticleWithTags>, AppError> {
    let article = db::articles::get_by_id(&state.db, id).await?;
    Ok(Json(article))
}

async fn update_article(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateArticle>,
) -> Result<Json<common::Article>, AppError> {
    let article = db::articles::update(&state.db, id, input).await?;
    let _ = cache::del(&state.cache, cache::keys::ARTICLES_LIST).await;
    let _ = cache::del(&state.cache, &cache::keys::article_slug(&article.slug)).await;
    let _ = cache::del(&state.cache, cache::keys::HOMEPAGE_FEED).await;
    Ok(Json(article))
}

async fn delete_article(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let article = db::articles::get_by_id(&state.db, id).await?;
    db::articles::delete(&state.db, id).await?;
    let _ = cache::del(&state.cache, cache::keys::ARTICLES_LIST).await;
    let _ = cache::del(&state.cache, &cache::keys::article_slug(&article.article.slug)).await;
    let _ = cache::del(&state.cache, cache::keys::HOMEPAGE_FEED).await;
    Ok(StatusCode::NO_CONTENT)
}

async fn publish_article(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::Article>, AppError> {
    let article = db::articles::publish(&state.db, id).await?;
    let _ = cache::del(&state.cache, cache::keys::ARTICLES_LIST).await;
    let _ = cache::del(&state.cache, &cache::keys::article_slug(&article.slug)).await;
    let _ = cache::del(&state.cache, cache::keys::HOMEPAGE_FEED).await;
    Ok(Json(article))
}
async fn archive_article(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::Article>, AppError> {
    let article = db::articles::archive(&state.db, id).await?;
    let _ = cache::del(&state.cache, cache::keys::ARTICLES_LIST).await;
    let _ = cache::del(&state.cache, &cache::keys::article_slug(&article.slug)).await;
    let _ = cache::del(&state.cache, cache::keys::HOMEPAGE_FEED).await;
    Ok(Json(article))
}