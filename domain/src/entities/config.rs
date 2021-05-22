use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub site: Site,
    pub author: Author,
    pub mathjax: MathJax,
    pub auth: AuthenticationSettings,
    pub hatena_star_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Site {
    pub title: String,
    pub description: String,
    pub generator: String,
    pub about: String,
    pub url: url::Url,
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

#[derive(Debug, Clone, Deserialize)]
pub struct AuthenticationSettings {
    /// Sign In With GoogleのクライアントID
    pub google_client_id: String,
    /// 管理者になるユーザーのGoogle User ID
    pub admin_user_id: String,
}
