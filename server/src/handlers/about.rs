use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use crate::{Error, Server};
use actix_web::{web, HttpResponse};
use domain::use_cases::get_config;
use templates::{AboutContent, AboutTemplate};

pub async fn about(context: AppContext, server: web::Data<Server>) -> Result<HttpResponse, Error> {
    let config = get_config(&server.config_repository)?;
    AboutTemplate {
        context,
        content: AboutContent(config.about),
    }
    .to_response()
}

mod templates {
    use crate::{askama_helpers::convert_body, context::AppContext};
    use askama::Template;

    pub struct AboutContent(pub String);

    impl AboutContent {
        fn converted(&self) -> String {
            convert_body(&self.0)
        }
    }

    #[derive(Template)]
    #[template(path = "about.html")]
    pub struct AboutTemplate {
        pub context: AppContext,
        pub content: AboutContent,
    }
}
