use super::Error;
use crate::server::Server;
use actix_web::{web, HttpResponse};
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

#[derive(Debug, Clone, Serialize)]
struct Year {
    year: u16,
    months: Vec<u8>,
}

pub async fn days_in_year_month(
    server: web::Data<Server>,
    args: web::Path<YearMonthArguments>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(DaysResponse {
        days: vec![4, 6, 8, args.month],
    }))
}

pub async fn months(server: web::Data<Server>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(MonthsResponse {
        years: vec![
            Year {
                year: 2019,
                months: vec![6, 7, 8, 9, 10, 11, 12],
            },
            Year {
                year: 2020,
                months: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            },
            Year {
                year: 2021,
                months: vec![1, 2, 3, 4],
            },
        ],
    }))
}
