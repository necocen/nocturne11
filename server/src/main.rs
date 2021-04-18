use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App};
use anyhow::Result;
use std::path::Path;
mod server;
use server::Server;
mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    let es_url = url::Url::parse("http://localhost:9200")?;
    let pg_url = url::Url::parse("postgres://root:password@127.0.0.1/andante")?;
    let server = Server::new(es_url, pg_url)?;
    actix_web::HttpServer::new(move || {
        let cors = Cors::default().allowed_origin("http://localhost:8080"); // for development
        App::new()
            .wrap(cors)
            .configure(config_app(server.clone(), "./frontend/build/src"))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await?;
    Ok(())
}

fn config_app(
    server: Server,
    static_path: impl AsRef<Path>,
) -> Box<dyn FnOnce(&mut web::ServiceConfig)> {
    let static_path = static_path.as_ref().to_owned();
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.data(server.clone())
            .service(web::resource("/").route(web::get().to(handlers::posts::all_posts)))
            .service(
                web::resource(r"/{id:\d+}").route(web::get().to(handlers::posts::post_with_id)),
            )
            .service(
                web::resource(r"/{year:\d{4}}-{month:\d{2}}")
                    .route(web::get().to(handlers::posts::posts_with_date)),
            )
            .service(
                web::resource(r"/{year:\d{4}}-{month:\d{2}}-{day:\d{2}}")
                    .route(web::get().to(handlers::posts::posts_with_date)),
            )
            .service(
                web::resource(r"/api/days/{year:\d{4}}-{month:\d{2}}")
                    .route(web::get().to(handlers::api::days_in_year_month)),
            )
            .service(web::resource(r"/api/months").route(web::get().to(handlers::api::months)))
            .service(fs::Files::new("/static", static_path));
    })
}
