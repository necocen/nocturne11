use crate::Error;
use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use actix_web::HttpResponse;
use templates::AboutTemplate;

pub async fn about(context: AppContext) -> Result<HttpResponse, Error> {
    AboutTemplate { context }.to_response()
}

mod templates {
    use crate::{context::AppContext, presentation::body::Body};
    use askama::Template;

    #[derive(Template)]
    #[template(path = "about.html")]
    pub struct AboutTemplate {
        pub context: AppContext,
    }

    trait AppContextExt {
        /// aboutの段落記法をHTMLタグに変換します
        fn converted_about(&self) -> String;
    }

    impl AppContextExt for AppContext {
        fn converted_about(&self) -> String {
            Body::new(&self.config.site.about).to_html()
        }
    }
}
