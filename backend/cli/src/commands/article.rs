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
