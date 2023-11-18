use crate::{
    context::{AppContext, AppContextService},
    handlers::{about, admin, api, atom, auth, errors, posts},
    Service,
};
use actix_cors::Cors;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    guard::{fn_guard, GuardContext},
    http::StatusCode,
    middleware::ErrorHandlers,
    web::{get, post, resource, route, scope, ServiceConfig},
    web::{Data, FormConfig},
    HttpResponse,
};

pub fn routing(service: Service) -> impl FnOnce(&mut ServiceConfig) {
    move |cfg: &mut ServiceConfig| {
        let session = SessionMiddleware::builder(
            CookieSessionStore::default(),
            Key::derive_from(service.secret_key.as_bytes()),
        )
        .cookie_name("nocturne-session".to_string())
        .cookie_same_site(SameSite::Lax)
        .cookie_secure(!service.is_development)
        .build();

        let cors = if service.is_development {
            Cors::default().allowed_origin("http://localhost:5173")
        } else {
            Cors::default()
        };
        let static_path = service.static_path.clone();

        cfg.app_data(Data::new(service))
            .app_data(FormConfig::default().limit(1024 * 1024 * 20))
            .service(Files::new("/static", static_path))
            .service(scope("/api").wrap(cors).configure(api))
            .service(
                scope("")
                    .wrap(
                        ErrorHandlers::new()
                            .handler(StatusCode::BAD_REQUEST, errors::error_400)
                            .handler(StatusCode::UNAUTHORIZED, errors::error_401)
                            .handler(StatusCode::NOT_FOUND, errors::error_404)
                            .handler(StatusCode::INTERNAL_SERVER_ERROR, errors::error_500),
                    )
                    .wrap(AppContextService)
                    .wrap(IdentityMiddleware::default())
                    .wrap(session)
                    .configure(posts)
                    .configure(atom)
                    .configure(auth)
                    .configure(about)
                    .service(
                        scope("/admin")
                            .service(scope("").guard(fn_guard(admin_guard)).configure(admin))
                            .default_service(route().to(|| async {
                                HttpResponse::Unauthorized().body("Unauthorized")
                            })),
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

fn atom(cfg: &mut ServiceConfig) {
    cfg.service(resource("/atom").route(get().to(atom::all_posts)));
}

fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        resource(r"/days/{year:\d{4}}-{month:\d{2}}").route(get().to(api::days_in_year_month)),
    )
    .service(resource("/months").route(get().to(api::months)));
}

fn auth(cfg: &mut ServiceConfig) {
    cfg.service(resource("/login").route(post().to(auth::login)))
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
        .service(resource("").route(get().to(|| async {
            HttpResponse::Found()
                .append_header((header::LOCATION, "/admin/"))
                .finish()
        })));
}

fn about(cfg: &mut ServiceConfig) {
    cfg.service(resource("/about").route(get().to(about::about)));
}

fn admin_guard(ctx: &GuardContext) -> bool {
    ctx.req_data()
        .get::<AppContext>()
        .map_or(false, |ctx| ctx.is_authorized)
}
