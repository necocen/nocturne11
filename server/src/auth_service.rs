use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use std::{
    future::{ready, Future, Ready},
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Clone, Debug)]
pub struct AuthService<D: Clone + 'static, F: Fn(&D, &str) -> bool + Clone + 'static> {
    is_authorized: F,
    phantom: PhantomData<D>,
}

impl<D: Clone + 'static, F: Fn(&D, &str) -> bool + Clone + 'static> AuthService<D, F> {
    pub fn new(is_authorized: F) -> AuthService<D, F> {
        AuthService {
            is_authorized,
            phantom: PhantomData::default(),
        }
    }
}

impl<D: Clone + 'static, S, F: Fn(&D, &str) -> bool + Clone + 'static, B: 'static>
    Transform<S, ServiceRequest> for AuthService<D, F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthServiceMiddleware<D, S, F>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthServiceMiddleware {
            service,
            auth: self.clone(),
        }))
    }
}

pub struct AuthServiceMiddleware<D: Clone + 'static, S, F: Fn(&D, &str) -> bool + Clone + 'static> {
    service: S,
    auth: AuthService<D, F>,
}

type ServiceResult<B> = Result<ServiceResponse<B>, Error>;

impl<S, D: Clone + 'static, F: Fn(&D, &str) -> bool + Clone + 'static, B: 'static>
    Service<ServiceRequest> for AuthServiceMiddleware<D, S, F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = ServiceResult<B>>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match (req.get_identity(), req.app_data::<Data<D>>()) {
            (Some(ref id), Some(data)) if (self.auth.is_authorized)(data, id) => {
                Box::pin(self.service.call(req))
            }
            _ => Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                "Unauthorized",
            )))),
        }
    }
}
