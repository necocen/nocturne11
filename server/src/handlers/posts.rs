use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{web, HttpResponse};
use domain::entities::date::{DateCondition, YearMonth};
use domain::use_cases::{get_post_with_id, get_posts, get_posts_with_day};
use serde::Deserialize;
use templates::{AllPostsTemplate, PostTemplate, PostsWithDateTemplate};

#[derive(Debug, Clone, Deserialize)]
pub(super) struct DateArguments {
    year: u16,
    month: u8,
    day: Option<u8>,
}

impl From<DateArguments> for DateCondition {
    fn from(args: DateArguments) -> DateCondition {
        DateCondition {
            ym: YearMonth(args.year, args.month),
            day: args.day,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct IdArguments {
    id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct PageQuery {
    page: Option<usize>,
}

pub(super) async fn all_posts(server: web::Data<Server>) -> Result<HttpResponse, Error> {
    AllPostsTemplate {
        posts: get_posts(&server.posts_repository)?,
        title: "タイトル",
    }
    .to_response()
}

pub(super) async fn post_with_id(
    server: web::Data<Server>,
    args: web::Path<IdArguments>,
) -> Result<HttpResponse, Error> {
    let (post, has_next) = get_post_with_id(&server.posts_repository, args.id)?;
    dbg!(has_next);
    PostTemplate {
        post,
        title: "タイトル",
    }
    .to_response()
}
pub(super) async fn posts_with_date(
    server: web::Data<Server>,
    args: web::Path<DateArguments>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let condition = args.to_owned().into();
    let page = get_posts_with_day(
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
    pub(super) use super::super::filters;
    use askama::Template;
    use askama_escape::{escape, Html};
    use chrono::NaiveDate;
    use domain::entities::{date::DateCondition, NextPage, Page, Post};
    use regex::Regex;

    #[derive(Template)]
    #[template(path = "all_posts.html")]
    pub(super) struct AllPostsTemplate<'a> {
        pub(super) title: &'a str,
        pub(super) posts: Vec<Post>,
    }

    #[derive(Template)]
    #[template(path = "posts_with_date.html")]
    pub(super) struct PostsWithDateTemplate<'a> {
        pub(super) title: &'a str,
        pub(super) page: Page<'a, DateCondition>,
    }

    #[derive(Template)]
    #[template(path = "post.html")]
    pub(super) struct PostTemplate<'a> {
        pub(super) title: &'a str,
        pub(super) post: Post,
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

    fn convert_body(body: &str) -> String {
        let separator = Regex::new(r"\n{3,}").unwrap();
        separator
            .split(body)
            .map(|topic| {
                "<p>".to_owned()
                    + &topic
                        .split("\n\n")
                        .map(|paragraph| {
                            paragraph
                                .split('\n')
                                .map(convert_line)
                                .collect::<Vec<_>>()
                                .join("<br />")
                        })
                        .collect::<Vec<_>>()
                        .join("</p>\n<p>")
                    + "</p>"
            })
            .collect::<Vec<_>>()
            .join("\n<hr />\n")
    }

    fn convert_line(line: &str) -> String {
        if line.is_empty() {
            return "".to_string();
        }
        let url_pattern = Regex::new(r"https?://[-_.!~*'()a-zA-Z0-9;/?:@&=+$,%#]+").unwrap();
        let mut pos: usize = 0;
        let mut fragments: Vec<String> = vec![];
        for m in url_pattern.find_iter(line) {
            fragments.push(escape(&line[pos..m.start()], Html).to_string());
            fragments.push(
                "<a href=\"".to_owned()
                    + m.as_str()
                    + "\" rel=\"external\">"
                    + &escape(m.as_str(), Html).to_string()
                    + "</a>",
            );
            pos = m.end();
        }
        fragments.push(escape(&line[pos..], Html).to_string());
        fragments.join("")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use domain::entities::date::*;

        #[test]
        fn has_no_links_in_a_line() {
            assert_eq!(convert_line("LINE"), "LINE");
        }

        #[test]
        fn has_link_in_a_line() {
            assert_eq!(convert_line("LINE http://example.com LINE"), "LINE <a href=\"http://example.com\" rel=\"external\">http:&#x2f;&#x2f;example.com</a> LINE");
            assert_eq!(convert_line("LINE http://example.com/?query=value LINE"), "LINE <a href=\"http://example.com/?query=value\" rel=\"external\">http:&#x2f;&#x2f;example.com&#x2f;?query=value</a> LINE");
            assert_eq!(convert_line("LINE http://example.com/path LINE"), "LINE <a href=\"http://example.com/path\" rel=\"external\">http:&#x2f;&#x2f;example.com&#x2f;path</a> LINE");
            assert_eq!(convert_line("LINE https://example.com/path LINE"), "LINE <a href=\"https://example.com/path\" rel=\"external\">https:&#x2f;&#x2f;example.com&#x2f;path</a> LINE");
        }

        #[test]
        fn has_one_paragraph() {
            let body = "Paragraph";
            assert_eq!(convert_body(body), "<p>Paragraph</p>");
        }

        #[test]
        fn has_linebreaks_in_a_paragraph() {
            let body = "Line1\nLine2\nLine3";
            assert_eq!(convert_body(body), "<p>Line1<br />Line2<br />Line3</p>");
        }

        #[test]
        fn has_two_paragraphs() {
            let body = "Paragraph 1\n\nParagraph 2";
            assert_eq!(convert_body(body), "<p>Paragraph 1</p>\n<p>Paragraph 2</p>");
        }

        #[test]
        fn has_three_paragraphs() {
            let body = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
            assert_eq!(
                convert_body(body),
                "<p>Paragraph 1</p>\n<p>Paragraph 2</p>\n<p>Paragraph 3</p>"
            );
        }

        #[test]
        fn has_two_paragraphs_and_one_separator() {
            let body = "Paragraph 1\n\n\nParagraph 2";
            assert_eq!(
                convert_body(body),
                "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>"
            );
        }

        #[test]
        fn has_three_paragraphs_and_two_separators() {
            let body = "Paragraph 1\n\n\nParagraph 2\n\n\nParagraph 3";
            assert_eq!(
                convert_body(body),
                "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>\n<hr />\n<p>Paragraph 3</p>"
            );
        }

        #[test]
        fn has_four_paragraphs_and_two_separators() {
            let body = "Paragraph 1\n\n\nParagraph 2\n\nParagraph 3\n\n\nParagraph 4";
            assert_eq!(convert_body(body), "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>\n<p>Paragraph 3</p>\n<hr />\n<p>Paragraph 4</p>");
        }

        #[test]
        fn has_many_linebreaks() {
            let body = "Paragraph 1\n\n\n\n\nParagraph 2";
            assert_eq!(
                convert_body(body),
                "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>"
            );
        }

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
