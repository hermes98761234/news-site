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
