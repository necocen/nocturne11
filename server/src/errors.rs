use actix_web::ResponseError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    General(#[from] anyhow::Error),
    #[error(transparent)]
    Domain(#[from] domain::Error),
    #[error(transparent)]
    Askama(#[from] askama::Error),
}
impl ResponseError for Error {}
