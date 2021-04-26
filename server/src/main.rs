use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{cookie::SameSite, App};
use anyhow::Result;
mod server;
use server::Server;
mod handlers;
use dotenv::dotenv;
use handlers::routing;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY")?;
    if secret_key.len() < 32 {
        panic!("SECRET_KEY is not long enough.");
    }
    let es_url = url::Url::parse(&env::var("ES_URL")?)?;
    let pg_url = url::Url::parse(&env::var("DATABASE_URL")?)?;
    let server = Server::new(&es_url, &pg_url)?;
    actix_web::HttpServer::new(move || {
        let identity = IdentityService::new(
            CookieIdentityPolicy::new(secret_key.as_bytes())
                .name("nocturne-identity")
                .same_site(SameSite::Lax)
                .secure(false), // for development
        );
        let session = CookieSession::signed(secret_key.as_bytes())
            .name("nocturne-session")
            .same_site(SameSite::Lax)
            .secure(false); // for development
        App::new()
            .wrap(session)
            .wrap(identity)
            .configure(routing(server.clone(), "./frontend/build/src"))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await?;
    Ok(())
}
