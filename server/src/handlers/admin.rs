use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{http::header, web, HttpResponse};
use serde::Deserialize;
use templates::NewPostTemplate;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FormParams {
    title: String,
}

pub(super) async fn new_post_form(_server: web::Data<Server>) -> Result<HttpResponse, Error> {
    NewPostTemplate { title: "投稿" }.to_response()
}

pub(super) async fn create(
    _server: web::Data<Server>,
    form: web::Form<FormParams>,
) -> Result<HttpResponse, Error> {
    dbg!(form);
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

mod templates {
    #[derive(askama::Template)]
    #[template(path = "admin/new.html")]
    pub(super) struct NewPostTemplate<'a> {
        pub(super) title: &'a str,
    }
}
