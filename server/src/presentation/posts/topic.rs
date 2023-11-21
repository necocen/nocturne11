use super::Paragraph;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topic<'a>(Vec<Paragraph<'a>>);

impl Topic<'_> {
    pub fn new(topic: &str) -> Topic {
        Topic(topic.split("\n\n").map(Paragraph::new).collect())
    }

    pub fn to_html(&self, yakumono: bool) -> String {
        self.0
            .iter()
            .map(|p| p.to_html(yakumono))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn has_two_paragraphs() {
        let topic = "Paragraph 1\n\nParagraph 2";
        assert_eq!(
            Topic::new(topic).0,
            vec![Paragraph::new("Paragraph 1"), Paragraph::new("Paragraph 2")]
        );
    }

    #[test]
    fn has_three_paragraphs() {
        let topic = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
        assert_eq!(
            Topic::new(topic).0,
            vec![
                Paragraph::new("Paragraph 1"),
                Paragraph::new("Paragraph 2"),
                Paragraph::new("Paragraph 3"),
            ]
        );
    }

    #[test]
    fn has_paragraph_with_linebreak() {
        let topic = "Paragraph 1 - Line 1\nParagraph 1 - Line 2\n\nParagraph 2\n\nParagraph 3 - Line 1\nParagraph 3 - Line 2\nParagraph 3 - Line 3";
        assert_eq!(
            Topic::new(topic).0,
            vec![
                Paragraph::new("Paragraph 1 - Line 1\nParagraph 1 - Line 2"),
                Paragraph::new("Paragraph 2"),
                Paragraph::new("Paragraph 3 - Line 1\nParagraph 3 - Line 2\nParagraph 3 - Line 3"),
            ]
        );
    }
}
