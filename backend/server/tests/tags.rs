// backend/server/tests/tags.rs
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
async fn list_tags_empty() {
    let (server, _pool) = spawn_server().await;
    let res = server.get("/api/tags").await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    assert_eq!(body.len(), 0);
}

#[tokio::test]
async fn create_tag() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/tags")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"name": "Technology"}))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["name"], "Technology");
    assert_eq!(body["slug"], "technology");
    assert!(body["id"].is_number());
}

#[tokio::test]
async fn create_tag_unauthorized() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/tags")
        .json(&json!({"name": "Hacker"}))
        .await;
    res.assert_status_unauthorized();
}

#[tokio::test]
async fn list_tags() {
    let (server, _pool) = spawn_server().await;
    for name in &["Tech", "Science", "World"] {
        let _ = server
            .post("/api/manage/tags")
            .add_header(cli_token_header().0, cli_token_header().1)
            .json(&json!({"name": name}))
            .await;
    }

    let res = server.get("/api/tags").await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    assert_eq!(body.len(), 3);
}

#[tokio::test]
async fn delete_tag() {
    let (server, _pool) = spawn_server().await;
    let _ = server
        .post("/api/manage/tags")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"name": "To Delete"}))
        .await;

    let res = server
        .delete("/api/manage/tags/to-delete")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status(axum::http::StatusCode::NO_CONTENT);

    // Verify list is empty
    let list_res = server.get("/api/tags").await;
    let body: Vec<serde_json::Value> = list_res.json();
    assert_eq!(body.len(), 0);
}

#[tokio::test]
async fn delete_tag_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .delete("/api/manage/tags/nonexistent")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn articles_by_tag() {
    let (server, _pool) = spawn_server().await;
    // Create a tag
    let tag_res = server
        .post("/api/manage/tags")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"name": "Rust"}))
        .await;
    let tag: serde_json::Value = tag_res.json();
    let tag_id = tag["id"].as_i64().unwrap();

    // Create a published article with that tag
    let create_res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Rust Article",
            "body": "Rust content",
            "author_name": "Tester",
            "tag_ids": [tag_id]
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let article_id = created["id"].as_i64().unwrap();

    // Publish the article
    let _ = server
        .post(&format!("/api/manage/articles/{}/publish", article_id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;

    let res = server.get("/api/tags/rust/articles").await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["title"], "Rust Article");
}
