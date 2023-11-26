mod app_context_service;
use actix_web::{
    dev::Payload, error::ErrorInternalServerError, Error, FromRequest, HttpMessage, HttpRequest,
};
pub use app_context_service::AppContextService;
use domain::entities::config::Config;
use std::future::{ready, Ready};

#[derive(Clone, Debug)]
pub struct AppContext {
    // TODO: AppContextProvider traitを定義して、UseCaseから参照するようにする
    pub is_authorized: bool,
    pub is_development: bool,
    pub config: Config,
    pub message: Option<String>,
}

impl FromRequest for AppContext {
    type Error = Error;
    type Future = Ready<Result<AppContext, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<AppContext>()
                .cloned()
                .ok_or_else(|| {
                    ErrorInternalServerError("Couldn't extract AppContext from Request.")
                }),
        )
    }
}
