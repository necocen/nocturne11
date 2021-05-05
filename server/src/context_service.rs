use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Extensions, Payload, RequestHead, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
    task::{Context as TaskContext, Poll},
};

#[derive(Clone)]
pub struct ContextService<D: 'static> {
    is_authorized: Rc<dyn Fn(&D, &str) -> bool + 'static>,
}

#[derive(Clone, Debug)]
struct ContextItem {
    pub field: u8,
    pub is_authorized: bool,
}

pub trait RequestContext {
    fn is_authorized(&self) -> bool;
}

impl<T> RequestContext for T
where
    T: HttpMessage,
{
    fn is_authorized(&self) -> bool {
        Context::get_is_authorized(&self.extensions())
    }
}
pub trait RequestHeadContext {
    fn is_authorized(&self) -> bool;
}
impl RequestHeadContext for RequestHead {
    fn is_authorized(&self) -> bool {
        Context::get_is_authorized(&self.extensions())
    }
}

#[derive(Clone, Debug)]
pub struct Context(HttpRequest);

impl Context {
    pub fn is_authorized(&self) -> bool {
        Context::get_is_authorized(&self.0.extensions())
    }

    fn get_is_authorized(extensions: &Extensions) -> bool {
        if let Some(item) = extensions.get::<ContextItem>() {
            item.is_authorized
        } else {
            false
        }
    }
}
impl FromRequest for Context {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Context, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(Context(req.clone())))
    }
}

impl<D: 'static> ContextService<D> {
    pub fn new(is_authorized: impl Fn(&D, &str) -> bool + 'static) -> ContextService<D> {
        ContextService {
            is_authorized: Rc::new(is_authorized),
        }
    }
}

impl<S, D: 'static, B: 'static> Transform<S, ServiceRequest> for ContextService<D>
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
                req.extensions_mut().insert(ContextItem {
                    field: 33,
                    is_authorized: true,
                });
            }
            _ => {
                req.extensions_mut().insert(ContextItem {
                    field: 33,
                    is_authorized: false,
                });
            }
        };
        Box::pin(self.service.call(req))
    }
}
