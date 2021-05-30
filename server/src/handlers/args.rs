use domain::entities::date::{DateCondition, YearMonth};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct IdArguments {
    pub id: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DateArguments {
    year: u16,
    month: u8,
    day: Option<u8>,
}

impl From<DateArguments> for DateCondition {
    fn from(args: DateArguments) -> DateCondition {
        DateCondition {
            ym: YearMonth(args.year, args.month),
            day: args.day,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageQuery {
    pub page: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeywordsQuery {
    pub page: Option<usize>,
    pub keywords: Option<String>,
    pub search_after: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFormParams {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFormParams {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteFormParams {
    pub id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginFormParams {
    pub id_token: String,
}
