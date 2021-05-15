use super::args::{DateArguments, IdArguments, PageQuery};
use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use crate::{Error, Service};
use actix_web::{web, HttpResponse};
use domain::use_cases::{get_post_with_id, get_posts, get_posts_with_date_condition};
use templates::{AllPostsTemplate, PostTemplate, PostsWithDateTemplate};

pub async fn all_posts(
    context: AppContext,
    service: web::Data<Service>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let page = get_posts(&service.posts_repository, 10, query.page.unwrap_or(1))?;
    AllPostsTemplate { context, page }.to_response()
}

pub async fn post_with_id(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Path<IdArguments>,
) -> Result<HttpResponse, Error> {
    let page = get_post_with_id(&service.posts_repository, &args.id)?;
    PostTemplate { context, page }.to_response()
}
pub async fn posts_with_date(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Path<DateArguments>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let condition = args.to_owned().into();
    let page = get_posts_with_date_condition(
        &service.posts_repository,
        &condition,
        10,
        query.page.unwrap_or(1),
    )?;

    PostsWithDateTemplate { context, page }.to_response()
}

mod templates {
    pub use crate::askama_helpers::filters;
    use crate::{context::AppContext, presentation::body::Body};
    use askama::Template;
    use chrono::NaiveDate;
    use domain::entities::{
        date::DateCondition, AdjacentPage, Page, Post, PostId, SearchCondition,
    };

    #[derive(Template)]
    #[template(path = "all_posts.html")]
    pub struct AllPostsTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, ()>,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct PostsWithDateTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, DateCondition>,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct PostTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, PostId>,
    }

    trait PostExt {
        /// 本文の段落記法をHTMLタグに変換します
        fn converted_body(&self) -> String;
    }

    impl PostExt for Post {
        fn converted_body(&self) -> String {
            Body::new(&self.body).to_html()
        }
    }

    trait DateConditionToString {
        fn to_string(&self) -> String;
    }

    impl DateConditionToString for DateCondition {
        fn to_string(&self) -> String {
            NaiveDate::from_ymd(
                self.ym.0.into(),
                self.ym.1.into(),
                self.day.unwrap_or(1).into(),
            )
            .format(if self.day.is_some() {
                "%Y-%m-%d"
            } else {
                "%Y-%m"
            })
            .to_string()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use domain::entities::date::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn year_month_to_string() {
            let condition = DateCondition {
                ym: YearMonth(1989, 9),
                day: None,
            };
            assert_eq!(condition.to_string(), "1989-09");
        }

        #[test]
        fn year_month_day_to_string() {
            let condition = DateCondition {
                ym: YearMonth(1989, 9),
                day: Some(30),
            };
            assert_eq!(condition.to_string(), "1989-09-30");
        }
    }
}
