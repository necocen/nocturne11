use application::models::YearMonth;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DaysResponse {
    pub days: Vec<u8>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YearMonthsResponse {
    pub year_months: Vec<YearMonth>,
}
