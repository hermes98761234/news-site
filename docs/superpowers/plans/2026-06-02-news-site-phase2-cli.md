# News Site — Phase 2: CLI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `news-cli` — a plain Rust CLI binary with all management subcommands calling the Rust REST API, plus the Claude Code skill file.

**Architecture:** Clap-based CLI in the existing Rust workspace (`backend/cli`). Reads config from `~/.config/news-cli/config.toml`. All commands send HTTP requests to the management API with `X-CLI-Token` header.

**Tech Stack:** Rust, clap 4 (derive), reqwest 0.12, serde, tokio, comfy-table, colored, toml

**Prerequisite:** Phase 1 backend must be running (`cargo run -p server`) for integration tests.

---

## File Map

```
backend/cli/
├── Cargo.toml                  (already exists from Phase 1 stub)
└── src/
    ├── main.rs                 # clap root + config loading
    ├── config.rs               # Config struct, load from toml
    ├── client.rs               # HTTP client wrapper (reqwest)
    └── commands/
        ├── mod.rs
        ├── article.rs          # article subcommands
        ├── page.rs             # page subcommands
        ├── tag.rs              # tag subcommands
        ├── category.rs         # category subcommands
        ├── settings.rs         # settings subcommands
        └── cache.rs            # cache subcommands

~/.claude/skills/news-site/
└── SKILL.md                    # Claude Code skill documenting all news-cli commands
```

---

### Task 1: Config loading

**Files:**
- Create: `backend/cli/src/config.rs`

- [ ] **Step 1: Write config.rs**

```rust
// backend/cli/src/config.rs
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub token: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("config not found at {}. Run: news-cli init", path.display()))?;
        toml::from_str(&content).context("invalid config.toml")
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("news-cli")
        .join("config.toml")
}
```

Add `dirs = "5"` to `backend/cli/Cargo.toml` dependencies.

- [ ] **Step 2: Verify compile**

```bash
cd backend && cargo check -p cli
```
Expected: no errors

- [ ] **Step 3: Commit**

```bash
git add backend/cli/src/config.rs backend/cli/Cargo.toml
git commit -m "feat: add cli config loading"
```

---

### Task 2: HTTP client wrapper

**Files:**
- Create: `backend/cli/src/client.rs`

- [ ] **Step 1: Write client.rs**

```rust
// backend/cli/src/client.rs
use anyhow::{bail, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

pub struct ApiClient {
    base_url: String,
    token: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token, client: Client::new() }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let res = self.client
            .get(format!("{}{}", self.base_url, path))
            .header("x-cli-token", &self.token)
            .send().await?;
        self.parse(res).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let res = self.client
            .post(format!("{}{}", self.base_url, path))
            .header("x-cli-token", &self.token)
            .json(body).send().await?;
        self.parse(res).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let res = self.client
            .put(format!("{}{}", self.base_url, path))
            .header("x-cli-token", &self.token)
            .json(body).send().await?;
        self.parse(res).await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let res = self.client
            .delete(format!("{}{}", self.base_url, path))
            .header("x-cli-token", &self.token)
            .send().await?;
        if !res.status().is_success() {
            let text = res.text().await?;
            bail!("API error: {text}");
        }
        Ok(())
    }

    async fn parse<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        let status = res.status();
        let text = res.text().await?;
        if !status.is_success() {
            bail!("API error {status}: {text}");
        }
        serde_json::from_str(&text).map_err(|e| anyhow::anyhow!("parse error: {e}\nbody: {text}"))
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add backend/cli/src/client.rs && git commit -m "feat: add cli http client wrapper"
```

---

### Task 3: Article commands

**Files:**
- Create: `backend/cli/src/commands/article.rs`

- [ ] **Step 1: Write commands/article.rs**

