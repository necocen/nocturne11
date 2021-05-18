use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cert for given kid '{0}' was not found")]
    NotFound(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait::async_trait]
pub trait GoogleAuthCertRepository {
    async fn key(&self, kid: &str) -> Result<(String, String)>;
}
