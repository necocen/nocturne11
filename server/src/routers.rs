use crate::handlers::{about, admin, api, auth, posts};
use actix_web::web::{get, post, resource, ServiceConfig};
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
    cfg.service(
        resource(r"/days/{year:\d{4}}-{month:\d{2}}").route(get().to(api::days_in_year_month)),
    )
    .service(resource("/months").route(get().to(api::months)));
}

pub fn auth(cfg: &mut ServiceConfig) {
    cfg.service(resource("/login").route(get().to(auth::login)))
        .service(resource("/logout").route(get().to(auth::logout)));
}

pub fn admin(cfg: &mut ServiceConfig) {
    use actix_web::{http::header, HttpResponse};
    cfg.service(resource("/").route(get().to(admin::index)))
        .service(resource("/new").route(get().to(admin::new_post_form)))
        .service(resource("/edit").route(get().to(admin::edit_post_form)))
        .service(resource("/create").route(post().to(admin::create)))
        .service(resource("/update").route(post().to(admin::update)))
        .service(resource("/delete").route(post().to(admin::delete)))
        .service(resource("/config").route(get().to(admin::show_config)))
        .service(resource("/update-config").route(post().to(admin::update_config)))
        .service(resource("").route(get().to(|| {
            HttpResponse::Found()
                .append_header((header::LOCATION, "/admin/"))
                .finish()
        })));
}

pub fn about(cfg: &mut ServiceConfig) {
    cfg.service(resource("/about").route(get().to(about::about)));
}

pub fn files<'a>(path: impl Into<PathBuf> + 'a) -> impl FnOnce(&mut ServiceConfig) + 'a {
    use actix_files::Files;
    move |cfg: &mut ServiceConfig| {
        cfg.service(Files::new("/static", path));
    }
}
