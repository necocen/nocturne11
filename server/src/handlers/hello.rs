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
}
