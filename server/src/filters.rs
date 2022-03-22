use chrono::{DateTime, Local, Utc};

pub fn format_date(date: &DateTime<Utc>) -> ::askama::Result<String> {
    Ok(date.with_timezone(&Local).format("%F %T").to_string())
}

pub fn iso8601(date: &DateTime<Utc>) -> ::askama::Result<String> {
    Ok(date.with_timezone(&Local).to_rfc3339())
}
