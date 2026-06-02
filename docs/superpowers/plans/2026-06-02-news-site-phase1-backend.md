# News Site — Phase 1: Backend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the Rust backend — SQLite DB, Valkey cache, Axum REST API (public + management endpoints), fully tested.

**Architecture:** Single Rust workspace with `common`, `server` crates. Axum serves REST API + static files. sqlx for SQLite, redis-rs for Valkey. Management endpoints protected by `X-CLI-Token` header.

**Tech Stack:** Rust 1.78, Axum 0.7, sqlx 0.7 (sqlite), redis-rs 0.25, tokio, serde, tower-http, slug

---

## File Map

```
backend/
├── Cargo.toml                          # workspace
├── common/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                      # Article, Page, Tag, Category, Settings types + CreateArticle etc.
├── server/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                     # tokio main, config, router setup
│   │   ├── config.rs                   # Config struct from env vars
│   │   ├── error.rs                    # AppError, IntoResponse impl
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   ├── articles.rs             # CRUD + list + publish/archive
│   │   │   ├── pages.rs
│   │   │   ├── tags.rs
│   │   │   ├── categories.rs
│   │   │   └── settings.rs
│   │   ├── cache/
│   │   │   ├── mod.rs
│   │   │   └── keys.rs                 # key constants + flush helpers
│   │   └── api/
│   │       ├── mod.rs                  # router assembly + auth middleware
│   │       ├── public/
│   │       │   ├── articles.rs
│   │       │   ├── pages.rs
│   │       │   ├── tags.rs
│   │       │   ├── categories.rs
│   │       │   └── settings.rs
│   │       └── manage/
│   │           ├── articles.rs
│   │           ├── pages.rs
│   │           ├── tags.rs
│   │           ├── categories.rs
│   │           └── settings.rs
│   └── tests/
│       ├── common/
│       │   └── mod.rs                  # test helpers: spawn_app, test DB setup
│       ├── articles_public.rs
│       ├── articles_manage.rs
│       ├── pages.rs
│       ├── tags.rs
│       ├── categories.rs
│       └── settings.rs
└── migrations/
    ├── 001_categories.sql
    ├── 002_tags.sql
    ├── 003_articles.sql
    ├── 004_article_tags.sql
    ├── 005_pages.sql
    └── 006_settings.sql
```

---

### Task 1: Workspace scaffold

**Files:**
- Create: `backend/Cargo.toml`
- Create: `backend/common/Cargo.toml`
- Create: `backend/server/Cargo.toml`

- [ ] **Step 1: Create workspace Cargo.toml**

```toml
# backend/Cargo.toml
[workspace]
members = ["common", "server", "cli"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
slug = "0.1"
```

- [ ] **Step 2: Create common/Cargo.toml**

```toml
# backend/common/Cargo.toml
[package]
name = "common"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
chrono = { workspace = true }
uuid = { workspace = true }
```

- [ ] **Step 3: Create server/Cargo.toml**

```toml
# backend/server/Cargo.toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { path = "../common" }
axum = { version = "0.7", features = ["macros"] }
tower-http = { version = "0.5", features = ["fs", "cors"] }
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio", "macros", "chrono", "uuid"] }
redis = { version = "0.25", features = ["tokio-comp"] }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
chrono = { workspace = true }
uuid = { workspace = true }
slug = { workspace = true }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dotenvy = "0.15"

[dev-dependencies]
axum-test = "14"
tokio = { workspace = true }
```

- [ ] **Step 4: Create placeholder cli/Cargo.toml** (so workspace compiles)

```toml
# backend/cli/Cargo.toml
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "news-cli"
path = "src/main.rs"

[dependencies]
common = { path = "../common" }
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
toml = "0.8"
serde = { workspace = true }
tokio = { workspace = true }
anyhow = { workspace = true }
comfy-table = "7"
colored = "2"
```

- [ ] **Step 5: Create cli/src/main.rs stub**

```rust
// backend/cli/src/main.rs
fn main() {}
```

- [ ] **Step 6: Verify workspace compiles**

```bash
cd backend && cargo check
```
Expected: no errors

- [ ] **Step 7: Commit**

```bash
cd backend && git add -A && git commit -m "feat: init rust workspace scaffold"
```

---

### Task 2: Migrations

**Files:**
- Create: `backend/migrations/001_categories.sql` through `006_settings.sql`

- [ ] **Step 1: Create migration files**

```sql
-- backend/migrations/001_categories.sql
CREATE TABLE IF NOT EXISTS categories (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    slug        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
```

