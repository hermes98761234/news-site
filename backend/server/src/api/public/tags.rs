use axum::{
    extract::{Path, State},
    routing::get, Json, Router,
};
use common::ArticleListParams;
use crate::{AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list_tags))
        .route("/tags/:slug/articles", get(articles_by_tag))
}

async fn list_tags(State(state): State<AppState>) -> Result<Json<Vec<common::Tag>>, AppError> {
    Ok(Json(crate::db::tags::list(&state.db).await?))
}

async fn articles_by_tag(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<common::ArticleWithTags>>, AppError> {
    let params = ArticleListParams {
        page: None,
        limit: None,
        tag: Some(slug),
        category: None,
    };
    let (items, _) = crate::db::articles::list(&state.db, &params, Some("published")).await?;
    Ok(Json(items))
}
