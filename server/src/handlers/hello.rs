use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{web, HttpResponse};
use askama::Template;
use domain::entities::Post;
use domain::use_cases::get_posts;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    title: &'a str,
    posts: Vec<Post>,
}

pub async fn hello(server: web::Data<Server>) -> Result<HttpResponse, Error> {
    HelloTemplate {
        posts: get_posts(&server.posts_repository)?,
        title: "タイトル",
    }
    .to_response()
}
