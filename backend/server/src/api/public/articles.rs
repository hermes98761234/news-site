use axum::{
    extract::{Path, Query, State},
    routing::get, Json, Router,
};
use common::{ArticleListParams, PaginatedArticles};
use crate::{cache, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/articles", get(list_articles))
        .route("/articles/:slug", get(get_article))
}

async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleListParams>,
) -> Result<Json<PaginatedArticles>, AppError> {
    let cache_key = format!("{}:{:?}", cache::keys::ARTICLES_LIST, params);
    if let Ok(Some(cached)) = cache::get::<PaginatedArticles>(&state.cache, &cache_key).await {
        return Ok(Json(cached));
    }
    let (items, total) = crate::db::articles::list(&state.db, &params, Some("published")).await?;
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let result = PaginatedArticles { items, total, page, limit };
    let _ = cache::set(&state.cache, &cache_key, &result, 300).await;
    Ok(Json(result))
}

async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<common::ArticleWithTags>, AppError> {
    let key = cache::keys::article_slug(&slug);
    if let Ok(Some(cached)) = cache::get(&state.cache, &key).await {
        return Ok(Json(cached));
    }
    let article = crate::db::articles::get_by_slug(&state.db, &slug).await?;
    if article.article.status != "published" {
        return Err(AppError::NotFound(format!("article '{slug}' not found")));
    }
    let _ = cache::set(&state.cache, &key, &article, 3600).await;
    Ok(Json(article))
}
