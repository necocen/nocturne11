use errors::Error;
use service::Service;
mod askama_helpers;
mod context;
mod errors;
mod handlers;
mod presentation;
mod routers;
mod service;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{App, HttpServer};
    env_logger::init();
    dotenv::dotenv().ok();
    let service = Service::new()?;
    HttpServer::new(move || App::new().configure(routers::routing(service.clone())))
        .bind("0.0.0.0:4000")?
        .run()
        .await?;
    Ok(())
}
