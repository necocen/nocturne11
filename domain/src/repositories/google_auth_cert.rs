use anyhow::Result;

#[async_trait::async_trait]
pub trait GoogleAuthCertRepository {
    async fn key(&self, kid: &str) -> Result<(String, String)>;
}
