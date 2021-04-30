use super::errors::Error;
use actix_web::HttpResponse;
use bytes::BytesMut;

pub(super) trait TemplateToResponse {
    fn to_response(&self) -> Result<HttpResponse, Error>;
}

impl<T: askama::Template> TemplateToResponse for T {
    fn to_response(&self) -> Result<HttpResponse, Error> {
        let mut buffer = BytesMut::with_capacity(self.size_hint());
        self.render_into(&mut buffer)?;

        let content_type =
            askama::mime::extension_to_mime_type(self.extension().unwrap_or("txt")).to_string();
        Ok(HttpResponse::Ok()
            .content_type(content_type.as_str())
            .body(buffer.freeze()))
    }
}
