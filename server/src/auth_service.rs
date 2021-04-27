use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Clone, Debug)]
pub struct AuthService<F: Fn(&str) -> bool + Clone + 'static> {
    pub is_authorized: F,
}

impl<F: Fn(&str) -> bool + Clone + 'static> AuthService<F> {
    pub fn new(is_authorized: F) -> AuthService<F> {
        AuthService { is_authorized }
    }
}

impl<S, F: Fn(&str) -> bool + Clone + 'static, B: 'static> Transform<S, ServiceRequest>
    for AuthService<F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthServiceMiddleware<S, F>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthServiceMiddleware {
            service,
            auth: self.clone(),
        }))
    }
}

pub struct AuthServiceMiddleware<S, F: Fn(&str) -> bool + Clone + 'static> {
    service: S,
    auth: AuthService<F>,
}

impl<S, F: Fn(&str) -> bool + Clone + 'static, B: 'static> Service<ServiceRequest>
    for AuthServiceMiddleware<S, F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match req.get_identity() {
            Some(ref id) if (self.auth.is_authorized)(id) => Box::pin(self.service.call(req)),
            _ => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                "Unauthorized",
            )))),
        }
    }
}
