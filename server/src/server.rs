use anyhow::Result;
use domain::{entities::config::Config, repositories::config::ConfigRepository};
use infrastructure::{
    config_repository_partial_impl::{ConfigRepositoryPartialImpl, PartialConfig},
    pg_repository_impls::pg_repositories,
    posts_repository_impl::PostsRepositoryImpl,
    search_repository_impl::SearchRepositoryImpl,
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
        let (posts_repository, config_repository_partial) = pg_repositories(pg_url)?;
        Ok(Server {
            search_repository,
            posts_repository,
            config_repository: ConfigRepositoryImpl {
                partial: config_repository_partial,
            },
            admin_user_id: admin_user.into(),
        })
    }

    pub fn authorize(&self, id: &str) -> bool {
        id == self.admin_user_id
    }
}

#[derive(Clone)]
pub struct ConfigRepositoryImpl {
    partial: ConfigRepositoryPartialImpl,
}

impl ConfigRepository for ConfigRepositoryImpl {
    fn get(&self) -> Result<Config> {
        let PartialConfig {
            about,
            mathjax_options,
            links,
        } = self.partial.get_partial()?;

        Ok(Config {
            title: "andante".to_owned(),
            description: "個人的な日記".to_owned(),
            author: "κねこせん".to_owned(),
            email: "necocen@gmail.com".to_owned(),
            generator: format!("Nocturne v{} {}", env!("VERGEN_BUILD_SEMVER"), env!("VERGEN_BUILD_TIMESTAMP")),
            about,
            mathjax_options,
            links,
        })
    }

    fn set_about(&self, about: &str) -> Result<()> {
        self.partial.set_about(about)
    }
}
