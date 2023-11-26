#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("Not Found")]
    PostNotFound,
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