```rust
// backend/cli/src/commands/article.rs
use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;
use comfy_table::{Table, presets::UTF8_FULL};
use crate::client::ApiClient;

#[derive(Subcommand)]
pub enum ArticleCmd {
    /// List articles
    List {
        #[arg(long, default_value = "all")]
        status: String,
    },
    /// Create a new draft article
    Create {
        #[arg(long)] title: String,
        #[arg(long)] author: String,
        #[arg(long)] body: String,
        #[arg(long)] excerpt: Option<String>,
        #[arg(long)] category_id: Option<i64>,
    },
    /// Update an article by ID
    Edit {
        id: i64,
        #[arg(long)] title: Option<String>,
        #[arg(long)] body: Option<String>,
        #[arg(long)] excerpt: Option<String>,
        #[arg(long)] author: Option<String>,
    },
    /// Publish a draft article
    Publish { id: i64 },
    /// Archive a published article
    Archive { id: i64 },
    /// Delete an article
    Delete { id: i64 },
}

pub async fn run(cmd: ArticleCmd, client: &ApiClient) -> Result<()> {
    match cmd {
        ArticleCmd::List { status } => {
            let query = if status == "all" { "".to_string() } else { format!("?status={status}") };
            let res: serde_json::Value = client.get(&format!("/api/manage/articles{query}")).await?;
            let items = res["items"].as_array().unwrap();
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(["ID", "Slug", "Title", "Author", "Status", "Published"]);
            for item in items {
                let a = &item["article"];
                table.add_row([
                    a["id"].to_string(),
                    a["slug"].as_str().unwrap_or("").to_string(),
                    a["title"].as_str().unwrap_or("").to_string(),
                    a["author_name"].as_str().unwrap_or("").to_string(),
                    a["status"].as_str().unwrap_or("").to_string(),
                    a["published_at"].as_str().unwrap_or("-").to_string(),
                ]);
            }
            println!("{table}");
        }
        ArticleCmd::Create { title, author, body, excerpt, category_id } => {
            let res: serde_json::Value = client.post("/api/manage/articles", &serde_json::json!({
                "title": title, "author_name": author, "body": body,
                "excerpt": excerpt, "category_id": category_id,
            })).await?;
            println!("{} Article created: {} (slug: {})",
                "✓".green(), res["title"].as_str().unwrap_or(""), res["slug"].as_str().unwrap_or(""));
        }
        ArticleCmd::Edit { id, title, body, excerpt, author } => {
            let res: serde_json::Value = client.put(&format!("/api/manage/articles/{id}"), &serde_json::json!({
                "title": title, "body": body, "excerpt": excerpt, "author_name": author,
            })).await?;
            println!("{} Article updated: {}", "✓".green(), res["title"].as_str().unwrap_or(""));
        }
        ArticleCmd::Publish { id } => {
            let res: serde_json::Value = client.post(&format!("/api/manage/articles/{id}/publish"), &serde_json::json!({})).await?;
            println!("{} Published: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        ArticleCmd::Archive { id } => {
            let res: serde_json::Value = client.post(&format!("/api/manage/articles/{id}/archive"), &serde_json::json!({})).await?;
            println!("{} Archived: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        ArticleCmd::Delete { id } => {
            client.delete(&format!("/api/manage/articles/{id}")).await?;
            println!("{} Deleted article {id}", "✓".green());
        }
    }
    Ok(())
}
```

- [ ] **Step 2: Commit**

```bash
git add backend/cli/src/commands/article.rs && git commit -m "feat: add article cli commands"
```

---

### Task 4: Page, tag, category, settings, cache commands

**Files:**
- Create: `backend/cli/src/commands/page.rs`
- Create: `backend/cli/src/commands/tag.rs`
- Create: `backend/cli/src/commands/category.rs`
- Create: `backend/cli/src/commands/settings.rs`
- Create: `backend/cli/src/commands/cache.rs`
- Create: `backend/cli/src/commands/mod.rs`

- [ ] **Step 1: Write commands/page.rs**

