use anyhow::{Context, Result};
use dotenv::dotenv;
use infrastructure::migration::migrate;
use std::env;

fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let pg_url = url::Url::parse(&env::var("DATABASE_URL").context("DATABASE_URL was not found")?)?;
    migrate(&pg_url)
}
