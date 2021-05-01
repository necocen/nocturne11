use super::{
    args::{CreateFormParams, DeleteFormParams, IdArguments},
    Error, TemplateToResponse,
};
use crate::server::Server;
use actix_web::{http::header, web, HttpResponse};
use chrono::Utc;
use domain::{
    entities::NewPost,
    repositories::posts::PostsRepository,
    use_cases::{create_post, delete_post},
};
use templates::{EditPostTemplate, NewPostTemplate};

pub(super) async fn new_post_form(_server: web::Data<Server>) -> Result<HttpResponse, Error> {
    NewPostTemplate { title: "投稿" }.to_response()
}

pub(super) async fn edit_post_form(
    server: web::Data<Server>,
    args: web::Query<IdArguments>,
) -> Result<HttpResponse, Error> {
    let post = &server.posts_repository.get(args.id)?;
    EditPostTemplate {
        title: "編集",
        post,
    }
    .to_response()
}

pub(super) async fn create(
    server: web::Data<Server>,
    form: web::Form<CreateFormParams>,
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

pub(super) async fn delete(
    server: web::Data<Server>,
    form: web::Form<DeleteFormParams>,
) -> Result<HttpResponse, Error> {
    delete_post(&server.posts_repository, &server.search_repository, form.id).await?;
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

mod templates {
    use domain::entities::Post;

    #[derive(askama::Template)]
    #[template(path = "admin/new.html")]
    pub(super) struct NewPostTemplate<'a> {
        pub(super) title: &'a str,
    }

    #[derive(askama::Template)]
    #[template(path = "admin/edit.html")]
    pub(super) struct EditPostTemplate<'a> {
        pub(super) title: &'a str,
        pub(super) post: &'a Post,
    }
}
