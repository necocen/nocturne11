use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use std::{future::{ready, Future, Ready}, pin::Pin, rc::Rc, task::{Context, Poll}};

#[derive(Clone)]
pub struct AuthService<D: Clone + 'static> {
    is_authorized: Rc<dyn Fn(&D, &str) -> bool + 'static>,
}

impl<D: Clone + 'static> AuthService<D> {
    pub fn new(is_authorized: impl Fn(&D, &str) -> bool + 'static) -> AuthService<D> {
        AuthService {
            is_authorized: Rc::new(is_authorized),
        }
    }
}

impl<D: Clone + 'static, S, B: 'static> Transform<S, ServiceRequest> for AuthService<D>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthServiceMiddleware<D, S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthServiceMiddleware {
            service,
            auth: self.clone(),
        }))
    }
}

pub struct AuthServiceMiddleware<D: Clone + 'static, S> {
    service: S,
    auth: AuthService<D>,
}

type ServiceResult<B> = Result<ServiceResponse<B>, Error>;

impl<S, D: Clone + 'static, B: 'static> Service<ServiceRequest> for AuthServiceMiddleware<D, S>
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
