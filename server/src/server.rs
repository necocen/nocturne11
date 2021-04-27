use anyhow::Result;
use infrastructure::{
    posts_repository_impl::PostsRepositoryImpl, search_repository_impl::SearchRepositoryImpl,
};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Server {
    pub search_repository: SearchRepositoryImpl,
    pub posts_repository: PostsRepositoryImpl,
    pub static_path: PathBuf,
    pub admin_user_id: String,
}

impl Server {
    pub fn new(
        es_url: &url::Url,
        pg_url: &url::Url,
        static_path: impl Into<PathBuf>,
        admin_user: impl Into<String>,
    ) -> Result<Self> {
        let search_repository = SearchRepositoryImpl::new(es_url)?;
        let posts_repository = PostsRepositoryImpl::new(pg_url)?;
        Ok(Server {
            search_repository,
            posts_repository,
            static_path: static_path.into(),
            admin_user_id: admin_user.into(),
        })
    }
}
