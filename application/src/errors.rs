#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("Not Found")]
    PostNotFound,
    #[error("Invalid YearMonth")]
    InvalidYearMonth,
    #[error("Invalid PageNumber")]
    InvalidPageNumber,
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
