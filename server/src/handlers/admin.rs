use super::args::{CreateFormParams, DeleteFormParams, IdArguments, UpdateFormParams};
use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use crate::{Error, Service};
use actix_web::{http::header, web, HttpResponse};
use chrono::Utc;
use domain::{
    entities::NewPost,
    use_cases::{create_post, delete_post, get_post_with_id, update_post},
};
use templates::{AdminIndexTemplate, EditPostTemplate, NewPostTemplate};

pub async fn index(context: AppContext) -> Result<HttpResponse, Error> {
    AdminIndexTemplate { context }.to_response()
}

pub async fn new_post_form(context: AppContext) -> Result<HttpResponse, Error> {
    NewPostTemplate { context }.to_response()
}

pub async fn edit_post_form(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Query<IdArguments>,
) -> Result<HttpResponse, Error> {
    let page = get_post_with_id(&service.posts_repository, &args.id)?;
    let post = page.post().unwrap();
    EditPostTemplate { context, post }.to_response()
}

pub async fn create(
    service: web::Data<Service>,
    form: web::Form<CreateFormParams>,
) -> Result<HttpResponse, Error> {
    let new_post = NewPost::new(&form.title, &form.body, Utc::now());
    create_post(
        &service.posts_repository,
        &service.search_repository,
        &new_post,
    )
    .await?;
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

pub async fn update(
    service: web::Data<Service>,
    form: web::Form<UpdateFormParams>,
) -> Result<HttpResponse, Error> {
    let new_post = NewPost::new(&form.title, &form.body, Utc::now());
    update_post(
        &service.posts_repository,
        &service.search_repository,
        form.id,
        &new_post,
    )
    .await?;
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, format!("/{}", form.id)))
        .finish())
}

pub async fn delete(
    service: web::Data<Service>,
    form: web::Form<DeleteFormParams>,
) -> Result<HttpResponse, Error> {
    delete_post(
        &service.posts_repository,
        &service.search_repository,
        form.id,
    )
    .await?;
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
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
}
