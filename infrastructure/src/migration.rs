use anyhow::Result;
use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness as _};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn migrate(pg_url: &url::Url) -> Result<()> {
    let mut conn = PgConnection::establish(pg_url.as_str())?;
    conn.run_pending_migrations(MIGRATIONS).expect("failed to run migrations");
    Ok(())
}
