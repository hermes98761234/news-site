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
