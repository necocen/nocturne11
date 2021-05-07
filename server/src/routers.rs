use crate::{
    context::{AppContextService, RequestHeadContext},
    handlers::{about, admin, api, auth, posts},
    Service,
};
use actix_cors::Cors;
use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    cookie::SameSite,
    guard::fn_guard,
    web::{get, post, resource, route, scope, ServiceConfig},
    HttpResponse,
};

pub fn routing(service: Service) -> impl FnOnce(&mut ServiceConfig) {
    move |cfg: &mut ServiceConfig| {
        let identity = IdentityService::new(
            CookieIdentityPolicy::new(service.secret_key.as_bytes())
                .name("nocturne-identity")
                .same_site(SameSite::Lax)
                .secure(false), // for development
        );
        let session = CookieSession::signed(service.secret_key.as_bytes())
            .name("nocturne-session")
            .same_site(SameSite::Lax)
            .secure(false); // for development
        let cors = Cors::default().allowed_origin("http://localhost:8080"); // for development
        let static_path = service.static_path.clone();

        cfg.data(service)
            .service(Files::new("/static", static_path))
            .service(scope("/api").wrap(cors).configure(api))
            .service(
                scope("")
                    .wrap(AppContextService)
                    .wrap(session)
                    .wrap(identity)
                    .configure(posts)
                    .configure(auth)
                    .configure(about)
                    .service(
                        scope("/admin")
                            .service(
                                scope("")
                                    .guard(fn_guard(RequestHeadContext::is_authorized))
                                    .configure(admin),
                            )
                            .default_service(
                                route().to(|| HttpResponse::Unauthorized().body("Unauthorized")),
                            ),
                    ),
            );
    }
}

fn posts(cfg: &mut ServiceConfig) {
    cfg.service(resource("/").route(get().to(posts::all_posts)))
        .service(resource(r"/{id:\d+}").route(get().to(posts::post_with_id)))
        .service(resource(r"/{year:\d{4}}-{month:\d{2}}").route(get().to(posts::posts_with_date)))
        .service(
            resource(r"/{year:\d{4}}-{month:\d{2}}-{day:\d{2}}")
                .route(get().to(posts::posts_with_date)),
        );
}

fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        resource(r"/days/{year:\d{4}}-{month:\d{2}}").route(get().to(api::days_in_year_month)),
    )
    .service(resource("/months").route(get().to(api::months)));
}

fn auth(cfg: &mut ServiceConfig) {
    cfg.service(resource("/login").route(get().to(auth::login)))
        .service(resource("/logout").route(get().to(auth::logout)));
}

fn admin(cfg: &mut ServiceConfig) {
    use actix_web::http::header;
    cfg.service(resource("/").route(get().to(admin::index)))
        .service(resource("/new").route(get().to(admin::new_post_form)))
        .service(resource("/edit").route(get().to(admin::edit_post_form)))
        .service(resource("/create").route(post().to(admin::create)))
        .service(resource("/update").route(post().to(admin::update)))
        .service(resource("/delete").route(post().to(admin::delete)))
        .service(resource("").route(get().to(|| {
            HttpResponse::Found()
                .append_header((header::LOCATION, "/admin/"))
                .finish()
        })));
}

fn about(cfg: &mut ServiceConfig) {
    cfg.service(resource("/about").route(get().to(about::about)));
}
