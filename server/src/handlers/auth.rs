use super::args::LoginFormParams;
use crate::{Error, Service};
use actix_identity::Identity;
use actix_web::{http::header, web, HttpResponse};
use domain::use_cases::check_login;

pub async fn login(
    service: web::Data<Service>,
    id: Identity,
    form: web::Form<LoginFormParams>,
) -> Result<HttpResponse, Error> {
    let user_id = check_login(
        &service.config_repository,
        &service.cert_repository,
        &form.id_token,
    )
    .await?;
    id.remember(user_id);
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/admin/"))
        .finish())
}

pub async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}
