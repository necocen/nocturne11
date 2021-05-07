use anyhow::{ensure, Result};
use infrastructure::{
    config_repository_impl::{ConfigRepositoryImpl, Version},
    posts_repository_impl::PostsRepositoryImpl,
    search_repository_impl::SearchRepositoryImpl,
};
use std::{env, path::PathBuf};

#[derive(Clone)] // FIXME: dieselのConnectionManagerがDebugを実装したらDebugにできる
pub struct Server {
    pub search_repository: SearchRepositoryImpl,
    pub posts_repository: PostsRepositoryImpl,
    pub config_repository: ConfigRepositoryImpl,
    pub admin_user_id: String,
    pub secret_key: String,
    pub static_path: PathBuf,
}

impl Server {
    pub fn new() -> Result<Self> {
        let es_url = url::Url::parse(&env::var("ES_URL")?)?;
        let pg_url = url::Url::parse(&env::var("DATABASE_URL")?)?;
        let admin_user_id = env::var("ADMIN_USER_ID")?;
        let static_path: PathBuf = "./frontend/build/src".into();
        let secret_key = env::var("SECRET_KEY")?;
        ensure!(secret_key.len() >= 32, "SECRET_KEY is not long enough.");

        let version = Version {
            version: env!("VERGEN_BUILD_SEMVER"),
            timestamp: env!("VERGEN_BUILD_TIMESTAMP"),
        };
        let config_toml = include_str!("../../config.toml");

        let search_repository = SearchRepositoryImpl::new(&es_url)?;
        let posts_repository = PostsRepositoryImpl::new(&pg_url)?;
        let config_repository = ConfigRepositoryImpl::new(config_toml, &version)?;

        Ok(Server {
            search_repository,
            posts_repository,
            config_repository,
            admin_user_id,
            secret_key,
            static_path,
        })
    }

    pub fn authorize(&self, id: &str) -> bool {
        id == self.admin_user_id
    }
}
