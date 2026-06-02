// backend/server/tests/pages.rs
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

// === PUBLIC ===

#[tokio::test]
async fn get_page_not_found_public() {
    let (server, _pool) = spawn_server().await;
    let res = server.get("/api/pages/nonexistent").await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn get_draft_page_returns_404() {
    let (server, _pool) = spawn_server().await;
    let _ = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Draft Page",
            "slug": "draft-page",
            "body": "Draft content"
        }))
        .await;

    let res = server.get("/api/pages/draft-page").await;
    res.assert_status_not_found();
}

// === MANAGE ===

#[tokio::test]
async fn create_page() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "About Us",
            "slug": "about-us",
            "body": "This is the about page."
        }))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "About Us");
    assert_eq!(body["slug"], "about-us");
    assert_eq!(body["body"], "This is the about page.");
    assert!(body["id"].is_number());
}

#[tokio::test]
async fn create_page_conflict() {
    let (server, _pool) = spawn_server().await;
    let _ = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Page 1",
            "slug": "same-slug",
            "body": "Content"
        }))
        .await;

    let res = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Page 2",
            "slug": "same-slug",
            "body": "Other"
        }))
        .await;
    res.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn create_page_unauthorized() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/pages")
        .json(&json!({
            "title": "No Auth",
            "slug": "no-auth",
            "body": "Nope"
        }))
        .await;
    res.assert_status_unauthorized();
}

#[tokio::test]
async fn get_page_by_id() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Test Page",
            "slug": "test-page",
            "body": "Body"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .get(&format!("/api/manage/pages/{}", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "Test Page");
    assert_eq!(body["body"], "Body");
}

#[tokio::test]
async fn get_page_by_id_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .get("/api/manage/pages/99999")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn update_page() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Original",
            "slug": "original",
            "body": "Original body"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .put(&format!("/api/manage/pages/{}", id))
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
}

#[tokio::test]
async fn delete_page() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "To Delete",
            "slug": "to-delete",
            "body": "Bye"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .delete(&format!("/api/manage/pages/{}", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status(StatusCode::NO_CONTENT);

    let get_res = server
        .get(&format!("/api/manage/pages/{}", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    get_res.assert_status_not_found();
}

#[tokio::test]
async fn publish_page() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "To Publish",
            "slug": "to-publish",
            "body": "Content"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    let res = server
        .post(&format!("/api/manage/pages/{}/publish", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["status"], "published");
}

#[tokio::test]
async fn list_pages() {
    let (server, _pool) = spawn_server().await;
    for i in 1..=3 {
        let _ = server
            .post("/api/manage/pages")
            .add_header(cli_token_header().0, cli_token_header().1)
            .json(&json!({
                "title": format!("Page {}", i),
                "slug": format!("page-{}", i),
                "body": format!("Body {}", i)
            }))
            .await;
    }

    let res = server
        .get("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    assert!(body.len() >= 3);
}

#[tokio::test]
async fn get_published_page_public() {
    let (server, _pool) = spawn_server().await;
    let create_res = server
        .post("/api/manage/pages")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({
            "title": "Published Page",
            "slug": "published-page",
            "body": "Public content"
        }))
        .await;
    let created: serde_json::Value = create_res.json();
    let id = created["id"].as_i64().unwrap();

    // Publish it
    let _ = server
        .post(&format!("/api/manage/pages/{}/publish", id))
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;

    // Public get should work now
    let res = server.get("/api/pages/published-page").await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "Published Page");
    assert_eq!(body["body"], "Public content");
    assert_eq!(body["status"], "published");
}
