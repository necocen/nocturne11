use super::Topic;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body<'a>(Vec<Topic<'a>>);

impl Body<'_> {
    pub fn new(body: &str) -> Body {
        let separator = Regex::new(r"\n{3,}").unwrap();
        Body(separator.split(body.trim()).map(Topic::new).collect())
    }

    pub fn to_html(&self) -> String {
        self.0
            .iter()
            .map(Topic::to_html)
            .collect::<Vec<_>>()
            .join("\n<hr />\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn has_one_topic() {
        let body = "Topic 1";
        assert_eq!(Body::new(body).0, vec![Topic::new("Topic 1")]);
    }

    #[test]
    fn has_two_topics() {
        let body = "Topic 1\n\n\nTopic 2";
        assert_eq!(
            Body::new(body).0,
            vec![Topic::new("Topic 1"), Topic::new("Topic 2")]
        );
    }

    #[test]
    fn has_three_topics() {
        let body = "Topic 1\n\n\nTopic 2\n\n\nTopic 3";
        assert_eq!(
            Body::new(body).0,
            vec![
                Topic::new("Topic 1"),
                Topic::new("Topic 2"),
                Topic::new("Topic 3")
            ]
        );
    }

    #[test]
    fn has_topics_with_many_paragraphs() {
        let body = "Topic 1\n\n\nTopic 2 - Paragraph 1\n\nTopic 2 - Paragraph 2\n\nTopic 2 - Paragraph 3\n\n\nTopic 3 - Paragraph 1\n\nTopic 3 - Paragraph 2";
        assert_eq!(
            Body::new(body).0,
            vec![
                Topic::new("Topic 1"),
                Topic::new(
                    "Topic 2 - Paragraph 1\n\nTopic 2 - Paragraph 2\n\nTopic 2 - Paragraph 3"
                ),
                Topic::new("Topic 3 - Paragraph 1\n\nTopic 3 - Paragraph 2")
            ]
        );
    }

    #[test]
    fn has_many_linebreaks() {
        let body = "Topic 1\n\n\n\n\nTopic 2\n\n\nTopic 3\n\n\n\nTopic 4";
        assert_eq!(
            Body::new(body).0,
            vec![
                Topic::new("Topic 1"),
                Topic::new("Topic 2"),
                Topic::new("Topic 3"),
                Topic::new("Topic 4")
            ]
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode_and_print_html() {
        let body = include_str!("./fixtures/input.txt");
        let html = Body::new(body).to_html();
        let expected = include_str!("./fixtures/expected.txt");
        assert_eq!(html, expected);
    }
}
