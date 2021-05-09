use super::Line;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'a>(Vec<Line<'a>>);

impl Paragraph<'_> {
    pub fn new(paragraph: &str) -> Paragraph {
        Paragraph(paragraph.split('\n').map(Line::new).collect())
    }

    pub fn to_html(&self) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn has_one_line() {
        let paragraph = "Paragraph";
        assert_eq!(Paragraph::new(paragraph).0, vec![Line::new("Paragraph")]);
    }

    #[test]
    fn has_many_lines() {
        let body = "Line1\nLine2\nLine3";
        assert_eq!(
            Paragraph::new(body).0,
            vec![Line::new("Line1"), Line::new("Line2"), Line::new("Line3"),]
        );
    }
}
