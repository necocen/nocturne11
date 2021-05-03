use super::args::{CreateFormParams, DeleteFormParams, IdArguments, UpdateFormParams};
use crate::askama_helpers::TemplateToResponse;
use crate::{Error, Server};
use actix_web::{http::header, web, HttpResponse};
use chrono::Utc;
use domain::{
    entities::NewPost,
    repositories::posts::PostsRepository,
    use_cases::{create_post, delete_post, update_post},
};
use templates::{AdminIndexTemplate, EditPostTemplate, NewPostTemplate};

pub async fn index() -> Result<HttpResponse, Error> {
    AdminIndexTemplate { title: "admin" }.to_response()
}

pub async fn new_post_form(_server: web::Data<Server>) -> Result<HttpResponse, Error> {
    NewPostTemplate { title: "投稿" }.to_response()
}

pub async fn edit_post_form(
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

pub async fn create(
    server: web::Data<Server>,
    form: web::Form<CreateFormParams>,
) -> Result<HttpResponse, Error> {
    let new_post = NewPost::new(&form.title, &form.body, Utc::now());
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

pub async fn update(
    server: web::Data<Server>,
    form: web::Form<UpdateFormParams>,
) -> Result<HttpResponse, Error> {
    let new_post = NewPost::new(&form.title, &form.body, Utc::now());
    update_post(
        &server.posts_repository,
        &server.search_repository,
        form.id,
        &new_post,
    )
    .await?;
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, format!("/{}", form.id)))
        .finish())
}

pub async fn delete(
    server: web::Data<Server>,
    form: web::Form<DeleteFormParams>,
) -> Result<HttpResponse, Error> {
    delete_post(&server.posts_repository, &server.search_repository, form.id).await?;
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

mod templates {
    use askama::Template;
    use domain::entities::Post;

    #[derive(Template)]
    #[template(path = "admin.html")]
    pub struct AdminIndexTemplate<'a> {
        pub title: &'a str,
    }

    #[derive(Template)]
    #[template(path = "admin/new.html")]
    pub struct NewPostTemplate<'a> {
        pub title: &'a str,
    }

    #[derive(Template)]
    #[template(path = "admin/edit.html")]
    pub struct EditPostTemplate<'a> {
        pub title: &'a str,
        pub post: &'a Post,
    }
}
