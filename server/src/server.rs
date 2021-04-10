use anyhow::Result;
use infrastructure::{
    posts_repository_impl::PostsRepositoryImpl, search_repository_impl::SearchRepositoryImpl,
};

#[derive(Clone)]
pub struct Server {
    pub search_repository: SearchRepositoryImpl,
    pub posts_repository: PostsRepositoryImpl,
}

impl Server {
    pub fn new(es_url: url::Url, pg_url: url::Url) -> Result<Self> {
        let search_repository = SearchRepositoryImpl::new(es_url)?;
        let posts_repository = PostsRepositoryImpl::new(pg_url.to_string());
        Ok(Server {
            search_repository,
            posts_repository,
        })
    }
}
