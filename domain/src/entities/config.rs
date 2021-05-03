pub struct Config {
    pub about: String,
    pub mathjax_options: String,
    pub links: Vec<Link>,
}

pub struct Link {
    pub name: String,
    pub url: url::Url,
    pub is_old: bool,
}
