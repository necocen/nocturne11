mod app_context_service;
mod request_head;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
pub use app_context_service::AppContextService;
pub use request_head::RequestHeadContext;
use std::future::{ready, Ready};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub field: u8,
    pub is_authorized: bool,
}

impl FromRequest for AppContext {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<AppContext, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let AppContext {
            field,
            is_authorized,
        } = req.extensions().get::<AppContext>().unwrap().clone();
        ready(Ok(AppContext {
            field,
            is_authorized,
        }))
    }
}
