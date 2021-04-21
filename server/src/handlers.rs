use crate::server::Server;
use actix_files as fs;
use actix_web::web;
use errors::Error;
use std::path::PathBuf;
use templates::TemplateToResponse;
mod api;
mod errors;
mod filters;
mod posts;
mod templates;

pub(crate) fn routing(
    server: Server,
    static_path: impl Into<PathBuf>,
) -> Box<dyn FnOnce(&mut web::ServiceConfig)> {
    let static_path: PathBuf = static_path.into();
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.data(server.clone())
            .service(web::resource("/").route(web::get().to(posts::all_posts)))
            .service(web::resource(r"/{id:\d+}").route(web::get().to(posts::post_with_id)))
            .service(
                web::resource(r"/{year:\d{4}}-{month:\d{2}}")
                    .route(web::get().to(posts::posts_with_date)),
            )
            .service(
                web::resource(r"/{year:\d{4}}-{month:\d{2}}-{day:\d{2}}")
                    .route(web::get().to(posts::posts_with_date)),
            )
            .service(
                web::resource(r"/api/days/{year:\d{4}}-{month:\d{2}}")
                    .route(web::get().to(api::days_in_year_month)),
            )
            .service(web::resource("/api/months").route(web::get().to(api::months)))
            .service(fs::Files::new("/static", static_path));
    })
}