```rust
// backend/cli/src/commands/page.rs
use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use crate::client::ApiClient;

#[derive(Subcommand)]
pub enum PageCmd {
    List,
    Create { #[arg(long)] title: String, #[arg(long)] slug: String, #[arg(long)] body: Option<String> },
    Edit { id: i64, #[arg(long)] title: Option<String>, #[arg(long)] body: Option<String> },
    Publish { id: i64 },
    Delete { id: i64 },
}

pub async fn run(cmd: PageCmd, client: &ApiClient) -> Result<()> {
    match cmd {
        PageCmd::List => {
            let pages: Vec<serde_json::Value> = client.get("/api/manage/pages").await?;
            for p in &pages {
                println!("{} {} [{}]", p["id"], p["slug"].as_str().unwrap_or(""), p["status"].as_str().unwrap_or(""));
            }
        }
        PageCmd::Create { title, slug, body } => {
            let res: serde_json::Value = client.post("/api/manage/pages", &serde_json::json!({
                "title": title, "slug": slug, "body": body,
            })).await?;
            println!("{} Page created: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        PageCmd::Edit { id, title, body } => {
            let res: serde_json::Value = client.put(&format!("/api/manage/pages/{id}"), &serde_json::json!({
                "title": title, "body": body,
            })).await?;
            println!("{} Page updated: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        PageCmd::Publish { id } => {
            let res: serde_json::Value = client.post(&format!("/api/manage/pages/{id}/publish"), &serde_json::json!({})).await?;
            println!("{} Published page: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        PageCmd::Delete { id } => {
            client.delete(&format!("/api/manage/pages/{id}")).await?;
            println!("{} Deleted page {id}", "✓".green());
        }
    }
    Ok(())
}
```

- [ ] **Step 2: Write commands/tag.rs**

```rust
// backend/cli/src/commands/tag.rs
use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use crate::client::ApiClient;

#[derive(Subcommand)]
pub enum TagCmd {
    List,
    Create { #[arg(long)] name: String },
    Delete { slug: String },
}

pub async fn run(cmd: TagCmd, client: &ApiClient) -> Result<()> {
    match cmd {
        TagCmd::List => {
            let tags: Vec<serde_json::Value> = client.get("/api/manage/tags").await?;
            for t in &tags {
                println!("{} ({})", t["name"].as_str().unwrap_or(""), t["slug"].as_str().unwrap_or(""));
            }
        }
        TagCmd::Create { name } => {
            let res: serde_json::Value = client.post("/api/manage/tags", &serde_json::json!({ "name": name })).await?;
            println!("{} Tag created: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        TagCmd::Delete { slug } => {
            client.delete(&format!("/api/manage/tags/{slug}")).await?;
            println!("{} Deleted tag: {slug}", "✓".green());
        }
    }
    Ok(())
}
```

- [ ] **Step 3: Write commands/category.rs**

```rust
// backend/cli/src/commands/category.rs
use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use crate::client::ApiClient;

#[derive(Subcommand)]
pub enum CategoryCmd {
    List,
    Create { #[arg(long)] name: String, #[arg(long)] slug: Option<String>, #[arg(long)] description: Option<String> },
    Edit { id: i64, #[arg(long)] name: Option<String>, #[arg(long)] description: Option<String> },
    Delete { slug: String },
}

pub async fn run(cmd: CategoryCmd, client: &ApiClient) -> Result<()> {
    match cmd {
        CategoryCmd::List => {
            let cats: Vec<serde_json::Value> = client.get("/api/manage/categories").await?;
            for c in &cats {
                println!("{} {} - {}", c["id"], c["slug"].as_str().unwrap_or(""), c["name"].as_str().unwrap_or(""));
            }
        }
        CategoryCmd::Create { name, slug, description } => {
            let res: serde_json::Value = client.post("/api/manage/categories", &serde_json::json!({
                "name": name, "slug": slug, "description": description,
            })).await?;
            println!("{} Category created: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        CategoryCmd::Edit { id, name, description } => {
            let res: serde_json::Value = client.put(&format!("/api/manage/categories/{id}"), &serde_json::json!({
                "name": name, "description": description,
            })).await?;
            println!("{} Category updated: {}", "✓".green(), res["slug"].as_str().unwrap_or(""));
        }
        CategoryCmd::Delete { slug } => {
            client.delete(&format!("/api/manage/categories/{slug}")).await?;
            println!("{} Deleted category: {slug}", "✓".green());
        }
    }
    Ok(())
}
```

