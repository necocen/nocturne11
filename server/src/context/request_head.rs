use super::AppContext;
use actix_web::dev::RequestHead;

pub trait RequestHeadContext {
    fn is_authorized(&self) -> bool;
}

impl RequestHeadContext for RequestHead {
    fn is_authorized(&self) -> bool {
        if let Some(item) = self.extensions().get::<AppContext>() {
            item.is_authorized
        } else {
            false
        }
    }
}
