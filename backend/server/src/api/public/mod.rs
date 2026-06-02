// backend/server/src/api/public/mod.rs
mod articles;
mod categories;
mod pages;
mod settings;
mod tags;

use axum::Router;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(articles::router())
        .merge(pages::router())
        .merge(tags::router())
        .merge(categories::router())
        .merge(settings::router())
}
