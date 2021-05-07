use anyhow::Result;
use infrastructure::{
    config_repository_partial_impl::ConfigRepositoryPartialImpl,
    pg_repository_impls::pg_repositories, posts_repository_impl::PostsRepositoryImpl,
    search_repository_impl::SearchRepositoryImpl,
};

#[derive(Clone)] // FIXME: dieselのConnectionManagerがDebugを実装したらDebugにできる
pub struct Server {
    pub search_repository: SearchRepositoryImpl,
    pub posts_repository: PostsRepositoryImpl,
    pub config_repository: ConfigRepositoryPartialImpl,
    pub admin_user_id: String,
}

impl Server {
    pub fn new(
        es_url: &url::Url,
        pg_url: &url::Url,
        admin_user: impl Into<String>,
    ) -> Result<Self> {
        let search_repository = SearchRepositoryImpl::new(es_url)?;
        let (posts_repository, config_repository) = pg_repositories(pg_url)?;
        Ok(Server {
            search_repository,
            posts_repository,
            config_repository,
            admin_user_id: admin_user.into(),
        })
    }

    pub fn authorize(&self, id: &str) -> bool {
        id == self.admin_user_id
    }
}
