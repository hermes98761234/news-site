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
