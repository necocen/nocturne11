use super::Line;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'a>(Vec<Line<'a>>);

impl Paragraph<'_> {
    pub fn new(paragraph: &str) -> Paragraph {
        // MathJaxのディスプレイ数式を検出し、その中では数式モード行にする
        let mut math_mode = false;
        let lines = paragraph.split('\n').fold(vec![], |mut acc, line| {
            if math_mode {
                acc.push(Line::new_math(line));
                if line.trim_end().ends_with(r"\]") || line.trim_end().ends_with("$$") {
                    math_mode = false;
                }
            } else if line.trim_start().starts_with(r"\[") || line.trim_start().starts_with("$$") {
                math_mode = true;
                acc.push(Line::new_math(line));
            } else {
                acc.push(Line::new(line));
            }
            acc
        });
        Paragraph(lines)
    }

    pub fn to_html(&self, yakumono: bool) -> String {
        "<p>".to_owned()
            + &self
                .0
                .iter()
                .map(|line| line.to_html(yakumono))
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

    #[test]
    fn has_math_mode() {
        let body = "Line1\n\\[\nMathLineA\nMathLineB\n\\]\nLine2";
        assert_eq!(
            Paragraph::new(body).0,
            vec![
                Line::new("Line1"),
                Line::new_math(r"\["),
                Line::new_math("MathLineA"),
                Line::new_math("MathLineB"),
                Line::new_math(r"\]"),
                Line::new("Line2")
            ]
        );
    }

    #[test]
    fn has_dollar_math_mode() {
        let body = "Line1\n$$\nMathLineA\nMathLineB\n$$\nLine2";
        assert_eq!(
            Paragraph::new(body).0,
            vec![
                Line::new("Line1"),
                Line::new_math(r"$$"),
                Line::new_math("MathLineA"),
                Line::new_math("MathLineB"),
                Line::new_math(r"$$"),
                Line::new("Line2")
            ]
        );
    }
}
