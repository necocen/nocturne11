use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{cookie::SameSite, web, App};
use anyhow::{ensure, Result};
use context::AppContextService;
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
            .data(server.clone())
            .configure(routers::api)
            .configure(routers::files("./frontend/build/src"))
            .service(
                web::scope("")
                    .wrap(AppContextService)
                    .wrap(session)
                    .wrap(identity)
                    .configure(routers::posts)
                    .configure(routers::admin)
                    .configure(routers::about),
            )
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await?;
    Ok(())
}
