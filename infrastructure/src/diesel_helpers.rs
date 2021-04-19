use chrono::FixedOffset;
use diesel::{sql_types::*, Connection};
use r2d2::CustomizeConnection;

sql_function! {
    #[sql_name = "DATE_PART"]
    fn date_part(part: Text, ts: Timestamptz) -> Double;
}

/// 各コネクションのタイムゾーンを設定するためのCustomizer
#[derive(Debug, Clone)]
pub(crate) struct TimezoneCustomizer {
    pub offset: FixedOffset,
}

impl<C: Connection> CustomizeConnection<C, diesel::r2d2::Error> for TimezoneCustomizer {
    fn on_acquire(&self, conn: &mut C) -> std::result::Result<(), diesel::r2d2::Error> {
        conn.execute(&format!(
            "SET TIME ZONE INTERVAL '{}' HOUR TO MINUTE;",
            &self.offset
        ))
        .map_err(diesel::r2d2::Error::QueryError)?;
        Ok(())
    }
}
