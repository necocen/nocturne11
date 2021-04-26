use super::Error;
use actix_identity::Identity;
use actix_web::{http::header, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct UserQuery {
    user_id: String,
}

pub(super) async fn login(
    id: Identity,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, Error> {
    id.remember(query.user_id.clone());
    dbg!(&query.user_id);
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

pub(super) async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}
