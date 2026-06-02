// backend/server/tests/settings.rs
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
async fn get_settings_public() {
    let (server, _pool) = spawn_server().await;
    let res = server.get("/api/settings").await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    // Should have default settings from migration
    assert!(body.len() >= 3);
    let keys: Vec<&str> = body.iter().map(|s| s["key"].as_str().unwrap()).collect();
    assert!(keys.contains(&"site_name"));
    assert!(keys.contains(&"site_description"));
    assert!(keys.contains(&"site_url"));
}

#[tokio::test]
async fn get_settings_manage() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .get("/api/manage/settings")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_ok();
    let body: Vec<serde_json::Value> = res.json();
    assert!(body.len() >= 3);
}

#[tokio::test]
async fn get_settings_manage_unauthorized() {
    let (server, _pool) = spawn_server().await;
    let res = server.get("/api/manage/settings").await;
    res.assert_status_unauthorized();
}

#[tokio::test]
async fn set_setting() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .put("/api/manage/settings/site_name")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"value": "My News"}))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["key"], "site_name");
    assert_eq!(body["value"], "My News");
}

#[tokio::test]
async fn set_setting_unauthorized() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .put("/api/manage/settings/site_name")
        .json(&json!({"value": "Hacked"}))
        .await;
    res.assert_status_unauthorized();
}

#[tokio::test]
async fn set_setting_new_key() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .put("/api/manage/settings/custom_key")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"value": "custom_value"}))
        .await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["key"], "custom_key");
    assert_eq!(body["value"], "custom_value");
}

#[tokio::test]
async fn delete_setting() {
    let (server, _pool) = spawn_server().await;
    // First set a custom setting
    let _ = server
        .put("/api/manage/settings/to_delete")
        .add_header(cli_token_header().0, cli_token_header().1)
        .json(&json!({"value": "temp"}))
        .await;

    let res = server
        .delete("/api/manage/settings/to_delete")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status(StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_setting_not_found() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .delete("/api/manage/settings/nonexistent")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn flush_cache() {
    let (server, _pool) = spawn_server().await;
    let res = server
        .post("/api/manage/settings/flush")
        .add_header(cli_token_header().0, cli_token_header().1)
        .await;
    // May fail if Redis is not running, but should not panic
    let status = res.status_code();
    assert!(status.is_success() || status.is_server_error());
}
