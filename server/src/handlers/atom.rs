use super::args::PageQuery;
use crate::context::AppContext;
use crate::{Error, Service};
use actix_web::{web, HttpResponse};
use application::use_cases::{GetLastUpdatedDateUseCase, GetLatestPostsUseCase};
use askama_actix::TemplateToResponse;
use templates::AtomTemplate;

pub async fn all_posts(
    context: AppContext,
    service: web::Data<Service>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let updated_at = GetLastUpdatedDateUseCase::execute(&service.search_client).await?;
    let page = GetLatestPostsUseCase::execute(
        &service.posts_repository,
        &service.search_client,
        query.page.unwrap_or(1),
    )
    .await?;
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
    use application::models::Page;
    use askama::Template;
    use chrono::{DateTime, Utc};
    use domain::entities::Post;

    #[derive(Template)]
    #[template(path = "atom.xml")]
    pub struct AtomTemplate<'a> {
        pub context: AppContext,
        pub updated_at: Option<DateTime<Utc>>,
        pub page: Page<'a, (), usize>,
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
