// backend/server/tests/articles_public.rs
use axum::http::StatusCode;
use serde_json::json;

mod common;
use common::{spawn_server, TEST_TOKEN};

#[tokio::test]
async fn list_articles_empty() {
    let (server, _pool) = spawn_server().await;
    let res = server.get("/api/articles").await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["items"].as_array().unwrap().len(), 0);
    assert_eq!(body["total"], 0);
    assert_eq!(body["page"], 1);
    assert_eq!(body["limit"], 20);
}

#[tokio::test]
async fn list_articles_with_pagination() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .get("/api/articles")
        .add_query_param("page", 2)
        .add_query_param("limit", 5)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["page"], 2);
    assert_eq!(body["limit"], 5);
}

#[tokio::test]
async fn get_article_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server.get("/api/articles/nonexistent").await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn get_draft_article_returns_404() {
    let (server, _pool) = spawn_server().await;
    // Create a draft article via manage API
    let create_res = server
        .post("/api/manage/articles")
        .add_header(
            axum::http::header::HeaderName::from_static("x-cli-token"),
            axum::http::header::HeaderValue::from_static(TEST_TOKEN),
        )
        .json(&json!({
            "title": "Draft Article",
            "body": "Draft content",
            "author_name": "Tester"
        }))
        .await;
    create_res.assert_status_ok();

    // Public endpoint should return 404 for draft
    let res = server.get("/api/articles/draft-article").await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn get_published_article() {
    let (server, _pool) = spawn_server().await;
    // Create and publish article
    let create_res = server
        .post("/api/manage/articles")
        .add_header(
            axum::http::header::HeaderName::from_static("x-cli-token"),
            axum::http::header::HeaderValue::from_static(TEST_TOKEN),
        )
        .json(&json!({
            "title": "Published Article",
            "body": "Published content here",
            "author_name": "Tester"
        }))
        .await;
    create_res.assert_status_ok();
    let created: serde_json::Value = create_res.json();
    let article_id = created["id"].as_i64().unwrap();

    // Publish it
    let _ = server
        .post(&format!("/api/manage/articles/{}/publish", article_id))
        .add_header(
            axum::http::header::HeaderName::from_static("x-cli-token"),
            axum::http::header::HeaderValue::from_static(TEST_TOKEN),
        )
        .await;

    // Public get
    let res = server.get("/api/articles/published-article").await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "Published Article");
    assert_eq!(body["body"], "Published content here");
    assert_eq!(body["author_name"], "Tester");
    assert_eq!(body["status"], "published");
}
