mod context_service;
use context_service::ContextItem;
pub use context_service::ContextService;
mod request_head;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
pub use request_head::RequestHeadContext;
use std::future::{ready, Ready};

#[derive(Clone, Debug)]
pub struct Context {
    pub field: u8,
    pub is_authorized: bool,
}

impl FromRequest for Context {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Context, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let ContextItem {
            field,
            is_authorized,
        } = req.extensions().get::<ContextItem>().unwrap().clone();
        ready(Ok(Context {
            field,
            is_authorized,
        }))
    }
}
