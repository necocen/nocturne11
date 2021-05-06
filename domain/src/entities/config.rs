#[derive(Debug, Clone)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub author: String,
    pub email: String,
    pub generator: String,
    pub about: String,
    pub mathjax_options: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub name: String,
    pub url: url::Url,
    pub is_old: bool,
}
