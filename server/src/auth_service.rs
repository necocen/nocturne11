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
pub struct AuthService {
    admin_user: String,
}

impl AuthService {
    pub fn new(admin_user: impl Into<String>) -> AuthService {
        AuthService {
            admin_user: admin_user.into(),
        }
    }
}

impl<S, B: 'static> Transform<S, ServiceRequest> for AuthService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthServiceMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthServiceMiddleware {
            service,
            auth: self.clone(),
        }))
    }
}

pub struct AuthServiceMiddleware<S> {
    service: S,
    auth: AuthService,
}
impl<S, B: 'static> Service<ServiceRequest> for AuthServiceMiddleware<S>
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
            Some(id) if id == self.auth.admin_user => Box::pin(self.service.call(req)),
            _ => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                "Unauthorized",
            )))),
        }
    }
}
