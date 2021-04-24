use actix_cors::Cors;
use actix_web::App;
use anyhow::Result;
mod server;
use server::Server;
mod handlers;
use handlers::routing;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    let es_url = url::Url::parse("http://localhost:9200")?;
    let pg_url = url::Url::parse("postgres://root:password@127.0.0.1/andante")?;
    let server = Server::new(&es_url, &pg_url)?;
    actix_web::HttpServer::new(move || {
        let cors = Cors::default().allowed_origin("http://localhost:8080"); // for development
        App::new()
            .wrap(cors)
            .configure(routing(server.clone(), "./frontend/build/src"))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await?;
    Ok(())
}
