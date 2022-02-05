use askama::Html;
use askama_escape::escape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineFragment<'a> {
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
    pub fn to_html(&self, yakumono: bool) -> String {
        match self {
            LineFragment::Link(link) => {
                format!(
                    r#"<a href="{}" rel="external">{}</a>"#,
                    link,
                    escape(link, Html)
                )
            }
            LineFragment::Text(text) => {
                if yakumono {
                    format!(r"<span>{}</span>", escape(text, Html))
                } else {
                    escape(text, Html).to_string()
                }
            }
            LineFragment::OpenBracket(c) => {
                if yakumono {
                    format!(r#"<span class="yakumono-open-bracket">{}</span>"#, c)
                } else {
                    c.to_string()
                }
            }
            LineFragment::CloseBracket(c) => {
                if yakumono {
                    format!(r#"<span class="yakumono-close-bracket">{}</span>"#, c)
                } else {
                    c.to_string()
                }
            }
            LineFragment::Punctuation(c) => {
                if yakumono {
                    format!(r#"<span class="yakumono-punctuation">{}</span>"#, c)
                } else {
                    c.to_string()
                }
            }
            LineFragment::Interpunct(c) => {
                if yakumono {
                    format!(r#"<span class="yakumono-interpunct">{}</span>"#, c)
                } else {
                    c.to_string()
                }
            }
            LineFragment::Other(c) => {
                if yakumono {
                    format!(r#"<span class="yakumono-other">{}</span>"#, c)
                } else {
                    c.to_string()
                }
            }
        }
    }
}

impl<'a> LineFragment<'a> {
    pub fn into_split(self) -> Vec<LineFragment<'a>> {
        use LineFragment::*;
        match self {
            Text(text) => {
                let mut pos = 0;
                let mut fragments: Vec<LineFragment> = vec![];
                for (c_start, c) in text.char_indices() {
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
