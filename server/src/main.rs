use std::path::Path;
use actix_files as fs;
use actix_web::{web, App};
use anyhow::Result;
mod server;
use server::Server;
mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    let es_url = url::Url::parse("http://localhost:9200")?;
    let pg_url = url::Url::parse("postgres://root:password@127.0.0.1/andante")?;
    let server = Server::new(es_url, pg_url)?;
    actix_web::HttpServer::new(move || {
        App::new().configure(config_app(
            server.clone(),
            "./frontend/build/src",
        ))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await?;
    Ok(())
}

fn config_app(server: Server, static_path: impl AsRef<Path>) -> Box<dyn FnOnce(&mut web::ServiceConfig)> {
    let static_path = static_path.as_ref().to_owned();
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.data(server.clone())
            .service(web::resource("/").route(web::get().to(handlers::posts::posts)))
            .service(fs::Files::new("/static", static_path));
    })
}
