use errors::Error;
use server::Server;
mod askama_helpers;
mod context;
mod errors;
mod handlers;
mod routers;
mod server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{App, HttpServer};
    env_logger::init();
    dotenv::dotenv().ok();
    let server = Server::new()?;
    HttpServer::new(move || App::new().configure(routers::routing(server.clone())))
        .bind("0.0.0.0:4000")?
        .run()
        .await?;
    Ok(())
}
