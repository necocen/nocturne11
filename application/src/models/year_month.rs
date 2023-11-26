use chrono::{DateTime, Datelike as _, Local, TimeZone as _, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, Hash)]
pub struct YearMonth(pub u16, pub u8);

impl From<DateTime<Utc>> for YearMonth {
    fn from(dt: DateTime<Utc>) -> Self {
        let naive = dt.with_timezone(&Local).naive_local();
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
