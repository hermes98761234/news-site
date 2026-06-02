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
