use actix_web::ResponseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum Error {
    #[error(transparent)]
    General(#[from] anyhow::Error),
    #[error(transparent)]
    Askama(#[from] askama::Error),
}
impl ResponseError for Error {}
