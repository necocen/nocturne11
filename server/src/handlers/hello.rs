use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
    title: &'a str,
}

pub async fn hello(server: web::Data<Server>) -> Result<HttpResponse, Error> {
    HelloTemplate {
        name: "ねこねこ",
        title: "タイトル",
    }
    .to_response()
}
