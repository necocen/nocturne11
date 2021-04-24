#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use anyhow::Result;
use diesel_migrations::embed_migrations;

embed_migrations!("migrations/");

fn main() -> Result<()> {
    let pg_url = url::Url::parse("postgres://root:password@127.0.0.1")?;
    migrate(pg_url, "andante")
}

fn migrate(pg_url: url::Url, db_name: impl Into<String>) -> Result<()> {
    let db_name = db_name.into();
    let conn = PgConnection::establish(pg_url.join(&db_name)?.as_str())?;
    embedded_migrations::run(&conn)?;
    Ok(())
}
