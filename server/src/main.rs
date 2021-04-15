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
            "./frontend/build/src".to_string(),
        ))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await?;
    Ok(())
}

fn config_app(server: Server, serve_from: String) -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.data(server.clone())
            .service(web::resource("/").route(web::get().to(handlers::posts::posts)))
            .service(fs::Files::new("/", serve_from.clone()));
    })
}
