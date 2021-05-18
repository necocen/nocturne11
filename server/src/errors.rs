use actix_web::{
    dev::HttpResponseBuilder,
    http::{header, StatusCode},
    HttpResponse, ResponseError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Domain(#[from] domain::Error),
    #[error(transparent)]
    Askama(#[from] askama::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use domain::repositories::posts::Error::NotFound;
        use domain::Error::Posts;
        if let Self::Domain(Posts(NotFound(_))) = self {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
            .body(self.to_string())
        // TODO: log
    }
}
