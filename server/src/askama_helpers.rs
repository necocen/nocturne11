use crate::Error;
use actix_web::HttpResponse;
use askama::Html;
use askama_escape::escape;
use bytes::BytesMut;
use regex::Regex;

pub trait TemplateToResponse {
    fn to_response(&self) -> Result<HttpResponse, Error>;
}

impl<T: askama::Template> TemplateToResponse for T {
    fn to_response(&self) -> Result<HttpResponse, Error> {
        let mut buffer = BytesMut::with_capacity(self.size_hint());
        self.render_into(&mut buffer)?;

        let content_type =
            askama::mime::extension_to_mime_type(self.extension().unwrap_or("txt")).to_string();
        Ok(HttpResponse::Ok()
            .content_type(content_type.as_str())
            .body(buffer.freeze()))
    }
}

pub mod filters {
    use chrono::{DateTime, Local, Utc};

    pub fn format_date(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date.with_timezone(&Local).format("%F %T").to_string())
    }

    pub fn iso8601(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date.with_timezone(&Local).to_rfc3339())
    }
}

pub fn convert_body(body: &str) -> String {
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
    use pretty_assertions::assert_eq;

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
}
