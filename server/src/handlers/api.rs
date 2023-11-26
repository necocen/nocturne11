use crate::{Error, Service};
use actix_web::{web, HttpResponse};
use application::use_cases::{GetDaysInYearMonthUseCase, GetYearMonthsUseCase};

use super::{
    args::YearMonthArguments,
    responses::{DaysResponse, YearMonthsResponse},
};

pub async fn days_in_year_month(
    service: web::Data<Service>,
    args: web::Path<YearMonthArguments>,
) -> Result<HttpResponse, Error> {
    let days =
        GetDaysInYearMonthUseCase::execute(&service.search_client, &args.into_inner().try_into()?)
            .await?;
    Ok(HttpResponse::Ok().json(DaysResponse { days }))
}

pub async fn year_months(service: web::Data<Service>) -> Result<HttpResponse, Error> {
    let year_months = GetYearMonthsUseCase::execute(&service.search_client).await?;
    Ok(HttpResponse::Ok().json(YearMonthsResponse { year_months }))
}