```sql
-- backend/migrations/002_tags.sql
CREATE TABLE IF NOT EXISTS tags (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    slug       TEXT NOT NULL UNIQUE,
    name       TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

```sql
-- backend/migrations/003_articles.sql
CREATE TABLE IF NOT EXISTS articles (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    slug         TEXT NOT NULL UNIQUE,
    title        TEXT NOT NULL,
    excerpt      TEXT NOT NULL DEFAULT '',
    body         TEXT NOT NULL DEFAULT '',
    author_name  TEXT NOT NULL,
    status       TEXT NOT NULL DEFAULT 'draft' CHECK(status IN ('draft','published','archived')),
    category_id  INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    published_at TEXT,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_articles_status ON articles(status);
CREATE INDEX IF NOT EXISTS idx_articles_slug   ON articles(slug);
```

```sql
-- backend/migrations/004_article_tags.sql
CREATE TABLE IF NOT EXISTS article_tags (
    article_id INTEGER NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    tag_id     INTEGER NOT NULL REFERENCES tags(id)     ON DELETE CASCADE,
    PRIMARY KEY (article_id, tag_id)
);
```

```sql
-- backend/migrations/005_pages.sql
CREATE TABLE IF NOT EXISTS pages (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    slug       TEXT NOT NULL UNIQUE,
    title      TEXT NOT NULL,
    body       TEXT NOT NULL DEFAULT '',
    status     TEXT NOT NULL DEFAULT 'draft' CHECK(status IN ('draft','published')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

```sql
-- backend/migrations/006_settings.sql
CREATE TABLE IF NOT EXISTS settings (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL DEFAULT ''
);
INSERT OR IGNORE INTO settings(key, value) VALUES
    ('site_name', 'My News Site'),
    ('site_description', ''),
    ('site_url', 'http://localhost:3000');
```

- [ ] **Step 2: Commit**

```bash
git add backend/migrations/ && git commit -m "feat: add sqlite migrations"
```

---

### Task 3: Common types

**Files:**
- Create: `backend/common/src/lib.rs`

- [ ] **Step 1: Write common/src/lib.rs**

```rust
// backend/common/src/lib.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Article {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub body: String,
    pub author_name: String,
    pub status: String,
    pub category_id: Option<i64>,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleWithTags {
    #[serde(flatten)]
    pub article: Article,
    pub tags: Vec<Tag>,
    pub category: Option<Category>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub excerpt: Option<String>,
    pub body: String,
    pub author_name: String,
    pub category_id: Option<i64>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub body: Option<String>,
    pub author_name: Option<String>,
    pub category_id: Option<i64>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Page {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePage {
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePage {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSetting {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ArticleListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub tag: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedArticles {
    pub items: Vec<ArticleWithTags>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
```

- [ ] **Step 2: Verify it compiles**

```bash
cd backend && cargo check -p common
```
Expected: no errors

- [ ] **Step 3: Commit**

```bash
git add backend/common/ && git commit -m "feat: add common domain types"
```

---

### Task 4: Config and error types

**Files:**
- Create: `backend/server/src/config.rs`
- Create: `backend/server/src/error.rs`

- [ ] **Step 1: Write config.rs**

```rust
// backend/server/src/config.rs
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub valkey_url: String,
    pub cli_token: String,
    pub port: u16,
    pub static_dir: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "news.db".to_string()),
            valkey_url: std::env::var("VALKEY_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            cli_token: std::env::var("CLI_TOKEN")
                .map_err(|_| anyhow::anyhow!("CLI_TOKEN env var required"))?,
            port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            static_dir: std::env::var("STATIC_DIR").ok(),
        })
    }
}
```

- [ ] **Step 2: Write error.rs**

```rust
// backend/server/src/error.rs
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Conflict(String),
    BadRequest(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(m)   => (StatusCode::NOT_FOUND, m),
            AppError::Conflict(m)   => (StatusCode::CONFLICT, m),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Internal(e)   => {
                tracing::error!("internal error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self { AppError::Internal(e) }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound("not found".to_string()),
            _ => AppError::Internal(e.into()),
        }
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add backend/server/src/config.rs backend/server/src/error.rs
git commit -m "feat: add server config and error types"
```

---

### Task 5: DB layer — articles

**Files:**
- Create: `backend/server/src/db/mod.rs`
- Create: `backend/server/src/db/articles.rs`

- [ ] **Step 1: Write failing test for article creation**

```rust
// backend/server/tests/articles_manage.rs
use axum_test::TestServer;
// helpers defined in Task 10
mod common;

#[sqlx::test(migrations = "../migrations")]
async fn test_create_article_returns_201(pool: sqlx::SqlitePool) {
    let server = common::spawn_server(pool).await;
    let res = server
        .post("/api/manage/articles")
        .add_header("x-cli-token", "test-token")
        .json(&serde_json::json!({
            "title": "Hello World",
            "body": "Content here",
            "author_name": "Alice",
            "excerpt": "Short summary"
        }))
        .await;
    res.assert_status_created();
    let body: serde_json::Value = res.json();
    assert_eq!(body["title"], "Hello World");
    assert_eq!(body["slug"], "hello-world");
    assert_eq!(body["status"], "draft");
}
```

- [ ] **Step 2: Write db/mod.rs**

```rust
// backend/server/src/db/mod.rs
pub mod articles;
pub mod categories;
pub mod pages;
pub mod settings;
pub mod tags;

pub use sqlx::SqlitePool;
```

- [ ] **Step 3: Write db/articles.rs**

```rust
// backend/server/src/db/articles.rs
use crate::error::AppError;
use common::{Article, ArticleListParams, ArticleWithTags, Category, CreateArticle, Tag, UpdateArticle};
use sqlx::SqlitePool;
use slug::slugify;

pub async fn create(pool: &SqlitePool, input: CreateArticle) -> Result<Article, AppError> {
    let slug = slugify(&input.title);
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM articles WHERE slug = ?"
    )
    .bind(&slug)
    .fetch_one(pool)
    .await?;
    if existing > 0 {
        return Err(AppError::Conflict(format!("slug '{slug}' already exists")));
    }
    let article = sqlx::query_as::<_, Article>(
        "INSERT INTO articles (slug, title, excerpt, body, author_name, category_id)
         VALUES (?, ?, ?, ?, ?, ?)
         RETURNING *"
    )
    .bind(&slug)
    .bind(&input.title)
    .bind(input.excerpt.unwrap_or_default())
    .bind(&input.body)
    .bind(&input.author_name)
    .bind(input.category_id)
    .fetch_one(pool)
    .await?;
    if let Some(tag_ids) = input.tag_ids {
        for tag_id in tag_ids {
            sqlx::query("INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?, ?)")
                .bind(article.id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }
    }
    Ok(article)
}

pub async fn get_by_slug(pool: &SqlitePool, slug: &str) -> Result<ArticleWithTags, AppError> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE slug = ?"
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("article '{slug}' not found")))?;

    let tags = sqlx::query_as::<_, Tag>(
        "SELECT t.* FROM tags t
         JOIN article_tags at ON at.tag_id = t.id
         WHERE at.article_id = ?"
    )
    .bind(article.id)
    .fetch_all(pool)
    .await?;

    let category = if let Some(cat_id) = article.category_id {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
            .bind(cat_id)
            .fetch_optional(pool)
            .await?
    } else {
        None
    };

    Ok(ArticleWithTags { article, tags, category })
}

pub async fn list(
    pool: &SqlitePool,
    params: &ArticleListParams,
    status: Option<&str>,
) -> Result<(Vec<ArticleWithTags>, i64), AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;
    let status_filter = status.unwrap_or("published");

    let articles = sqlx::query_as::<_, Article>(
        "SELECT a.* FROM articles a
         LEFT JOIN categories c ON c.id = a.category_id
         WHERE a.status = ?
           AND (? IS NULL OR c.slug = ?)
         ORDER BY a.published_at DESC, a.created_at DESC
         LIMIT ? OFFSET ?"
    )
    .bind(status_filter)
    .bind(&params.category)
    .bind(&params.category)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM articles a
         LEFT JOIN categories c ON c.id = a.category_id
         WHERE a.status = ?
           AND (? IS NULL OR c.slug = ?)"
    )
    .bind(status_filter)
    .bind(&params.category)
    .bind(&params.category)
    .fetch_one(pool)
    .await?;

    let mut result = Vec::new();
    for article in articles {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT t.* FROM tags t JOIN article_tags at ON at.tag_id = t.id WHERE at.article_id = ?"
        )
        .bind(article.id)
        .fetch_all(pool)
        .await?;

        if let Some(ref tag_slug) = params.tag {
            if !tags.iter().any(|t| &t.slug == tag_slug) {
                continue;
            }
        }

        let category = if let Some(cat_id) = article.category_id {
            sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
                .bind(cat_id)
                .fetch_optional(pool)
                .await?
        } else {
            None
        };

        result.push(ArticleWithTags { article, tags, category });
    }

    Ok((result, total))
}

pub async fn update(pool: &SqlitePool, id: i64, input: UpdateArticle) -> Result<Article, AppError> {
    let article = sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("article {id} not found")))?;

    let article = sqlx::query_as::<_, Article>(
        "UPDATE articles SET
           title       = COALESCE(?, title),
           excerpt     = COALESCE(?, excerpt),
           body        = COALESCE(?, body),
           author_name = COALESCE(?, author_name),
           category_id = COALESCE(?, category_id),
           updated_at  = datetime('now')
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.title)
    .bind(input.excerpt)
    .bind(input.body)
    .bind(input.author_name)
    .bind(input.category_id)
    .bind(article.id)
    .fetch_one(pool)
    .await?;

    if let Some(tag_ids) = input.tag_ids {
        sqlx::query("DELETE FROM article_tags WHERE article_id = ?")
            .bind(article.id)
            .execute(pool)
            .await?;
        for tag_id in tag_ids {
            sqlx::query("INSERT OR IGNORE INTO article_tags (article_id, tag_id) VALUES (?, ?)")
                .bind(article.id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }
    }

    Ok(article)
}

pub async fn publish(pool: &SqlitePool, id: i64) -> Result<Article, AppError> {
    sqlx::query_as::<_, Article>(
        "UPDATE articles SET status = 'published', published_at = datetime('now'), updated_at = datetime('now')
         WHERE id = ? RETURNING *"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("article {id} not found")))
}

pub async fn archive(pool: &SqlitePool, id: i64) -> Result<Article, AppError> {
    sqlx::query_as::<_, Article>(
        "UPDATE articles SET status = 'archived', updated_at = datetime('now')
         WHERE id = ? RETURNING *"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("article {id} not found")))
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
    if rows == 0 {
        return Err(AppError::NotFound(format!("article {id} not found")));
    }
    Ok(())
}
```

- [ ] **Step 4: Commit**

```bash
git add backend/server/src/db/ && git commit -m "feat: add article db layer"
```

---

### Task 6: DB layer — pages, tags, categories, settings

**Files:**
- Create: `backend/server/src/db/pages.rs`
- Create: `backend/server/src/db/tags.rs`
- Create: `backend/server/src/db/categories.rs`
- Create: `backend/server/src/db/settings.rs`

- [ ] **Step 1: Write db/pages.rs**

```rust
// backend/server/src/db/pages.rs
use crate::error::AppError;
use common::{CreatePage, Page, UpdatePage};
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, input: CreatePage) -> Result<Page, AppError> {
    let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM pages WHERE slug = ?")
        .bind(&input.slug)
        .fetch_one(pool)
        .await?;
    if existing > 0 {
        return Err(AppError::Conflict(format!("slug '{}' already exists", input.slug)));
    }
    Ok(sqlx::query_as::<_, Page>(
        "INSERT INTO pages (slug, title, body) VALUES (?, ?, ?) RETURNING *"
    )
    .bind(&input.slug)
    .bind(&input.title)
    .bind(input.body.unwrap_or_default())
    .fetch_one(pool)
    .await?)
}

pub async fn get_by_slug(pool: &SqlitePool, slug: &str) -> Result<Page, AppError> {
    sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("page '{slug}' not found")))
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Page>, AppError> {
    Ok(sqlx::query_as::<_, Page>("SELECT * FROM pages ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?)
}

pub async fn update(pool: &SqlitePool, id: i64, input: UpdatePage) -> Result<Page, AppError> {
    sqlx::query_as::<_, Page>(
        "UPDATE pages SET title = COALESCE(?, title), body = COALESCE(?, body),
         updated_at = datetime('now') WHERE id = ? RETURNING *"
    )
    .bind(input.title)
    .bind(input.body)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("page {id} not found")))
}

pub async fn publish(pool: &SqlitePool, id: i64) -> Result<Page, AppError> {
    sqlx::query_as::<_, Page>(
        "UPDATE pages SET status = 'published', updated_at = datetime('now') WHERE id = ? RETURNING *"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("page {id} not found")))
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM pages WHERE id = ?")
        .bind(id).execute(pool).await?.rows_affected();
    if rows == 0 { return Err(AppError::NotFound(format!("page {id} not found"))); }
    Ok(())
}
```

- [ ] **Step 2: Write db/tags.rs**

```rust
// backend/server/src/db/tags.rs
use crate::error::AppError;
use common::{CreateTag, Tag};
use slug::slugify;
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, input: CreateTag) -> Result<Tag, AppError> {
    let slug = slugify(&input.name);
    Ok(sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (slug, name) VALUES (?, ?) RETURNING *"
    )
    .bind(&slug)
    .bind(&input.name)
    .fetch_one(pool)
    .await?)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Tag>, AppError> {
    Ok(sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name").fetch_all(pool).await?)
}

pub async fn delete(pool: &SqlitePool, slug: &str) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM tags WHERE slug = ?")
        .bind(slug).execute(pool).await?.rows_affected();
    if rows == 0 { return Err(AppError::NotFound(format!("tag '{slug}' not found"))); }
    Ok(())
}
```

- [ ] **Step 3: Write db/categories.rs**

```rust
// backend/server/src/db/categories.rs
use crate::error::AppError;
use common::{Category, CreateCategory, UpdateCategory};
use slug::slugify;
use sqlx::SqlitePool;

pub async fn create(pool: &SqlitePool, input: CreateCategory) -> Result<Category, AppError> {
    let slug = input.slug.unwrap_or_else(|| slugify(&input.name));
    Ok(sqlx::query_as::<_, Category>(
        "INSERT INTO categories (slug, name, description) VALUES (?, ?, ?) RETURNING *"
    )
    .bind(&slug)
    .bind(&input.name)
    .bind(input.description.unwrap_or_default())
    .fetch_one(pool)
    .await?)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Category>, AppError> {
    Ok(sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name").fetch_all(pool).await?)
}

pub async fn get_by_slug(pool: &SqlitePool, slug: &str) -> Result<Category, AppError> {
    sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("category '{slug}' not found")))
}

pub async fn update(pool: &SqlitePool, id: i64, input: UpdateCategory) -> Result<Category, AppError> {
    sqlx::query_as::<_, Category>(
        "UPDATE categories SET name = COALESCE(?, name), description = COALESCE(?, description),
         updated_at = datetime('now') WHERE id = ? RETURNING *"
    )
    .bind(input.name)
    .bind(input.description)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("category {id} not found")))
}

pub async fn delete(pool: &SqlitePool, slug: &str) -> Result<(), AppError> {
    let rows = sqlx::query("DELETE FROM categories WHERE slug = ?")
        .bind(slug).execute(pool).await?.rows_affected();
    if rows == 0 { return Err(AppError::NotFound(format!("category '{slug}' not found"))); }
    Ok(())
}
```

- [ ] **Step 4: Write db/settings.rs**

```rust
// backend/server/src/db/settings.rs
use crate::error::AppError;
use common::Setting;
use sqlx::SqlitePool;

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Setting>, AppError> {
    Ok(sqlx::query_as!(Setting, "SELECT key, value FROM settings ORDER BY key")
        .fetch_all(pool)
        .await?)
}

pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<Setting, AppError> {
    Ok(sqlx::query_as!(Setting,
        "INSERT INTO settings(key, value) VALUES(?,?) ON CONFLICT(key) DO UPDATE SET value=excluded.value RETURNING key, value",
        key, value
    )
    .fetch_one(pool)
    .await?)
}
```

- [ ] **Step 5: Compile check**

```bash
cd backend && cargo check -p server
```
Expected: no errors

- [ ] **Step 6: Commit**

```bash
git add backend/server/src/db/ && git commit -m "feat: add pages/tags/categories/settings db layers"
```

---

### Task 7: Cache layer

**Files:**
- Create: `backend/server/src/cache/mod.rs`
- Create: `backend/server/src/cache/keys.rs`

- [ ] **Step 1: Write cache/keys.rs**

```rust
// backend/server/src/cache/keys.rs
pub const ARTICLES_LIST: &str = "articles:list";
pub const TAGS_LIST: &str = "tags:list";
pub const CATEGORIES_LIST: &str = "categories:list";
pub const HOMEPAGE_FEED: &str = "feed:homepage";

pub fn article_slug(slug: &str) -> String { format!("articles:slug:{slug}") }
pub fn page_slug(slug: &str) -> String { format!("pages:slug:{slug}") }
pub fn tag_articles(slug: &str) -> String { format!("tags:{slug}:articles") }
pub fn category_articles(slug: &str) -> String { format!("categories:{slug}:articles") }
```

- [ ] **Step 2: Write cache/mod.rs**

```rust
// backend/server/src/cache/mod.rs
pub mod keys;

use anyhow::Result;
use redis::AsyncCommands;

pub type RedisPool = redis::Client;

pub async fn get<T: serde::de::DeserializeOwned>(
    client: &RedisPool,
    key: &str,
) -> Result<Option<T>> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let val: Option<String> = conn.get(key).await?;
    Ok(val.and_then(|s| serde_json::from_str(&s).ok()))
}

pub async fn set<T: serde::Serialize>(
    client: &RedisPool,
    key: &str,
    value: &T,
    ttl_secs: u64,
) -> Result<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let serialized = serde_json::to_string(value)?;
    conn.set_ex(key, serialized, ttl_secs).await?;
    Ok(())
}

pub async fn del(client: &RedisPool, key: &str) -> Result<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.del(key).await?;
    Ok(())
}

pub async fn flush_pattern(client: &RedisPool, pattern: &str) -> Result<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let keys: Vec<String> = conn.keys(pattern).await?;
    if !keys.is_empty() {
        conn.del(keys).await?;
    }
    Ok(())
}
```

- [ ] **Step 3: Commit**

```bash
git add backend/server/src/cache/ && git commit -m "feat: add valkey cache layer"
```

---

### Task 8: App state and main.rs

**Files:**
- Create: `backend/server/src/main.rs`

- [ ] **Step 1: Write main.rs**

```rust
// backend/server/src/main.rs
mod api;
mod cache;
mod config;
mod db;
mod error;

use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub cache: cache::RedisPool,
    pub config: Arc<config::Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Arc::new(config::Config::from_env()?);

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    sqlx::migrate!("../migrations").run(&db).await?;

    let cache = redis::Client::open(config.valkey_url.clone())?;

    let state = AppState { db, cache, config: config.clone() };
    let mut router = api::router(state);

    if let Some(ref static_dir) = config.static_dir {
        router = router.fallback_service(ServeDir::new(static_dir));
    }

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
```

- [ ] **Step 2: Commit**

```bash
git add backend/server/src/main.rs && git commit -m "feat: add server main with axum setup"
```

---

### Task 9: API router + auth middleware

**Files:**
- Create: `backend/server/src/api/mod.rs`

- [ ] **Step 1: Write api/mod.rs**

```rust
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
```

- [ ] **Step 2: Create public/mod.rs stub**

```rust
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
```

- [ ] **Step 3: Create manage/mod.rs stub**

```rust
// backend/server/src/api/manage/mod.rs
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
```

- [ ] **Step 4: Commit**

```bash
git add backend/server/src/api/ && git commit -m "feat: add api router with auth middleware"
```

---

### Task 10: Public API handlers

**Files:**
- Create: `backend/server/src/api/public/articles.rs`
- Create: `backend/server/src/api/public/pages.rs`
- Create: `backend/server/src/api/public/tags.rs`
- Create: `backend/server/src/api/public/categories.rs`
- Create: `backend/server/src/api/public/settings.rs`

- [ ] **Step 1: Write failing test for public article list**

```rust
// backend/server/tests/articles_public.rs
mod common;

#[sqlx::test(migrations = "../migrations")]
async fn test_list_returns_only_published(pool: sqlx::SqlitePool) {
    let server = common::spawn_server(pool.clone()).await;
    // create draft article directly in db
    sqlx::query("INSERT INTO articles (slug,title,body,author_name,status) VALUES ('draft-one','Draft','body','Alice','draft')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO articles (slug,title,body,author_name,status,published_at) VALUES ('pub-one','Published','body','Bob','published',datetime('now'))")
        .execute(&pool).await.unwrap();

    let res = server.get("/api/articles").await;
    res.assert_status_ok();
    let body: serde_json::Value = res.json();
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"][0]["article"]["slug"], "pub-one");
}
```

- [ ] **Step 2: Write public/articles.rs**

```rust
// backend/server/src/api/public/articles.rs
use axum::{extract::{Path, Query, State}, routing::get, Json, Router};
use common::ArticleListParams;
use crate::{cache, AppState, error::AppError};
use common::PaginatedArticles;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/articles", get(list_articles))
        .route("/articles/:slug", get(get_article))
}

async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleListParams>,
) -> Result<Json<PaginatedArticles>, AppError> {
    let cache_key = format!("{}:{:?}", cache::keys::ARTICLES_LIST, params);
    if let Ok(Some(cached)) = cache::get::<PaginatedArticles>(&state.cache, &cache_key).await {
        return Ok(Json(cached));
    }
    let (items, total) = crate::db::articles::list(&state.db, &params, Some("published")).await?;
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let result = PaginatedArticles { items, total, page, limit };
    let _ = cache::set(&state.cache, &cache_key, &result, 300).await;
    Ok(Json(result))
}

async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<common::ArticleWithTags>, AppError> {
    let key = cache::keys::article_slug(&slug);
    if let Ok(Some(cached)) = cache::get(&state.cache, &key).await {
        return Ok(Json(cached));
    }
    let article = crate::db::articles::get_by_slug(&state.db, &slug).await?;
    if article.article.status != "published" {
        return Err(AppError::NotFound(format!("article '{slug}' not found")));
    }
    let _ = cache::set(&state.cache, &key, &article, 3600).await;
    Ok(Json(article))
}
```

- [ ] **Step 3: Write remaining public handlers (pages, tags, categories, settings)**

```rust
// backend/server/src/api/public/pages.rs
use axum::{extract::{Path, State}, routing::get, Json, Router};
use crate::{cache, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/pages/:slug", get(get_page))
}

async fn get_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<common::Page>, AppError> {
    let key = cache::keys::page_slug(&slug);
    if let Ok(Some(cached)) = cache::get(&state.cache, &key).await {
        return Ok(Json(cached));
    }
    let page = crate::db::pages::get_by_slug(&state.db, &slug).await?;
    if page.status != "published" {
        return Err(AppError::NotFound(format!("page '{slug}' not found")));
    }
    let _ = cache::set(&state.cache, &key, &page, 3600).await;
    Ok(Json(page))
}
```

```rust
// backend/server/src/api/public/tags.rs
use axum::{extract::{Path, State}, routing::get, Json, Router};
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
    use common::ArticleListParams;
    let params = ArticleListParams { page: None, limit: None, tag: Some(slug), category: None };
    let (items, _) = crate::db::articles::list(&state.db, &params, Some("published")).await?;
    Ok(Json(items))
}
```

```rust
// backend/server/src/api/public/categories.rs
use axum::{extract::State, routing::get, Json, Router};
use crate::{AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/categories", get(list_categories))
}

async fn list_categories(State(state): State<AppState>) -> Result<Json<Vec<common::Category>>, AppError> {
    Ok(Json(crate::db::categories::list(&state.db).await?))
}
```

```rust
// backend/server/src/api/public/settings.rs
use axum::{extract::State, routing::get, Json, Router};
use crate::{AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/settings", get(get_settings))
}

async fn get_settings(State(state): State<AppState>) -> Result<Json<Vec<common::Setting>>, AppError> {
    Ok(Json(crate::db::settings::get_all(&state.db).await?))
}
```

- [ ] **Step 4: Commit**

```bash
git add backend/server/src/api/public/ && git commit -m "feat: add public API handlers"
```

---

### Task 11: Management API handlers

**Files:**
- Create: `backend/server/src/api/manage/articles.rs`
- Create: `backend/server/src/api/manage/pages.rs`
- Create: `backend/server/src/api/manage/tags.rs`
- Create: `backend/server/src/api/manage/categories.rs`
- Create: `backend/server/src/api/manage/settings.rs`

- [ ] **Step 1: Write manage/articles.rs**

```rust
// backend/server/src/api/manage/articles.rs
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::{ArticleListParams, CreateArticle, UpdateArticle};
use crate::{cache, db, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/articles", get(list).post(create))
        .route("/articles/:id", put(update).delete(del))
        .route("/articles/:id/publish", post(publish))
        .route("/articles/:id/archive", post(archive))
}

async fn list(
    State(state): State<AppState>,
    Query(params): Query<ArticleListParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (items, total) = db::articles::list(&state.db, &params, None).await?;
    Ok(Json(serde_json::json!({ "items": items, "total": total })))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateArticle>,
) -> Result<(StatusCode, Json<common::Article>), AppError> {
    let article = db::articles::create(&state.db, body).await?;
    let _ = cache::flush_pattern(&state.cache, &format!("{}*", cache::keys::ARTICLES_LIST)).await;
    Ok((StatusCode::CREATED, Json(article)))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateArticle>,
) -> Result<Json<common::Article>, AppError> {
    let article = db::articles::update(&state.db, id, body).await?;
    let _ = cache::del(&state.cache, &cache::keys::article_slug(&article.slug)).await;
    let _ = cache::flush_pattern(&state.cache, &format!("{}*", cache::keys::ARTICLES_LIST)).await;
    Ok(Json(article))
}

async fn publish(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::Article>, AppError> {
    let article = db::articles::publish(&state.db, id).await?;
    let _ = cache::del(&state.cache, &cache::keys::article_slug(&article.slug)).await;
    let _ = cache::flush_pattern(&state.cache, &format!("{}*", cache::keys::ARTICLES_LIST)).await;
    let _ = cache::del(&state.cache, cache::keys::HOMEPAGE_FEED).await;
    Ok(Json(article))
}

async fn archive(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::Article>, AppError> {
    let article = db::articles::archive(&state.db, id).await?;
    let _ = cache::del(&state.cache, &cache::keys::article_slug(&article.slug)).await;
    let _ = cache::flush_pattern(&state.cache, &format!("{}*", cache::keys::ARTICLES_LIST)).await;
    Ok(Json(article))
}

async fn del(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    db::articles::delete(&state.db, id).await?;
    let _ = cache::flush_pattern(&state.cache, &format!("{}*", cache::keys::ARTICLES_LIST)).await;
    Ok(StatusCode::NO_CONTENT)
}
```

- [ ] **Step 2: Write manage/pages.rs**

```rust
// backend/server/src/api/manage/pages.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::{CreatePage, UpdatePage};
use crate::{cache, db, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pages", get(list).post(create))
        .route("/pages/:id", put(update).delete(del))
        .route("/pages/:id/publish", post(publish))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<common::Page>>, AppError> {
    Ok(Json(db::pages::list(&state.db).await?))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreatePage>,
) -> Result<(StatusCode, Json<common::Page>), AppError> {
    Ok((StatusCode::CREATED, Json(db::pages::create(&state.db, body).await?)))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdatePage>,
) -> Result<Json<common::Page>, AppError> {
    let page = db::pages::update(&state.db, id, body).await?;
    let _ = cache::del(&state.cache, &cache::keys::page_slug(&page.slug)).await;
    Ok(Json(page))
}

async fn publish(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<common::Page>, AppError> {
    let page = db::pages::publish(&state.db, id).await?;
    let _ = cache::del(&state.cache, &cache::keys::page_slug(&page.slug)).await;
    Ok(Json(page))
}

async fn del(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    db::pages::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

- [ ] **Step 3: Write manage/tags.rs, manage/categories.rs, manage/settings.rs**

```rust
// backend/server/src/api/manage/tags.rs
use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Json, Router};
use common::CreateTag;
use crate::{db, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list).post(create))
        .route("/tags/:slug", delete(del))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<common::Tag>>, AppError> {
    Ok(Json(db::tags::list(&state.db).await?))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateTag>,
) -> Result<(StatusCode, Json<common::Tag>), AppError> {
    Ok((StatusCode::CREATED, Json(db::tags::create(&state.db, body).await?)))
}

async fn del(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<StatusCode, AppError> {
    db::tags::delete(&state.db, &slug).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

```rust
// backend/server/src/api/manage/categories.rs
use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post, put}, Json, Router};
use common::{CreateCategory, UpdateCategory};
use crate::{db, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list).post(create))
        .route("/categories/:id", put(update).delete(del))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<common::Category>>, AppError> {
    Ok(Json(db::categories::list(&state.db).await?))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateCategory>,
) -> Result<(StatusCode, Json<common::Category>), AppError> {
    Ok((StatusCode::CREATED, Json(db::categories::create(&state.db, body).await?)))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateCategory>,
) -> Result<Json<common::Category>, AppError> {
    Ok(Json(db::categories::update(&state.db, id, body).await?))
}

async fn del(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<StatusCode, AppError> {
    db::categories::delete(&state.db, &slug).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

```rust
// backend/server/src/api/manage/settings.rs
use axum::{extract::{Path, State}, routing::{get, put}, Json, Router};
use common::UpdateSetting;
use crate::{db, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/settings", get(get_all))
        .route("/settings/:key", put(set))
}

async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<common::Setting>>, AppError> {
    Ok(Json(db::settings::get_all(&state.db).await?))
}

async fn set(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(body): Json<UpdateSetting>,
) -> Result<Json<common::Setting>, AppError> {
    Ok(Json(db::settings::set(&state.db, &key, &body.value).await?))
}
```

- [ ] **Step 4: Compile check**

```bash
cd backend && cargo check -p server
```
Expected: no errors

- [ ] **Step 5: Commit**

```bash
git add backend/server/src/api/manage/ && git commit -m "feat: add management API handlers"
```

---

### Task 12: Test helpers + run tests

**Files:**
- Create: `backend/server/tests/common/mod.rs`

- [ ] **Step 1: Write test helpers**

```rust
// backend/server/tests/common/mod.rs
use axum_test::TestServer;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn spawn_server(pool: SqlitePool) -> TestServer {
    std::env::set_var("CLI_TOKEN", "test-token");
    let cache = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let config = Arc::new(server::config::Config {
        database_url: "".to_string(),
        valkey_url: "redis://127.0.0.1:6379".to_string(),
        cli_token: "test-token".to_string(),
        port: 0,
        static_dir: None,
    });
    let state = server::AppState { db: pool, cache, config };
    let app = server::api::router(state);
    TestServer::new(app).unwrap()
}
```

Note: expose `AppState` and `api::router` as `pub` in `main.rs` and `api/mod.rs`.

- [ ] **Step 2: Make AppState and api::router pub in main.rs**

In `backend/server/src/main.rs`, change:
```rust
pub mod api;
pub mod cache;
pub mod config;
pub mod db;
pub mod error;

#[derive(Clone)]
pub struct AppState { ... }
```

- [ ] **Step 3: Run tests**

```bash
cd backend && cargo test -p server 2>&1
```
Expected: all tests pass (articles_public, articles_manage suites)

- [ ] **Step 4: Commit**

```bash
git add backend/server/tests/ && git commit -m "test: add backend integration tests"
```

---

### Task 13: Full server build verification

- [ ] **Step 1: Build release binary**

```bash
cd backend && cargo build --release -p server
```
Expected: `target/release/server` binary produced, no errors

- [ ] **Step 2: Run all backend tests**

```bash
cd backend && cargo test --workspace 2>&1
```
Expected: all tests pass

- [ ] **Step 3: Commit**

```bash
git commit -m "chore: verify full backend build passes"
```
