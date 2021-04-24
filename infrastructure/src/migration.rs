use crate::embedded_migrations;
use anyhow::Result;
use diesel::{Connection, PgConnection};

pub fn migrate(pg_url: &url::Url) -> Result<()> {
    let conn = PgConnection::establish(pg_url.as_str())?;
    embedded_migrations::run(&conn)?;
    Ok(())
}
