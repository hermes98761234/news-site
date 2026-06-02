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
