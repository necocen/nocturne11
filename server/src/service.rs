use anyhow::{ensure, Result};
use infrastructure::{
    config_repository_impl::{ConfigRepositoryImpl, Version},
    google_auth_cert_repository_impl::GoogleAuthCertRepositoryImpl,
    posts_repository_impl::PostsRepositoryImpl,
    search_repository_impl::SearchRepositoryImpl,
};
use std::{env, path::PathBuf};
use super::Opts;

#[derive(Clone)] // FIXME: dieselのConnectionManagerがDebugを実装したらDebugにできる
pub struct Service {
    pub search_repository: SearchRepositoryImpl,
    pub posts_repository: PostsRepositoryImpl,
    pub config_repository: ConfigRepositoryImpl,
    pub cert_repository: GoogleAuthCertRepositoryImpl,
    pub admin_user_id: String,
    pub secret_key: String,
    pub static_path: PathBuf,
    pub mathjax_path: PathBuf,
}

impl Service {
    pub(crate) fn new(opts: &Opts) -> Result<Self> {
        let es_url = url::Url::parse(&env::var("ES_URL")?)?;
        let pg_url = url::Url::parse(&env::var("DATABASE_URL")?)?;
        let admin_user_id = env::var("ADMIN_USER_ID")?;
        let static_path = opts.static_path.clone();
        let mathjax_path = opts.mathjax_path.clone();
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
        let cert_repository = GoogleAuthCertRepositoryImpl::default();

        Ok(Service {
            search_repository,
            posts_repository,
            config_repository,
            cert_repository,
            admin_user_id,
            secret_key,
            static_path,
            mathjax_path,
        })
    }

    pub fn authorize(&self, id: &str) -> bool {
        id == self.admin_user_id
    }
}
