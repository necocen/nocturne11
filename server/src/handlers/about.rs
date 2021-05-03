use crate::askama_helpers::TemplateToResponse;
use crate::{Error, Server};
use actix_web::{web, HttpResponse};
use domain::use_cases::get_config;
use templates::{AboutContent, AboutTemplate};

pub async fn about(server: web::Data<Server>) -> Result<HttpResponse, Error> {
    let config = get_config(&server.config_repository)?;
    AboutTemplate {
        title: "About",
        content: AboutContent(config.about),
    }
    .to_response()
}

mod templates {
    use crate::askama_helpers::convert_body;
    use askama::Template;

    pub struct AboutContent(pub String);

    impl AboutContent {
        fn converted(&self) -> String {
            convert_body(&self.0)
        }
    }

    #[derive(Template)]
    #[template(path = "about.html")]
    pub struct AboutTemplate<'a> {
        pub title: &'a str,
        pub content: AboutContent,
    }
}
