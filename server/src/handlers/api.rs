use crate::{Error, Service};
use actix_web::{web, HttpResponse};
use domain::entities::date::{Year, YearMonth};
use domain::use_cases::{get_days, get_years};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct YearMonthArguments {
    year: u16,
    month: u8,
}

#[derive(Debug, Clone, Serialize)]
struct DaysResponse {
    days: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
struct MonthsResponse {
    years: Vec<Year>,
}

pub async fn days_in_year_month(
    service: web::Data<Service>,
    args: web::Path<YearMonthArguments>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(DaysResponse {
        days: get_days(&service.posts_repository, YearMonth(args.year, args.month))?,
    }))
}

pub async fn months(service: web::Data<Service>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(MonthsResponse {
        years: get_years(&service.posts_repository)?,
    }))
}
