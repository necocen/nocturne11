use super::args::{DateArguments, IdArguments, PageQuery};
use crate::askama_helpers::TemplateToResponse;
use crate::{Error, Server};
use actix_web::{web, HttpResponse};
use domain::use_cases::{get_post_with_id, get_posts, get_posts_with_date_condition};
use templates::{AllPostsTemplate, PostTemplate, PostsWithDateTemplate};

pub async fn all_posts(
    server: web::Data<Server>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let page = get_posts(&server.posts_repository, 10, query.page.unwrap_or(1))?;
    AllPostsTemplate {
        page,
        title: "タイトル",
    }
    .to_response()
}

pub async fn post_with_id(
    server: web::Data<Server>,
    args: web::Path<IdArguments>,
) -> Result<HttpResponse, Error> {
    let page = get_post_with_id(&server.posts_repository, &args.id)?;
    PostTemplate {
        page,
        title: "タイトル",
    }
    .to_response()
}
pub async fn posts_with_date(
    server: web::Data<Server>,
    args: web::Path<DateArguments>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let condition = args.to_owned().into();
    let page = get_posts_with_date_condition(
        &server.posts_repository,
        &condition,
        10,
        query.page.unwrap_or(1),
    )?;

    PostsWithDateTemplate {
        title: "タイトル",
        page,
    }
    .to_response()
}

mod templates {
    use crate::askama_helpers::convert_body;
    pub use crate::askama_helpers::filters;
    use askama::Template;
    use chrono::NaiveDate;
    use domain::entities::{date::DateCondition, AdjacentPage, Page, Post, PostId};

    #[derive(Template)]
    #[template(path = "all_posts.html")]
    pub struct AllPostsTemplate<'a> {
        pub title: &'a str,
        pub page: Page<'a, ()>,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct PostsWithDateTemplate<'a> {
        pub title: &'a str,
        pub page: Page<'a, DateCondition>,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct PostTemplate<'a> {
        pub title: &'a str,
        pub page: Page<'a, PostId>,
    }

    trait PostExt {
        /// 本文の段落記法をHTMLタグに変換します
        fn converted_body(&self) -> String;

        /// Permalink URLの出力
        fn permalink(&self) -> url::Url;
    }

    impl PostExt for Post {
        fn converted_body(&self) -> String {
            convert_body(&self.body)
        }

        fn permalink(&self) -> url::Url {
            let base_url = url::Url::parse("https://ofni.necocen.info/").unwrap();
            base_url.join(&self.id.to_string()).unwrap()
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
