use super::args::PageQuery;
use crate::context::AppContext;
use crate::{Error, Service};
use actix_web::{web, HttpResponse};
use askama_actix::TemplateToResponse;
use domain::use_cases::{get_last_updated_date, get_posts};
use templates::AtomTemplate;

pub async fn all_posts(
    context: AppContext,
    service: web::Data<Service>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let updated_at = get_last_updated_date(&service.posts_repository)?;
    let page = get_posts(&service.posts_repository, 20, query.page.unwrap_or(1))?;
    Ok(AtomTemplate {
        context,
        updated_at,
        page,
    }
    .to_response())
}

mod templates {
    use crate::filters;
    use crate::{context::AppContext, presentation::posts::Body};
    use askama::Template;
    use chrono::{DateTime, Utc};
    use domain::entities::{Page, Post};

    #[derive(Template)]
    #[template(path = "atom.xml")]
    pub struct AtomTemplate<'a> {
        pub context: AppContext,
        pub updated_at: Option<DateTime<Utc>>,
        pub page: Page<'a, ()>,
    }

    trait PostExt {
        /// 本文の段落記法をHTMLタグに変換します
        fn converted_body(&self) -> String;
    }

    impl PostExt for Post {
        fn converted_body(&self) -> String {
            Body::new(&self.body).to_html(false)
        }
    }

    trait ConditionToString {
        fn to_string(&self) -> String;
    }

    impl ConditionToString for () {
        fn to_string(&self) -> String {
            "".to_owned()
        }
    }
}
