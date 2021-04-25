use anyhow::Result;
use dotenv::dotenv;
use infrastructure::migration::migrate;
use std::env;

fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let pg_url = url::Url::parse(&env::var("DATABASE_NAME")?)?;
    migrate(&pg_url)
}
