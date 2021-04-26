use crate::server::Server;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::web::{get, post, resource, scope, ServiceConfig};
use errors::Error;
use std::path::PathBuf;
use templates::TemplateToResponse;
mod admin;
mod api;
mod errors;
mod filters;
mod posts;
mod templates;

pub(crate) fn routing(
    server: Server,
    static_path: impl Into<PathBuf>,
) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    let static_path: PathBuf = static_path.into();
    Box::new(move |cfg: &mut ServiceConfig| {
        let cors = Cors::default().allowed_origin("http://localhost:8080"); // for development
        cfg.data(server.clone())
            .service(resource("/").route(get().to(posts::all_posts)))
            .service(
                scope("/admin")
                    .service(resource("/new").route(get().to(admin::new_post_form)))
                    .service(resource("/create").route(post().to(admin::create))),
            )
            .service(resource(r"/{id:\d+}").route(get().to(posts::post_with_id)))
            .service(
                resource(r"/{year:\d{4}}-{month:\d{2}}").route(get().to(posts::posts_with_date)),
            )
            .service(
                resource(r"/{year:\d{4}}-{month:\d{2}}-{day:\d{2}}")
                    .route(get().to(posts::posts_with_date)),
            )
            .service(
                scope("/api")
                    .wrap(cors)
                    .service(
                        resource(r"/days/{year:\d{4}}-{month:\d{2}}")
                            .route(get().to(api::days_in_year_month)),
                    )
                    .service(resource("/months").route(get().to(api::months))),
            )
            .service(fs::Files::new("/static", static_path));
    })
}
