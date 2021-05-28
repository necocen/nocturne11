use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Domain(#[from] domain::Error),
    #[error("{0}")]
    NoResult(String),
    #[error(transparent)]
    Askama(#[from] askama::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
