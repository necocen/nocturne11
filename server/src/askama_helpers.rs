use crate::{post::Body, Error};
use actix_web::HttpResponse;
use bytes::BytesMut;

pub trait TemplateToResponse {
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

pub mod filters {
    use chrono::{DateTime, Local, Utc};

    pub fn format_date(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date.with_timezone(&Local).format("%F %T").to_string())
    }

    pub fn iso8601(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date.with_timezone(&Local).to_rfc3339())
    }
}

pub fn convert_body(body: &str) -> String {
    Body::new(body).to_html()
}
