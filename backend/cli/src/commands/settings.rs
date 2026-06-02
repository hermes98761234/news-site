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
