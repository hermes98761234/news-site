use axum::{
    extract::State,
    routing::{delete, get, post},
    Json, Router,
};
use common::CreateTag;
use crate::{db, error::AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:slug", delete(delete_tag))
}

async fn list_tags(
    State(state): State<AppState>,
) -> Result<Json<Vec<common::Tag>>, AppError> {
    let tags = db::tags::list(&state.db).await?;
    Ok(Json(tags))
}

async fn create_tag(
    State(state): State<AppState>,
    Json(input): Json<CreateTag>,
) -> Result<Json<common::Tag>, AppError> {
    let tag = db::tags::create(&state.db, input).await?;
    Ok(Json(tag))
}

async fn delete_tag(
    State(state): State<AppState>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> Result<axum::http::StatusCode, AppError> {
    db::tags::delete(&state.db, &slug).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
