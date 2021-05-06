use actix_web::App;
use anyhow::{ensure, Result};
use dotenv::dotenv;
use errors::Error;
use server::Server;
use std::env;
mod askama_helpers;
mod context;
mod errors;
mod handlers;
mod routers;
mod server;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY")?;
    ensure!(secret_key.len() >= 32, "SECRET_KEY is not long enough.");

    let es_url = url::Url::parse(&env::var("ES_URL")?)?;
    let pg_url = url::Url::parse(&env::var("DATABASE_URL")?)?;
    let server = Server::new(&es_url, &pg_url, &env::var("ADMIN_USER_ID")?)?;
    actix_web::HttpServer::new(move || {
        App::new().configure(routers::routing(
            server.clone(),
            secret_key.clone(),
            "./frontend/build/src",
        ))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await?;
    Ok(())
}
