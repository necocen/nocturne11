use askama::Html;
use askama_escape::escape;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body<'a>(Vec<Topic<'a>>);

impl Body<'_> {
    pub fn new(body: &str) -> Body {
        let separator = Regex::new(r"\n{3,}").unwrap();
        Body(separator.split(body).map(Topic::new).collect())
    }

    pub fn to_html(&self) -> String {
        self.0
            .iter()
            .map(Topic::to_html)
            .collect::<Vec<_>>()
            .join("\n<hr />\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Topic<'a>(Vec<Paragraph<'a>>);

impl Topic<'_> {
    fn new(topic: &str) -> Topic {
        Topic(topic.split("\n\n").map(Paragraph::new).collect())
    }

    fn to_html(&self) -> String {
        self.0
            .iter()
            .map(Paragraph::to_html)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Paragraph<'a>(Vec<Line<'a>>);

impl Paragraph<'_> {
    fn new(paragraph: &str) -> Paragraph {
        Paragraph(paragraph.split('\n').map(Line::new).collect())
    }

    fn to_html(&self) -> String {
        "<p>".to_owned()
            + &self
                .0
                .iter()
                .map(Line::to_html)
                .collect::<Vec<_>>()
                .join("<br />")
            + "</p>"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line<'a>(Vec<LineFragment<'a>>);

impl Line<'_> {
    fn new(line: &str) -> Line {
        if line.is_empty() {
            return Line(vec![]);
        }
        let url_pattern = Regex::new(r"https?://[-_.!~*'()a-zA-Z0-9;/?:@&=+$,%#]+").unwrap();
        let mut pos: usize = 0;
        let mut fragments: Vec<LineFragment> = vec![];
        for m in url_pattern.find_iter(line) {
            fragments.push(LineFragment::Text(&line[pos..m.start()]));
            fragments.push(LineFragment::Link(m.as_str()));
            pos = m.end();
        }
        fragments.push(LineFragment::Text(&line[pos..]));
        Line(
            fragments
                .into_iter()
                .flat_map(LineFragment::into_split)
                .collect(),
        )
    }

    fn to_html(&self) -> String {
        self.0
            .iter()
            .map(LineFragment::to_html)
            .collect::<Vec<_>>()
            .join("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LineFragment<'a> {
    /// 約物を含まない文字のひとかたまり
    Text(&'a str),
    /// <a>タグで囲まれるべきURL
    Link(&'a str),
    /// 開き括弧としてタグづけされる文字
    OpenBracket(&'a str),
    /// 閉じ括弧としてタグづけされる文字
    CloseBracket(&'a str),
    /// 句読点としてタグづけされる文字
    Punctuation(&'a str),
    /// 中黒としてタグづけされる文字
    Interpunct(&'a str),
    /// その他の約物としてタグづけされる文字
    Other(&'a str),
}

impl LineFragment<'_> {
    fn to_html(&self) -> String {
        match self {
            LineFragment::Text(text) => {
                format!(
                    r#"<span class="non-yakumono">{}</span>"#,
                    escape(text, Html)
                )
            }
            LineFragment::Link(link) => {
                format!(
                    r#"<a href="{}" rel="external">{}</a>"#,
                    link,
                    escape(link, Html)
                )
            }
            LineFragment::OpenBracket(c) => {
                format!(r#"<span class="yakumono-open-bracket">{}</span>"#, c)
            }
            LineFragment::CloseBracket(c) => {
                format!(r#"<span class="yakumono-close-bracket">{}</span>"#, c)
            }
            LineFragment::Punctuation(c) => {
                format!(r#"<span class="yakumono-punctuation">{}</span>"#, c)
            }
            LineFragment::Interpunct(c) => {
                format!(r#"<span class="yakumono-interpunct">{}</span>"#, c)
            }
            LineFragment::Other(c) => {
                format!(r#"<span class="yakumono-other">{}</span>"#, c)
            }
        }
    }
}

impl<'a> LineFragment<'a> {
    fn into_split(self) -> Vec<LineFragment<'a>> {
        use LineFragment::*;
        match self {
            Text(text) => {
                let mut pos = 0;
                let mut fragments: Vec<LineFragment> = vec![];
                for (c_start, c) in text.char_indices().into_iter() {
                    let fragment: LineFragment;
                    let c_end = c_start + c.len_utf8();
                    if "「『（〈【｛［《〔“".contains(c) {
                        fragment = OpenBracket(&text[c_start..c_end]);
                    } else if "」』）〉】｝］》〕”".contains(c) {
                        fragment = CloseBracket(&text[c_start..c_end]);
                    } else if "、。，．".contains(c) {
                        fragment = Punctuation(&text[c_start..c_end]);
                    } else if "・".contains(c) {
                        fragment = Interpunct(&text[c_start..c_end]);
                    } else if "／＼！？".contains(c) {
                        fragment = Other(&text[c_start..c_end]);
                    } else {
                        // 通常文字列の場合は一旦スルーする
                        continue;
                    }
                    // 通常文字列以外なので積んである通常文字列を先に積む
                    if pos < c_start {
                        fragments.push(Text(&text[pos..c_start]));
                    }
                    fragments.push(fragment);
                    pos = c_end;
                }
                if pos < text.len() {
                    // 通常文字列で終わる場合は積み残しがある
                    fragments.push(Text(&text[pos..]));
                }
                fragments
            }
            _ => vec![self],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use LineFragment::*;

    #[test]
    fn has_no_links_in_a_line() {
        assert_eq!(Line::new("LINE").0, vec![Text("LINE")]);
    }

    #[test]
    fn has_link_in_a_line() {
        assert_eq!(
            Line::new("LINE http://example.com LINE").0,
            vec![Text("LINE "), Link("http://example.com"), Text(" LINE")]
        );
        assert_eq!(
            Line::new("LINE http://example.com/?query=value LINE").0,
            vec![
                Text("LINE "),
                Link("http://example.com/?query=value"),
                Text(" LINE")
            ]
        );
        assert_eq!(
            Line::new("LINE http://example.com/path LINE").0,
            vec![
                Text("LINE "),
                Link("http://example.com/path"),
                Text(" LINE")
            ]
        );
        assert_eq!(
            Line::new("LINE https://example.com/path LINE").0,
            vec![
                Text("LINE "),
                Link("https://example.com/path"),
                Text(" LINE")
            ]
        );
    }

    #[test]
    fn has_one_paragraph() {
        let body = "Paragraph";
        assert_eq!(
            Body::new(body),
            Body(vec![Topic(vec![Paragraph(vec![Line(vec![Text(
                "Paragraph"
            )])])])])
        );
    }

    #[test]
    fn has_linebreaks_in_a_paragraph() {
        let body = "Line1\nLine2\nLine3";
        assert_eq!(
            Body::new(body),
            Body(vec![Topic(vec![Paragraph(vec![
                Line(vec![Text("Line1")]),
                Line(vec![Text("Line2")]),
                Line(vec![Text("Line3")])
            ])])])
        );
    }

    #[test]
    fn has_two_paragraphs() {
        let body = "Paragraph 1\n\nParagraph 2";
        assert_eq!(
            Body::new(body),
            Body(vec![Topic(vec![
                Paragraph(vec![Line(vec![Text("Paragraph 1")])]),
                Paragraph(vec![Line(vec![Text("Paragraph 2")])])
            ])])
        );
    }

    #[test]
    fn has_three_paragraphs() {
        let body = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
        assert_eq!(
            Body::new(body),
            Body(vec![Topic(vec![
                Paragraph(vec![Line(vec![Text("Paragraph 1")])]),
                Paragraph(vec![Line(vec![Text("Paragraph 2")])]),
                Paragraph(vec![Line(vec![Text("Paragraph 3")])])
            ])])
        );
    }

    #[test]
    fn has_two_paragraphs_and_one_separator() {
        let body = "Paragraph 1\n\n\nParagraph 2";
        assert_eq!(
            Body::new(body),
            Body(vec![
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 1")])])]),
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 2")])])])
            ])
        );
    }

    #[test]
    fn has_three_paragraphs_and_two_separators() {
        let body = "Paragraph 1\n\n\nParagraph 2\n\n\nParagraph 3";
        assert_eq!(
            Body::new(body),
            Body(vec![
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 1")])])]),
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 2")])])]),
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 3")])])])
            ])
        );
    }

    #[test]
    fn has_four_paragraphs_and_two_separators() {
        let body = "Paragraph 1\n\n\nParagraph 2\n\nParagraph 3\n\n\nParagraph 4";
        assert_eq!(
            Body::new(body),
            Body(vec![
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 1")])])]),
                Topic(vec![
                    Paragraph(vec![Line(vec![Text("Paragraph 2")])]),
                    Paragraph(vec![Line(vec![Text("Paragraph 3")])])
                ]),
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 4")])])])
            ])
        );
    }

    #[test]
    fn has_many_linebreaks() {
        let body = "Paragraph 1\n\n\n\n\nParagraph 2";
        assert_eq!(
            Body::new(body),
            Body(vec![
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 1")])])]),
                Topic(vec![Paragraph(vec![Line(vec![Text("Paragraph 2")])])])
            ])
        );
    }
}
