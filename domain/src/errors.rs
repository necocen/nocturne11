use crate::repositories::config::Error as ConfigError;
use crate::repositories::export_posts::Error as ExportError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not Found")]
    NotFound,
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Export(#[from] ExportError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
