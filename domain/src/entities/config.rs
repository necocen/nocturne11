use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub site: Site,
    pub author: Author,
    pub mathjax: MathJax,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Site {
    pub title: String,
    pub description: String,
    pub generator: String,
    pub about: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MathJax {
    pub options: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Link {
    pub name: String,
    pub url: url::Url,
    pub active: bool,
}
