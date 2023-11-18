use super::args::LoginFormParams;
use crate::{Error, Service};
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use domain::use_cases::check_login;

pub async fn login(
    service: web::Data<Service>,
    request: HttpRequest,
    session: Session,
    form: web::Form<LoginFormParams>,
) -> Result<HttpResponse, Error> {
    let user_id = check_login(
        &service.config_repository,
        &service.cert_repository,
        &form.id_token,
    )
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
