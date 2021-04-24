use anyhow::Result;
use infrastructure::migration::migrate;

fn main() -> Result<()> {
    let pg_url = url::Url::parse("postgres://root:password@127.0.0.1")?;
    migrate(pg_url, "andante")
}
