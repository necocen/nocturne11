use super::AppContext;
use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
    task::{Context as TaskContext, Poll},
};

#[derive(Clone)]
pub struct AppContextService<D> {
    is_authorized: Rc<dyn Fn(&D, &str) -> bool + 'static>,
}

impl<D> AppContextService<D> {
    pub fn new(is_authorized: impl Fn(&D, &str) -> bool + 'static) -> AppContextService<D> {
        AppContextService {
            is_authorized: Rc::new(is_authorized),
        }
    }
}

impl<S, D: 'static, B: 'static> Transform<S, ServiceRequest> for AppContextService<D>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ContextServiceMiddleware<S, D>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ContextServiceMiddleware {
            service: Rc::new(service),
            is_authorized: self.is_authorized.clone(),
        }))
    }
}

pub struct ContextServiceMiddleware<S, D: 'static> {
    service: Rc<S>,
    is_authorized: Rc<dyn Fn(&D, &str) -> bool + 'static>,
}

impl<S, D: 'static, B: 'static> Service<ServiceRequest> for ContextServiceMiddleware<S, D>
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
        match (req.get_identity(), req.app_data::<Data<D>>()) {
            (Some(ref id), Some(data)) if (self.is_authorized)(data, id) => {
                req.extensions_mut().insert(AppContext {
                    title: "andante".to_owned(),
                    is_authorized: true,
                });
            }
            _ => {
                req.extensions_mut().insert(AppContext {
                    title: "andante".to_owned(),
                    is_authorized: false,
                });
            }
        };
        Box::pin(self.service.call(req))
    }
}
