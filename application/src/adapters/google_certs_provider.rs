#[async_trait::async_trait]
pub trait GoogleCertsProvider {
    async fn get_by_key(&self, key: &str) -> anyhow::Result<(String, String)>;
}
