// backend/server/tests/articles_manage.rs
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

// === CREATE ===

#[tokio::test]
async fn create_article() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Test Article",
            "body": "Hello world",
            "author_name": "Tester"
        }))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "Test Article");
    assert_eq!(body["body"], "Hello world");
    assert_eq!(body["author_name"], "Tester");
    assert!(body["id"].is_number());
}

#[tokio::test]
async fn create_article_conflict_duplicate_slug() {
    let (server, _pool) = spawn_server().await;
    let _ = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Test Article",
            "body": "Hello",
            "author_name": "Tester"
        }))
        .await;

    let res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Test Article",
            "body": "Different body",
            "author_name": "Other"
        }))
        .await;
    res.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn create_article_unauthorized() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/articles")
        .json(&json!({
            "title": "Test",
            "body": "Body",
            "author_name": "Tester"
        }))
        .await;
    res.assert_status_unauthorized();
}

// === GET ===

#[tokio::test]
async fn get_article_by_id() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Get Test",
            "body": "Content",
            "author_name": "Tester"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .get(&format!("/api/manage/articles/{}", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "Get Test");
}

#[tokio::test]
async fn get_article_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .get("/api/manage/articles/99999")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_not_found();
}

// === UPDATE ===

#[tokio::test]
async fn update_article() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Original Title",
            "body": "Original body",
            "author_name": "Tester"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .put(&format!("/api/manage/articles/{}", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Updated Title",
            "body": "Updated body"
        }))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "Updated Title");
    assert_eq!(body["body"], "Updated body");
    // author_name should remain unchanged
    assert_eq!(body["author_name"], "Tester");
}

#[tokio::test]
async fn update_article_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .put("/api/manage/articles/99999")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"title": "New"}))
        .await;
    res.assert_status_not_found();
}

// === DELETE ===

#[tokio::test]
async fn delete_article() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "To Delete",
            "body": "Bye",
            "author_name": "Tester"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .delete(&format!("/api/manage/articles/{}", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status(StatusCode::NO_CONTENT);

    // Verify it's gone
    let get_res = server
        .get(&format!("/api/manage/articles/{}", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    get_res.assert_status_not_found();
}

#[tokio::test]
async fn delete_article_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .delete("/api/manage/articles/99999")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_not_found();
}

// === PUBLISH / ARCHIVE ===

#[tokio::test]
async fn publish_article() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Draft to Publish",
            "body": "Content",
            "author_name": "Tester"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .post(&format!("/api/manage/articles/{}/publish", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["status"], "published");
}

#[tokio::test]
async fn archive_article() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "To Archive",
            "body": "Content",
            "author_name": "Tester"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .post(&format!("/api/manage/articles/{}/archive", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["status"], "archived");
}

// === LIST ===

#[tokio::test]
async fn list_articles_manage() {
    let (server, _pool) = spawn_server().await;
    // Create a few articles
    for i in 1..=3 {
        let _ = server
            .post("/api/manage/articles")
            .add_header(cli_token_header().0, cli_token_header().1)
            .json(&json!({
                "title": format!("Article {}", i),
                "body": format!("Body {}", i),
                "author_name": "Tester"
            }))
            .await;
    }

    let res = server
        .get("/api/manage/articles")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["items"].as_array().unwrap().len(), 3);
    assert_eq!(body["total"], 3);
}
