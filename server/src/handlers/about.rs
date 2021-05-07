use crate::Error;
use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use actix_web::HttpResponse;
use templates::AboutTemplate;

pub async fn about(context: AppContext) -> Result<HttpResponse, Error> {
    AboutTemplate { context }.to_response()
}

mod templates {
    use crate::context::AppContext;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "about.html")]
    pub struct AboutTemplate {
        pub context: AppContext,
    }

    impl AboutTemplate {
        pub fn convert_body(body: &str) -> String {
            use crate::askama_helpers::convert_body;
            convert_body(body)
        }
    }
}
