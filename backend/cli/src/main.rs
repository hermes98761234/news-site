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
        base_url: String,
        #[arg(long)]
        token: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Init { base_url, token } = cli.command {
        let cfg = config::Config { base_url, token };
        let path = config::config_path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, toml::to_string_pretty(&cfg)?)?;
        println!("Config saved to {}", path.display());
        return Ok(());
    }

    let config = config::Config::load()?;
    let client = client::ApiClient::new(config.base_url, config.token);

    match cli.command {
        Commands::Article { cmd }    => commands::article::run(cmd, &client).await?,
        Commands::Page { cmd }       => commands::page::run(cmd, &client).await?,
        Commands::Tag { cmd }        => commands::tag::run(cmd, &client).await?,
        Commands::Category { cmd }   => commands::category::run(cmd, &client).await?,
        Commands::Settings { cmd }   => commands::settings::run(cmd, &client).await?,
        Commands::Cache { cmd }      => commands::cache::run(cmd, &client).await?,
        Commands::Init { .. }        => unreachable!(),
    }
    Ok(())
}
