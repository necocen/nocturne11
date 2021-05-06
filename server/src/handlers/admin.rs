use super::args::{
    ConfigFormParams, CreateFormParams, DeleteFormParams, IdArguments, UpdateFormParams,
};
use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use crate::{Error, Server};
use actix_web::{http::header, web, HttpResponse};
use anyhow::anyhow;
use chrono::Utc;
use domain::{
    entities::NewPost,
    use_cases::{create_post, delete_post, get_post_with_id, set_about, update_post},
};
use templates::{AdminIndexTemplate, ConfigTemplate, EditPostTemplate, NewPostTemplate};

pub async fn index(context: AppContext) -> Result<HttpResponse, Error> {
    AdminIndexTemplate { context }.to_response()
}

pub async fn new_post_form(context: AppContext) -> Result<HttpResponse, Error> {
    NewPostTemplate { context }.to_response()
}

pub async fn edit_post_form(
    context: AppContext,
    server: web::Data<Server>,
    args: web::Query<IdArguments>,
) -> Result<HttpResponse, Error> {
    let page = get_post_with_id(&server.posts_repository, &args.id)?;
    let post = page
        .post()
        .ok_or_else(|| Error::General(anyhow!("Post not found")))?;
    EditPostTemplate { context, post }.to_response()
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

pub async fn show_config(context: AppContext) -> Result<HttpResponse, Error> {
    ConfigTemplate { context }.to_response()
}

pub async fn update_config(
    server: web::Data<Server>,
    form: web::Form<ConfigFormParams>,
) -> Result<HttpResponse, Error> {
    set_about(&server.config_repository, &form.about)?;
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/about"))
        .finish())
}

mod templates {
    use crate::context::AppContext;
    use askama::Template;
    use domain::entities::Post;

    #[derive(Template)]
    #[template(path = "admin.html")]
    pub struct AdminIndexTemplate {
        pub context: AppContext,
    }

    #[derive(Template)]
    #[template(path = "admin/new.html")]
    pub struct NewPostTemplate {
        pub context: AppContext,
    }

    #[derive(Template)]
    #[template(path = "admin/edit.html")]
    pub struct EditPostTemplate<'a> {
        pub context: AppContext,
        pub post: &'a Post,
    }

    #[derive(Template)]
    #[template(path = "admin/config.html")]
    pub struct ConfigTemplate {
        pub context: AppContext,
    }
}
