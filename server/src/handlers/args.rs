use anyhow::anyhow;
use chrono::NaiveDate;
use domain::entities::date::YearMonth;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct IdArguments {
    pub id: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DateArguments {
    year: u16,
    month: u8,
    day: u8,
}

impl TryFrom<DateArguments> for NaiveDate {
    type Error = anyhow::Error;

    fn try_from(args: DateArguments) -> Result<NaiveDate, Self::Error> {
        NaiveDate::from_ymd_opt(args.year as i32, args.month as u32, args.day as u32)
            .ok_or(anyhow!("invalid date"))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct YearMonthArguments {
    year: u16,
    month: u8,
}

impl From<YearMonthArguments> for YearMonth {
    fn from(args: YearMonthArguments) -> YearMonth {
        // TODO: たぶん本当はYearMonthにコンストラクタがあって、そこでバリデーションするべき
        YearMonth(args.year, args.month)
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
