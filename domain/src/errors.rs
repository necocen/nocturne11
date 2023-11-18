use crate::repositories::config::Error as ConfigError;
use crate::repositories::export_posts::Error as ExportError;
use crate::repositories::google_auth_cert::Error as CertError;
use crate::repositories::posts::Error as PostsError;
use crate::repositories::search::Error as SearchError;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not Found")]
    NotFound,
    #[error(transparent)]
    Posts(#[from] PostsError),
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Export(#[from] ExportError),
    #[error(transparent)]
    Jwt(#[from] JwtError),
    #[error("Unexpected JWT issuer: {0}")]
    JwtIssuer(String),
    #[error(transparent)]
    Cert(#[from] CertError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
