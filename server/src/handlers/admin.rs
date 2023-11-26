use super::args::{CreateFormParams, DeleteFormParams, IdArguments, UpdateFormParams};
use crate::context::AppContext;
use crate::{Error, Service};
use actix_session::Session;
use actix_web::{http::header, web, HttpResponse};
use application::use_cases::{
    CreateNewPostUseCase, DeletePostUseCase, GetPostByIdUseCase, UpdatePostUseCase,
};
use askama_actix::TemplateToResponse;
use chrono::Utc;
use domain::entities::{NewPost, PostId};
use templates::{AdminIndexTemplate, EditPostTemplate, NewPostTemplate};

pub async fn index(context: AppContext) -> Result<HttpResponse, Error> {
    Ok(AdminIndexTemplate { context }.to_response())
}

pub async fn new_post_form(context: AppContext) -> Result<HttpResponse, Error> {
    Ok(NewPostTemplate { context }.to_response())
}

pub async fn edit_post_form(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Query<IdArguments>,
) -> Result<HttpResponse, Error> {
    let post_id = PostId(args.id);
    let post =
        GetPostByIdUseCase::execute(&service.posts_repository, &service.search_client, &post_id)
            .await?
            .post()?;
    Ok(EditPostTemplate { context, post }.to_response())
}

pub async fn create(
    service: web::Data<Service>,
    form: web::Form<CreateFormParams>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let new_post = NewPost::new(&form.title, &form.body, Utc::now());
    CreateNewPostUseCase::execute(&service.posts_repository, &service.search_client, new_post)
        .await?;
    session.insert("message", "記事の投稿に成功しました").ok();
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

pub async fn update(
    service: web::Data<Service>,
    form: web::Form<UpdateFormParams>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let post_id = PostId(form.id);
    let mut post =
        GetPostByIdUseCase::execute(&service.posts_repository, &service.search_client, &post_id)
            .await?
            .post()?;
    post.title = form.title.clone();
    post.body = form.body.clone();
    UpdatePostUseCase::execute(&service.posts_repository, &service.search_client, &post).await?;
    session.insert("message", "記事の編集に成功しました").ok();
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, format!("/{}", form.id)))
        .finish())
}

pub async fn delete(
    service: web::Data<Service>,
    form: web::Form<DeleteFormParams>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let post_id = PostId(form.id);
    DeletePostUseCase::execute(&service.posts_repository, &service.search_client, &post_id).await?;
    session.insert("message", "記事の削除に成功しました").ok();
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
    pub struct EditPostTemplate {
        pub context: AppContext,
        pub post: Post,
    }
}
