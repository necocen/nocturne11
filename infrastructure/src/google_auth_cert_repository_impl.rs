use anyhow::Context;
use domain::repositories::google_auth_cert::{Error, GoogleAuthCertRepository, Result};
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
        let url = url::Url::parse(URL).with_context(|| format!("URL parse error: '{}'", URL))?;
        let keys = reqwest::get(url)
            .await
            .context("Failed to get certs from googleapis.com")?
            .json::<Keys>()
            .await
            .context("Failed to parse cert JSON")?;
        let key = keys
            .keys
            .into_iter()
            .find(|key| key.kid == kid)
            .ok_or_else(|| Error::NotFound(kid.to_owned()))?;
        Ok((key.n, key.e))
    }
}