- [ ] **Step 4: Write commands/settings.rs**

```rust
// backend/cli/src/commands/settings.rs
use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use crate::client::ApiClient;

#[derive(Subcommand)]
pub enum SettingsCmd {
    Show,
    Set { key: String, value: String },
}

pub async fn run(cmd: SettingsCmd, client: &ApiClient) -> Result<()> {
    match cmd {
        SettingsCmd::Show => {
            let settings: Vec<serde_json::Value> = client.get("/api/manage/settings").await?;
            for s in &settings {
                println!("{} = {}", s["key"].as_str().unwrap_or(""), s["value"].as_str().unwrap_or(""));
            }
        }
        SettingsCmd::Set { key, value } => {
            let res: serde_json::Value = client.put(
                &format!("/api/manage/settings/{key}"),
                &serde_json::json!({ "value": value }),
            ).await?;
            println!("{} {} = {}", "✓".green(), res["key"].as_str().unwrap_or(""), res["value"].as_str().unwrap_or(""));
        }
    }
    Ok(())
}
```

- [ ] **Step 5: Write commands/cache.rs**

```rust
// backend/cli/src/commands/cache.rs
use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use crate::client::ApiClient;

#[derive(Subcommand)]
pub enum CacheCmd {
    Flush {
        #[arg(long)] key: Option<String>,
    },
}

pub async fn run(cmd: CacheCmd, client: &ApiClient) -> Result<()> {
    match cmd {
        CacheCmd::Flush { key } => {
            let body = serde_json::json!({ "pattern": key });
            let _: serde_json::Value = client.post("/api/manage/cache/flush", &body).await?;
            println!("{} Cache flushed", "✓".green());
        }
    }
    Ok(())
}
```

Also add cache flush endpoint to manage router in Phase 1 server:

```rust
// In backend/server/src/api/manage/mod.rs, add:
mod cache;
// In router(), add:
.merge(cache::router())
```

```rust
// backend/server/src/api/manage/cache.rs
use axum::{extract::State, routing::post, Json, Router};
use crate::{cache, AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/cache/flush", post(flush))
}

async fn flush(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let pattern = body["pattern"].as_str().unwrap_or("*");
    cache::flush_pattern(&state.cache, pattern).await
        .map_err(|e| AppError::Internal(e))?;
    Ok(Json(serde_json::json!({ "flushed": true })))
}
```

- [ ] **Step 6: Write commands/mod.rs**

```rust
// backend/cli/src/commands/mod.rs
pub mod article;
pub mod cache;
pub mod category;
pub mod page;
pub mod settings;
pub mod tag;
```

- [ ] **Step 7: Commit**

```bash
git add backend/cli/src/commands/ backend/server/src/api/manage/cache.rs
git commit -m "feat: add all cli subcommands"
```

---

### Task 5: Main CLI entry point

**Files:**
- Modify: `backend/cli/src/main.rs`

- [ ] **Step 1: Write main.rs**

