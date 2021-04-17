use super::Error;
use crate::server::Server;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct YearMonthArguments {
    year: u16,
    month: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YearArguments {
    year: u16,
}

#[derive(Debug, Clone, Serialize)]
struct DaysResponse {
    days: Vec<u8>,
}

pub async fn days_in_year_month(
    server: web::Data<Server>,
    args: web::Path<YearMonthArguments>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(DaysResponse { days: vec![4, 6, 8] }))
}
