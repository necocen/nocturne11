use super::args::{DateArguments, IdArguments, KeywordsQuery, PageQuery, YearMonthArguments};
use crate::context::AppContext;
use crate::{Error, Service};
use actix_web::{web, HttpResponse};
use application::models::YearMonth;
use application::use_cases::{
    GetLatestPostsUseCase, GetPostByIdUseCase, GetPostsByDateUseCase, GetPostsByYearMonthUseCase,
    SearchPostsUseCase,
};
use askama_actix::TemplateToResponse;
use chrono::NaiveDate;
use domain::entities::PostId;
use templates::{
    AllPostsTemplate, PostTemplate, PostsWithDateTemplate, PostsWithYearMonthTemplate,
    SearchPostsTemplate,
};

pub async fn all_posts(
    context: AppContext,
    service: web::Data<Service>,
    query: web::Query<KeywordsQuery>,
) -> Result<HttpResponse, Error> {
    if let Some(keywords) = &query.keywords {
        let keywords = keywords.split_whitespace().collect::<Vec<_>>();
        let page = SearchPostsUseCase::execute(
            &service.search_client,
            &service.posts_repository,
            &keywords,
            query.page.unwrap_or(1),
        )
        .await?;
        if page.posts.is_empty() {
            return Err(Error::NoResult(
                "このページには記事が存在しません。".to_owned(),
            ));
        }
        Ok(SearchPostsTemplate { context, page }.to_response())
    } else {
        let page = GetLatestPostsUseCase::execute(
            &service.posts_repository,
            &service.search_client,
            query.page.unwrap_or(1),
        )
        .await?;
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
    let page =
        GetPostByIdUseCase::execute(&service.posts_repository, &service.search_client, &post_id)
            .await?;
    Ok(PostTemplate { context, page }.to_response())
}

pub async fn posts_with_date(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Path<DateArguments>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let date: NaiveDate = args.into_inner().try_into()?; // TODO: map to 404
    let page = GetPostsByDateUseCase::execute(
        &service.posts_repository,
        &service.search_client,
        &date,
        query.page.unwrap_or(1),
    )
    .await?;
    if page.posts.is_empty() {
        return Err(Error::NoResult(
            "この日付には記事が存在しません。".to_owned(),
        ));
    }
    Ok(PostsWithDateTemplate { context, page }.to_response())
}

pub async fn posts_with_year_month(
    context: AppContext,
    service: web::Data<Service>,
    args: web::Path<YearMonthArguments>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let year_month: YearMonth = args.into_inner().try_into()?;
    let page = GetPostsByYearMonthUseCase::execute(
        &service.posts_repository,
        &service.search_client,
        &year_month,
        query.page.unwrap_or(1),
    )
    .await?;
    if page.posts.is_empty() {
        return Err(Error::NoResult(
            "この日付には記事が存在しません。".to_owned(),
        ));
    }
    Ok(PostsWithYearMonthTemplate { context, page }.to_response())
}

mod templates {
    use crate::filters;
    use crate::{context::AppContext, presentation::posts::Body};
    use application::models::{AdjacentPageInfo, Page, YearMonth};
    use askama::Template;
    use chrono::NaiveDate;
    use domain::entities::{Post, PostId};
    use urlencoding::encode;

    #[derive(Template)]
    #[template(path = "all_posts.html")]
    pub struct AllPostsTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, (), usize>,
    }

    #[derive(Template)]
    #[template(path = "search_posts.html")]
    pub struct SearchPostsTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, Vec<&'a str>, usize>,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct PostsWithYearMonthTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, YearMonth, usize>,
    }
    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct PostsWithDateTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, NaiveDate, usize>,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct PostTemplate<'a> {
        pub context: AppContext,
        pub page: Page<'a, PostId, ()>,
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

    impl KeywordsConditionExt for Vec<&str> {
        fn keywords(&self) -> String {
            self.join(" ")
        }
    }

    trait ConditionToString {
        fn to_string(&self) -> String;
    }

    impl ConditionToString for YearMonth {
        fn to_string(&self) -> String {
            format!("{:04}-{:02}", self.year, self.month)
        }
    }

    impl ConditionToString for Vec<&str> {
        fn to_string(&self) -> String {
            format!("keywords({})", self.join(", "))
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

    impl ConditionToUrl for Page<'_, (), usize> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                Some(AdjacentPageInfo::PageIndex(page)) => Some(format!("/?page={}", page)),
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                Some(AdjacentPageInfo::PageIndex(page)) => Some(format!("/?page={}", page)),
                _ => None,
            }
        }
    }

    impl ConditionToUrl for Page<'_, PostId, ()> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                Some(AdjacentPageInfo::Condition(post_id)) => Some(format!("/{post_id}")),
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                Some(AdjacentPageInfo::Condition(post_id)) => Some(format!("/{post_id}")),
                _ => None,
            }
        }
    }

    impl ConditionToUrl for Page<'_, Vec<&str>, usize> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                Some(AdjacentPageInfo::PageIndex(page)) => Some(format!(
                    "/?keywords={}&page={}",
                    encode(&self.condition.join(" ")),
                    page
                )),
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                Some(AdjacentPageInfo::PageIndex(page)) => Some(format!(
                    "/?keywords={}&page={}",
                    encode(&self.condition.join(" ")),
                    page
                )),
                _ => None,
            }
        }
    }

    impl ConditionToUrl for Page<'_, NaiveDate, usize> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                Some(AdjacentPageInfo::Condition(ref date)) => {
                    Some(format!("/{}?page={}", date.format("%Y-%m-%d"), self.index))
                }
                Some(AdjacentPageInfo::PageIndex(page)) => Some(format!(
                    "/{}?page={}",
                    self.condition.format("%Y-%m-%d"),
                    page
                )),
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                Some(AdjacentPageInfo::Condition(ref date)) => {
                    Some(format!("/{}", date.format("%Y-%m-%d")))
                }
                Some(AdjacentPageInfo::PageIndex(page)) => Some(format!(
                    "/{}?page={}",
                    self.condition.format("%Y-%m-%d"),
                    page
                )),
                _ => None,
            }
        }
    }

    impl ConditionToUrl for Page<'_, YearMonth, usize> {
        fn next_href(&self) -> Option<String> {
            match self.next_page {
                Some(AdjacentPageInfo::Condition(ym)) => Some(format!("/{}", ym.to_string())),
                Some(AdjacentPageInfo::PageIndex(page)) => {
                    Some(format!("/{}?page={}", self.condition.to_string(), page))
                }
                _ => None,
            }
        }

        fn prev_href(&self) -> Option<String> {
            match self.prev_page {
                Some(AdjacentPageInfo::Condition(ym)) => Some(format!("/{}", ym.to_string())),
                Some(AdjacentPageInfo::PageIndex(page)) => {
                    Some(format!("/{}?page={}", self.condition.to_string(), page))
                }
                _ => None,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn year_month_to_string() {
            let condition = YearMonth::new(1989, 9).unwrap();
            assert_eq!(condition.to_string(), "1989-09");
        }
    }
}