```rust
// backend/cli/src/main.rs
mod client;
mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{article::ArticleCmd, cache::CacheCmd, category::CategoryCmd,
               page::PageCmd, settings::SettingsCmd, tag::TagCmd};

#[derive(Parser)]
#[command(name = "news-cli", version, about = "News site management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage articles
    Article {
        #[command(subcommand)]
        cmd: ArticleCmd,
    },
    /// Manage pages
    Page {
        #[command(subcommand)]
        cmd: PageCmd,
    },
    /// Manage tags
    Tag {
        #[command(subcommand)]
        cmd: TagCmd,
    },
    /// Manage categories
    Category {
        #[command(subcommand)]
        cmd: CategoryCmd,
    },
    /// Manage settings
    Settings {
        #[command(subcommand)]
        cmd: SettingsCmd,
    },
    /// Manage cache
    Cache {
        #[command(subcommand)]
        cmd: CacheCmd,
    },
    /// Initialize config file
    Init {
        #[arg(long, default_value = "http://localhost:3000")]
        api_url: String,
        #[arg(long)]
        token: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Init { api_url, token } = cli.command {
        let config = config::Config { api_url, token };
        config.save()?;
        println!("Config saved.");
        return Ok(());
    }

    let config = config::Config::load()?;
    let client = client::ApiClient::new(config.api_url, config.token);

    match cli.command {
        Commands::Article { cmd } => commands::article::run(cmd, &client).await?,
        Commands::Page { cmd }    => commands::page::run(cmd, &client).await?,
        Commands::Tag { cmd }     => commands::tag::run(cmd, &client).await?,
        Commands::Category { cmd }=> commands::category::run(cmd, &client).await?,
        Commands::Settings { cmd }=> commands::settings::run(cmd, &client).await?,
        Commands::Cache { cmd }   => commands::cache::run(cmd, &client).await?,
        Commands::Init { .. }     => unreachable!(),
    }
    Ok(())
}
```

- [ ] **Step 2: Build CLI**

```bash
cd backend && cargo build --release -p cli
```
Expected: `target/release/news-cli` binary produced

- [ ] **Step 3: Smoke test**

```bash
./target/release/news-cli --help
./target/release/news-cli article --help
```
Expected: help text printed, no panics

- [ ] **Step 4: Commit**

```bash
git add backend/cli/src/main.rs && git commit -m "feat: add cli main entry point"
```

---

### Task 6: Claude Code skill file

**Files:**
- Create: `~/.claude/skills/news-site/SKILL.md`

- [ ] **Step 1: Create skill directory**

```bash
mkdir -p ~/.claude/skills/news-site
```

- [ ] **Step 2: Write SKILL.md**

```markdown
---
name: news-site
description: Use when managing a news site via news-cli — creating/publishing/archiving articles, managing pages, tags, categories, settings, or flushing cache.
---

# News Site CLI

Manage the news site by running `news-cli` commands. The binary is at `/usr/local/bin/news-cli` (or `./target/release/news-cli` in dev).

## Setup (first time)

```bash
news-cli init --api-url http://localhost:3000 --token <your-token>
```

## Articles

```bash
news-cli article list                              # all articles (any status)
news-cli article list --status draft               # drafts only
news-cli article list --status published
news-cli article create --title "Title" --author "Name" --body "Markdown content" --excerpt "Short summary"
news-cli article edit <id> --title "New title"
news-cli article edit <id> --body "New body content"
news-cli article publish <id>
news-cli article archive <id>
news-cli article delete <id>
```

## Pages

```bash
news-cli page list
news-cli page create --title "About" --slug about --body "Markdown content"
news-cli page edit <id> --body "Updated content"
news-cli page publish <id>
news-cli page delete <id>
```

## Tags

```bash
news-cli tag list
news-cli tag create --name "Rust"
news-cli tag delete <slug>
```

## Categories

```bash
news-cli category list
news-cli category create --name "Technology" --slug tech --description "Tech news"
news-cli category edit <id> --name "New Name"
news-cli category delete <slug>
```

## Settings

```bash
news-cli settings show
news-cli settings set site_name "My News Site"
news-cli settings set site_description "Latest news"
news-cli settings set site_url "https://example.com"
```

## Cache

```bash
news-cli cache flush                   # flush all cache
news-cli cache flush --key "articles*" # flush specific pattern
```

## Common workflows

**Publish a new article:**
1. `news-cli article list --status draft` — find the article ID
2. `news-cli article publish <id>`

**Create and immediately publish:**
1. `news-cli article create --title "..." --author "..." --body "..."`
2. Note the returned ID
3. `news-cli article publish <id>`

**Update site name:**
`news-cli settings set site_name "New Site Name"`
```

- [ ] **Step 3: Commit**

```bash
git add ~/.claude/skills/news-site/SKILL.md
git commit -m "feat: add news-site claude code skill"
```
