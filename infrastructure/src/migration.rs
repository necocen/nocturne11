use crate::embedded_migrations;
use anyhow::Result;
use diesel::{Connection, PgConnection};

pub fn migrate(pg_url: url::Url, db_name: impl Into<String>) -> Result<()> {
    let conn = PgConnection::establish(pg_url.join(&db_name.into())?.as_str())?;
    embedded_migrations::run(&conn)?;
    Ok(())
}
