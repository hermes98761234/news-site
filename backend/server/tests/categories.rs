// backend/server/tests/categories.rs
use axum::http::StatusCode;
use serde_json::json;

mod common;
use common::{spawn_server, TEST_TOKEN};

fn cli_token_header() -> (axum::http::header::HeaderName, axum::http::header::HeaderValue) {
    (
        axum::http::header::HeaderName::from_static("x-cli-token"),
        axum::http::header::HeaderValue::from_static(TEST_TOKEN),
    )
}

#[tokio::test]
async fn list_categories_empty() {
    let (server, _pool) = spawn_server().await;
    let res = server.get("/api/categories").await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    assert_eq!(body.len(), 0);
}

#[tokio::test]
async fn create_category() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/categories")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "name": "Politics",
            "description": "Political news"
        }))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "Politics");
    assert_eq!(body["slug"], "politics");
    assert_eq!(body["description"], "Political news");
    assert!(body["id"].is_number());
}

#[tokio::test]
async fn create_category_with_custom_slug() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/categories")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "name": "Tech News",
            "slug": "tech-news",
            "description": "Technology"
        }))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["slug"], "tech-news");
}

#[tokio::test]
async fn create_category_unauthorized() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/categories")
        .json(&json!({"name": "Hacker"}))
        .await;
    res.assert_status_unauthorized();
}

#[tokio::test]
async fn list_categories() {
    let (server, _pool) = spawn_server().await;
    for name in &["Politics", "Tech", "World"] {
        let _ = server
            .post("/api/manage/categories")
            .add_header(cli_token_header().0, cli_token_header().1)
            .json(&json!({"name": name}))
            .await;
    }

    let res = server.get("/api/categories").await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    assert_eq!(body.len(), 3);
}

#[tokio::test]
async fn get_category_by_slug() {
    let (server, _pool) = spawn_server().await;
    let _ = server
        .post("/api/manage/categories")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"name": "Science"}))
        .await;

    let res = server
        .get("/api/manage/categories/science")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "Science");
}

#[tokio::test]
async fn get_category_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .get("/api/manage/categories/nonexistent")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn update_category() {
    let (server, _pool) = spawn_server().await;
    let _ = server
        .post("/api/manage/categories")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"name": "Original"}))
        .await;

    let res = server
        .put("/api/manage/categories/original")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "name": "Updated",
            "description": "New desc"
        }))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "Updated");
    assert_eq!(body["description"], "New desc");
}

#[tokio::test]
async fn delete_category() {
    let (server, _pool) = spawn_server().await;
    let _ = server
        .post("/api/manage/categories")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"name": "To Delete"}))
        .await;

    let res = server
        .delete("/api/manage/categories/to-delete")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status(StatusCode::NO_CONTENT);

    let get_res = server
        .get("/api/manage/categories/to-delete")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    get_res.assert_status_not_found();
}

#[tokio::test]
async fn delete_category_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .delete("/api/manage/categories/nonexistent")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_not_found();
}
