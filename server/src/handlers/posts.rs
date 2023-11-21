use super::args::{DateArguments, IdArguments, KeywordsQuery, PageQuery};
use crate::context::AppContext;
use crate::{Error, Service};
use actix_web::{web, HttpResponse};
use askama_actix::TemplateToResponse;
use domain::entities::{KeywordsCondition, PostId};
use domain::use_cases::{get_post_with_id, get_posts, get_posts_with_date_condition, search_posts};
use templates::{AllPostsTemplate, PostTemplate, PostsWithDateTemplate, SearchPostsTemplate};

pub async fn all_posts(
    context: AppContext,
    service: web::Data<Service>,
    query: web::Query<KeywordsQuery>,
) -> Result<HttpResponse, Error> {
    if let Some(keywords) = &query.keywords {
        let keywords = KeywordsCondition(keywords.split_whitespace().collect::<Vec<_>>());
        let search_after = query.search_after.as_ref().and_then(|search_after| {
            let mut search_after_iter = search_after
                .split(',')
                .map(str::parse::<u64>)
                .flat_map(Result::ok);
            let a = search_after_iter.next();
            let b = search_after_iter.next();
            a.and_then(|a| b.map(|b| (a, b)))
        });
        let page = search_posts(
            &service.posts_repository,
            &service.search_repository,
            &keywords,
            10,
            search_after,
        )
        .await?;
        if page.posts.is_empty() {
            return if search_after.is_none() {
                Err(Error::NoResult(
                    "このページには記事が存在しません。".to_owned(),
                ))
            } else {
                Err(Error::NoResult(
                    "指定されたキーワードに一致する記事が存在しません。".to_owned(),
                ))
            };
        }
        Ok(SearchPostsTemplate { context, page }.to_response())
    } else {
        let page = get_posts(&service.posts_repository, 10, query.page.unwrap_or(1))?;
        if page.posts.is_empty() {
            return Err(Error::NoResult(
                "このページには記事が存在しません。".to_owned(),
            ));
        }
        Ok(AllPostsTemplate { context, page }.to_response())
    }
}

pub async fn post_with_id(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Path<IdArguments>,
) -> Result<HttpResponse, Error> {
    let post_id = PostId(args.id);
    let page = get_post_with_id(&service.posts_repository, &post_id)?;
    Ok(PostTemplate { context, page }.to_response())
}

pub async fn posts_with_date(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Path<DateArguments>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let condition = args.into_inner().into();
    let page = get_posts_with_date_condition(
        &service.posts_repository,
        &condition,
        10,
        query.page.unwrap_or(1),
    )?;
    if page.posts.is_empty() {
        return Err(Error::NoResult(
            "この日付には記事が存在しません。".to_owned(),
        ));
    }
    Ok(PostsWithDateTemplate { context, page }.to_response())
}

mod templates {
    use crate::filters;
    use crate::{context::AppContext, presentation::body::Body};
    use askama::Template;
    use chrono::NaiveDate;
    use domain::entities::{
        date::DateCondition, AdjacentPage, KeywordsCondition, Page, Post, PostId,
    };
    use urlencoding::encode;

    #[derive(Template)]
    #[template(path = "all_posts.html")]
    pub struct AllPostsTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, ()>,
    }

    #[derive(Template)]
    #[template(path = "search_posts.html")]
    pub struct SearchPostsTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, KeywordsCondition<'a>>,
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
            Body::new(&self.body).to_html(true)
        }
    }

    trait KeywordsConditionExt {
        fn keywords(&self) -> String;
    }

    impl KeywordsConditionExt for KeywordsCondition<'_> {
        fn keywords(&self) -> String {
            self.0.join(" ")
        }
    }

    trait ConditionToString {
        fn to_string(&self) -> String;
    }

    impl ConditionToString for DateCondition {
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

    impl<'a> ConditionToString for KeywordsCondition<'a> {
        fn to_string(&self) -> String {
            format!("keywords({})", &self.0.join(", "))
        }
    }

    impl ConditionToString for () {
        fn to_string(&self) -> String {
            "".to_owned()
        }
    }

    trait ConditionToUrl {
        fn next_href(&self) -> Option<String>;
        fn prev_href(&self) -> Option<String>;
    }

    impl ConditionToUrl for Page<'_, DateCondition> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                AdjacentPage::Condition(ref condition) => {
                    Some(format!("/{}", condition.to_string()))
                }
                AdjacentPage::Page(page) => {
                    Some(format!("/{}?page={}", self.condition.to_string(), page))
                }
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                AdjacentPage::Condition(ref condition) => {
                    Some(format!("/{}", condition.to_string()))
                }
                AdjacentPage::Page(page) => {
                    Some(format!("/{}?page={}", self.condition.to_string(), page))
                }
                _ => None,
            }
        }
    }

    impl ConditionToUrl for Page<'_, ()> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                AdjacentPage::Page(page) => Some(format!("/?page={}", page)),
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                AdjacentPage::Page(page) => Some(format!("/?page={}", page)),
                AdjacentPage::Condition(_) => Some("/".to_owned()),
                _ => None,
            }
        }
    }

    impl ConditionToUrl for Page<'_, PostId> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                AdjacentPage::Condition(condition) => Some(format!("/{}", condition)),
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                AdjacentPage::Condition(condition) => Some(format!("/{}", condition)),
                _ => None,
            }
        }
    }

    impl ConditionToUrl for Page<'_, KeywordsCondition<'_>> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                AdjacentPage::Page(Some((a, b))) => Some(format!(
                    "/?keywords={}&search_after={}%2C{}",
                    encode(&self.condition.0.join(" ")),
                    a,
                    b
                )),
                AdjacentPage::Page(None) => Some(format!(
                    "/?keywords={}",
                    encode(&self.condition.0.join(" "))
                )),
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            None
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
