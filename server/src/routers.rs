use crate::{
    auth_service::AuthService,
    handlers::{admin, api, auth, posts},
    server::Server,
};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::web::{get, post, resource, scope, ServiceConfig};
use std::path::PathBuf;

pub fn posts(cfg: &mut ServiceConfig) {
    cfg.service(resource("/").route(get().to(posts::all_posts)))
        .service(resource(r"/{id:\d+}").route(get().to(posts::post_with_id)))
        .service(resource(r"/{year:\d{4}}-{month:\d{2}}").route(get().to(posts::posts_with_date)))
        .service(
            resource(r"/{year:\d{4}}-{month:\d{2}}-{day:\d{2}}")
                .route(get().to(posts::posts_with_date)),
        );
}

pub fn api(cfg: &mut ServiceConfig) {
    let cors = Cors::default().allowed_origin("http://localhost:8080"); // for development
    cfg.service(
        scope("/api")
            .wrap(cors)
            .service(
                resource(r"/days/{year:\d{4}}-{month:\d{2}}")
                    .route(get().to(api::days_in_year_month)),
            )
            .service(resource("/months").route(get().to(api::months))),
    );
}

pub fn admin(cfg: &mut ServiceConfig) {
    let auth = AuthService::new(|server: &Server, id| id == server.admin_user_id);
    cfg.service(resource("/login").route(get().to(auth::login)))
        .service(resource("/logout").route(get().to(auth::logout)))
        .service(
            scope("/admin")
                .wrap(auth)
                .service(resource("/new").route(get().to(admin::new_post_form)))
                .service(resource("/edit").route(get().to(admin::edit_post_form)))
                .service(resource("/create").route(post().to(admin::create)))
                .service(resource("/update").route(post().to(admin::update)))
                .service(resource("/delete").route(post().to(admin::delete))),
        );
}

pub(crate) fn files<'a>(path: impl Into<PathBuf> + 'a) -> Box<dyn FnOnce(&mut ServiceConfig) + 'a> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.service(Files::new("/static", path));
    })
}
