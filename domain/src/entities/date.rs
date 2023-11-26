use super::Condition;
use chrono::{DateTime, Datelike as _, Local, TimeZone as _, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Year {
    pub year: u16,
    pub months: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
pub struct YearMonth(pub u16, pub u8);

impl From<DateTime<Utc>> for YearMonth {
    fn from(dt: DateTime<Utc>) -> Self {
        let naive = dt.naive_local();
        YearMonth(naive.year() as u16, naive.month() as u8)
    }
}

impl From<YearMonth> for DateTime<Utc> {
    fn from(ym: YearMonth) -> Self {
        let local_datetime = Local
            .with_ymd_and_hms(ym.0 as i32, ym.1 as u32, 1, 0, 0, 0)
            .unwrap();
        local_datetime.with_timezone(&Utc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateCondition {
    pub ym: YearMonth,
    pub day: Option<u8>,
}

impl Condition for DateCondition {
    type Page = usize;
}
