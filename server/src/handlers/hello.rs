use super::Error;
use super::TemplateToResponse;
use crate::server::Server;
use actix_web::{web, HttpResponse};
use askama::Template;
use domain::entities::Post;
use domain::use_cases::get_posts;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    title: &'a str,
    posts: Vec<Post>,
}

pub async fn hello(server: web::Data<Server>) -> Result<HttpResponse, Error> {
    HelloTemplate {
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
        let separator = Regex::new("\\n{3,}").unwrap();
        let paragraph = Regex::new("\\n\\n").unwrap();
        Ok(separator
            .split(body)
            .map(|topic| {
                "<p>".to_owned()
                    + &paragraph
                        .split(topic)
                        .map(|paragraph| {
                            paragraph
                                .split("\n")
                                .map(|line| escape(line, Html).to_string())
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

    #[cfg(test)]
    mod tests {
        use super::*;

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
