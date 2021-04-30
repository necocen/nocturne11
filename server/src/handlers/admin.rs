use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{http::header, web, HttpResponse};
use chrono::Utc;
use domain::{entities::NewPost, use_cases::create_post};
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
    let new_post = NewPost {
        title: form.title.clone(),
        body: form.body.clone(),
        created_at: Utc::now(),
    };
    create_post(
        &server.posts_repository,
        &server.search_repository,
        &new_post,
    )
    .await?;
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
