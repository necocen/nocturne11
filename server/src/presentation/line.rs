use super::LineFragment;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line<'a>(Vec<LineFragment<'a>>);

impl Line<'_> {
    pub fn new(line: &str) -> Line {
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

    pub fn to_html(&self) -> String {
        self.0
            .iter()
            .map(LineFragment::to_html)
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use LineFragment::*;

    #[test]
    fn has_no_links() {
        assert_eq!(Line::new("LINE").0, vec![Text("LINE")]);
    }

    #[test]
    fn has_one_link() {
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
    fn has_many_links() {
        assert_eq!(
            Line::new("TEXT http://example.com http://example2.com TEXT").0,
            vec![
                Text("TEXT "),
                Link("http://example.com"),
                Text(" "),
                Link("http://example2.com"),
                Text(" TEXT")
            ]
        );

        assert_eq!(
            Line::new("TEXT http://example.com TEXT http://example2.com TEXT").0,
            vec![
                Text("TEXT "),
                Link("http://example.com"),
                Text(" TEXT "),
                Link("http://example2.com"),
                Text(" TEXT")
            ]
        );
    }
}
