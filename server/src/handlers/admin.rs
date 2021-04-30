use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{http::header, web, HttpResponse};
use chrono::Utc;
use domain::{entities::NewPost, repositories::posts::PostsRepository};
use serde::Deserialize;
use templates::NewPostTemplate;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FormParams {
    title: String,
    body: String,
}

pub(super) async fn new_post_form(_server: web::Data<Server>) -> Result<HttpResponse, Error> {
    NewPostTemplate { title: "投稿" }.to_response()
}

pub(super) async fn create(
    server: web::Data<Server>,
    form: web::Form<FormParams>,
) -> Result<HttpResponse, Error> {
    server.posts_repository.create(NewPost {
        title: form.title.clone(),
        body: form.body.clone(),
        created_at: Utc::now(),
    })?;
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
