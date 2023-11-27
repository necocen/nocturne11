use super::args::LoginFormParams;
use crate::{context::AppContext, Error, Service};
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use application::use_cases::AuthenticateUseCase;

pub async fn login(
    context: AppContext,
    service: web::Data<Service>,
    request: HttpRequest,
    session: Session,
    form: web::Form<LoginFormParams>,
) -> Result<HttpResponse, Error> {
    let user_id =
        AuthenticateUseCase::execute(&context.config, &service.cert_repository, &form.id_token)
            .await?;
    Identity::login(&request.extensions(), user_id).map_err(|e| Error::Other(e.into()))?;
    session.insert("message", "ログインに成功しました").ok();
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/admin/"))
        .finish())
}

pub async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.logout();
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}
