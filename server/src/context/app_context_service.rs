use super::AppContext;
use crate::Service;
use actix_identity::IdentityExt as _;
use actix_session::SessionExt;
use actix_web::{
    dev::{Service as ActixService, ServiceRequest, ServiceResponse, Transform},
    error::ErrorInternalServerError,
    web::Data,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    task::{Context as TaskContext, Poll},
};

#[derive(Clone)]
pub struct AppContextService;

impl<S, B: 'static> Transform<S, ServiceRequest> for AppContextService
where
    S: ActixService<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ContextServiceMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ContextServiceMiddleware { service }))
    }
}

pub struct ContextServiceMiddleware<S> {
    service: S,
}

impl<S, B: 'static> ActixService<ServiceRequest> for ContextServiceMiddleware<S>
where
    S: ActixService<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut TaskContext) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(app) = req.app_data::<Data<Service>>() {
            let message = req
                .get_session()
                .remove_as::<String>("message")
                .and_then(Result::ok);
            let is_authorized = matches!(req.get_identity().and_then(|id| id.id()), Ok(ref id) if app.authorize(id));
            let is_development = app.is_development;
            req.extensions_mut().insert(AppContext {
                is_authorized,
                is_development,
                config: app.config.clone(),
                message,
            });
            Box::pin(self.service.call(req))
        } else {
            Box::pin(ready(Err(ErrorInternalServerError(
                "Couldn't extract Service from Request.",
            ))))
        }
    }
}
