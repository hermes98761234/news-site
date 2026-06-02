// backend/server/src/api/mod.rs
pub mod manage;
pub mod public;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    Router,
};
use crate::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/api", public::router())
        .nest("/api/manage", manage::router()
            .layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
        .with_state(state)
}

async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req.headers()
        .get("x-cli-token")
        .and_then(|v| v.to_str().ok());
    if token != Some(state.config.cli_token.as_str()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(req).await)
}
