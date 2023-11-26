use chrono::{DateTime, Datelike as _, Local, TimeZone as _, Utc};

use crate::{errors::ApplicationError, ApplicationResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, Hash)]
#[non_exhaustive]
pub struct YearMonth {
    pub year: u16,
    pub month: u8,
}

impl YearMonth {
    pub fn new(year: u16, month: u8) -> ApplicationResult<Self> {
        if year == 0 || month > 12 || month == 0 {
            return Err(ApplicationError::InvalidYearMonth);
        }
        Ok(Self { year, month })
    }
}

impl From<DateTime<Utc>> for YearMonth {
    fn from(dt: DateTime<Utc>) -> Self {
        let naive = dt.with_timezone(&Local).naive_local();
        YearMonth {
            year: naive.year() as u16,
            month: naive.month() as u8,
        }
    }
}

impl From<YearMonth> for DateTime<Utc> {
    fn from(ym: YearMonth) -> Self {
        let local_datetime = Local
            .with_ymd_and_hms(ym.year as i32, ym.month as u32, 1, 0, 0, 0)
            .unwrap();
        local_datetime.with_timezone(&Utc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ApplicationError;
    use assert_matches::assert_matches;

    #[test]
    fn test_constructor() {
        assert_matches!(YearMonth::new(2021, 1), Ok(_));
        assert_matches!(YearMonth::new(2021, 12), Ok(_));
    }

    #[test]
    fn test_returns_err_for_invalid_year_month() {
        assert_matches!(
            YearMonth::new(0, 0).unwrap_err(),
            ApplicationError::InvalidYearMonth
        );
        assert_matches!(
            YearMonth::new(0, 13).unwrap_err(),
            ApplicationError::InvalidYearMonth
        );
    }
}
