use anyhow::Result;
use domain::{entities::config::Config, repositories::config::ConfigRepository};
use infrastructure::{
    posts_repository_impl::PostsRepositoryImpl, search_repository_impl::SearchRepositoryImpl,
};

#[derive(Clone)] // FIXME: dieselのConnectionManagerがDebugを実装したらDebugにできる
pub struct Server {
    pub search_repository: SearchRepositoryImpl,
    pub posts_repository: PostsRepositoryImpl,
    pub config_repository: ConfigRepositoryImpl,
    pub admin_user_id: String,
}

impl Server {
    pub fn new(
        es_url: &url::Url,
        pg_url: &url::Url,
        admin_user: impl Into<String>,
    ) -> Result<Self> {
        let search_repository = SearchRepositoryImpl::new(es_url)?;
        let posts_repository = PostsRepositoryImpl::new(pg_url)?;
        Ok(Server {
            search_repository,
            posts_repository,
            config_repository: ConfigRepositoryImpl {},
            admin_user_id: admin_user.into(),
        })
    }

    pub fn authorize(&self, id: &str) -> bool {
        id == self.admin_user_id
    }
}

#[derive(Clone)]
pub struct ConfigRepositoryImpl {}

impl ConfigRepository for ConfigRepositoryImpl {
    fn get(&self) -> Result<Config> {
        Ok(Config {
            title: "andante".to_owned(),
            description: "個人的な日記".to_owned(),
            author: "κねこせん".to_owned(),
            email: "necocen@gmail.com".to_owned(),
            generator: "Nocturne v11".to_owned(),
            about: "単なる日記です\n\n\n単なる日記なんやで".to_string(),
            mathjax_options: "".to_string(),
            links: vec![],
        })
    }
}
