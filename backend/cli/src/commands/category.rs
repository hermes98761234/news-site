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
