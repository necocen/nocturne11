use anyhow::{Context, Result};
use domain::repositories::google_auth_cert::GoogleAuthCertRepository;
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct GoogleAuthCertRepositoryImpl {}

#[derive(Debug, Clone, Deserialize)]
struct Key {
    e: String,
    n: String,
    kid: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Keys {
    keys: Vec<Key>,
}

#[async_trait::async_trait]
impl GoogleAuthCertRepository for GoogleAuthCertRepositoryImpl {
    async fn key(&self, kid: &str) -> Result<(String, String)> {
        const URL: &str = "https://www.googleapis.com/oauth2/v3/certs";
        let url = url::Url::parse(URL)?;
        let keys = reqwest::get(url).await?.json::<Keys>().await?;
        let key = keys
            .keys
            .into_iter()
            .find(|key| key.kid == kid)
            .context("No certificate with given kid was found.")?;
        Ok((key.n, key.e))
    }
}
