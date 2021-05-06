mod app_context_service;
mod request_head;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
pub use app_context_service::AppContextService;
pub use request_head::RequestHeadContext;
use std::future::{ready, Ready};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub title: String,
    pub is_authorized: bool,
}

impl FromRequest for AppContext {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<AppContext, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // TODO: unwrapはちゃんとハンドルする
        ready(Ok(req.extensions().get::<AppContext>().unwrap().clone()))
    }
}
