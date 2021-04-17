use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{web, HttpResponse};
use askama::Template;
use domain::entities::Post;
use domain::use_cases::get_posts;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate<'a> {
    title: &'a str,
    posts: Vec<Post>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DateArguments {
    year: u16,
    month: u8,
    day: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdArguments {
    id: i32,
}

pub async fn all_posts(server: web::Data<Server>) -> Result<HttpResponse, Error> {
    PostsTemplate {
        posts: get_posts(&server.posts_repository)?,
        title: "タイトル",
    }
    .to_response()
}

pub async fn post_with_id(
    server: web::Data<Server>,
    args: web::Path<IdArguments>,
) -> Result<HttpResponse, Error> {
    dbg!(args);
    PostsTemplate {
        posts: get_posts(&server.posts_repository)?,
        title: "タイトル",
    }
    .to_response()
}
pub async fn posts_with_date(
    server: web::Data<Server>,
    args: web::Path<DateArguments>,
) -> Result<HttpResponse, Error> {
    dbg!(args);
    PostsTemplate {
        posts: get_posts(&server.posts_repository)?,
        title: "タイトル",
    }
    .to_response()
}

mod filters {
    use askama_escape::{escape, Html};
    use chrono::{DateTime, Local, Utc};
    use regex::Regex;

    pub fn format_date(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date.with_timezone(&Local).format("%F %T").to_string())
    }

    pub fn iso8601(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date.with_timezone(&Local).to_rfc3339())
    }

    pub fn post_body(body: &str) -> ::askama::Result<String> {
        let separator = Regex::new(r"\n{3,}").unwrap();
        let paragraph = Regex::new(r"\n\n").unwrap();
        Ok(separator
            .split(body)
            .map(|topic| {
                "<p>".to_owned()
                    + &paragraph
                        .split(topic)
                        .map(|paragraph| {
                            paragraph
                                .split("\n")
                                .map(convert_line)
                                .collect::<Vec<_>>()
                                .join("<br />")
                        })
                        .collect::<Vec<_>>()
                        .join("</p>\n<p>")
                    + "</p>"
            })
            .collect::<Vec<_>>()
            .join("\n<hr />\n"))
    }

    fn convert_line(line: &str) -> String {
        if line.len() == 0 {
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
            assert_eq!(post_body(body).unwrap(), "<p>Paragraph</p>");
        }

        #[test]
        fn has_linebreaks_in_a_paragraph() {
            let body = "Line1\nLine2\nLine3";
            assert_eq!(
                post_body(body).unwrap(),
                "<p>Line1<br />Line2<br />Line3</p>"
            );
        }

        #[test]
        fn has_two_paragraphs() {
            let body = "Paragraph 1\n\nParagraph 2";
            assert_eq!(
                post_body(body).unwrap(),
                "<p>Paragraph 1</p>\n<p>Paragraph 2</p>"
            );
        }

        #[test]
        fn has_three_paragraphs() {
            let body = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
            assert_eq!(
                post_body(body).unwrap(),
                "<p>Paragraph 1</p>\n<p>Paragraph 2</p>\n<p>Paragraph 3</p>"
            );
        }

        #[test]
        fn has_two_paragraphs_and_one_separator() {
            let body = "Paragraph 1\n\n\nParagraph 2";
            assert_eq!(
                post_body(body).unwrap(),
                "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>"
            );
        }

        #[test]
        fn has_three_paragraphs_and_two_separators() {
            let body = "Paragraph 1\n\n\nParagraph 2\n\n\nParagraph 3";
            assert_eq!(
                post_body(body).unwrap(),
                "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>\n<hr />\n<p>Paragraph 3</p>"
            );
        }

        #[test]
        fn has_four_paragraphs_and_two_separators() {
            let body = "Paragraph 1\n\n\nParagraph 2\n\nParagraph 3\n\n\nParagraph 4";
            assert_eq!(post_body(body).unwrap(), "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>\n<p>Paragraph 3</p>\n<hr />\n<p>Paragraph 4</p>");
        }

        #[test]
        fn has_many_linebreaks() {
            let body = "Paragraph 1\n\n\n\n\nParagraph 2";
            assert_eq!(
                post_body(body).unwrap(),
                "<p>Paragraph 1</p>\n<hr />\n<p>Paragraph 2</p>"
            );
        }
    }
}
