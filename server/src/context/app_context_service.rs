use super::AppContext;
use crate::Error as AppError;
use crate::Server;
use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorInternalServerError,
    web::Data,
    Error, HttpMessage,
};
use domain::use_cases::get_config;
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    task::{Context as TaskContext, Poll},
};

#[derive(Clone)]
pub struct AppContextService;

impl<S, B: 'static> Transform<S, ServiceRequest> for AppContextService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
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

impl<S, B: 'static> Service<ServiceRequest> for ContextServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut TaskContext) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(app) = req.app_data::<Data<Server>>() {
            let is_authorized = matches!(req.get_identity(), Some(ref id) if app.authorize(id));
            match get_config(&app.config_repository) {
                Ok(config) => {
                    req.extensions_mut().insert(AppContext {
                        config,
                        is_authorized,
                    });
                    Box::pin(self.service.call(req))
                }
                Err(e) => Box::pin(ready(Err(AppError::General(e).into()))),
            }
        } else {
            Box::pin(ready(Err(ErrorInternalServerError(
                "Couldn't extract Server from Request.",
            ))))
        }
    }
}
