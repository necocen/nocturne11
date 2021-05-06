use super::context_service::ContextItem;
use actix_web::dev::RequestHead;

pub trait RequestHeadContext {
    fn is_authorized(&self) -> bool;
}

impl RequestHeadContext for RequestHead {
    fn is_authorized(&self) -> bool {
        if let Some(item) = self.extensions().get::<ContextItem>() {
            item.is_authorized
        } else {
            false
        }
    }
}
