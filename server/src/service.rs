use super::Opts;
use anyhow::{ensure, Context as _, Result};
use config::{builder::DefaultState, ConfigBuilder, File, FileFormat};
use domain::entities::config::Config;
use infrastructure::{
    google_auth_cert_repository_impl::GoogleAuthCertRepositoryImpl,
    posts_repository_impl::PostsRepositoryImpl, search_client::SearchClient,
    search_repository_impl::SearchRepositoryImpl,
};
use std::{env, path::PathBuf};

#[derive(Clone)] // FIXME: dieselのConnectionManagerがDebugを実装したらDebugにできる
pub struct Service {
    pub search_repository: SearchRepositoryImpl,
    pub posts_repository: PostsRepositoryImpl,
    pub cert_repository: GoogleAuthCertRepositoryImpl,
    pub search_client: SearchClient,
    pub admin_user_id: String,
    pub secret_key: String,
    pub static_path: PathBuf,
    pub is_development: bool,
    pub config: Config,
}

impl Service {
    pub(crate) fn new(opts: &Opts) -> Result<Self> {
        let es_url = url::Url::parse(&env::var("ES_URL")?)?;
        let pg_url = url::Url::parse(&env::var("DATABASE_URL")?)?;
        let admin_user_id = env::var("ADMIN_USER_ID")?;
        let static_path = opts.static_path.clone();
        let secret_key = env::var("SECRET_KEY")?;
        ensure!(secret_key.len() >= 32, "SECRET_KEY is not long enough.");

        let config_toml = include_str!("../../config.toml");
        let config = Self::get_config(config_toml)?;

        let search_repository = SearchRepositoryImpl::new(&es_url)?;
        let posts_repository = PostsRepositoryImpl::new(&pg_url)?;
        let cert_repository = GoogleAuthCertRepositoryImpl::default();
        let search_client = SearchClient::new(&es_url, &pg_url)?;

        Ok(Service {
            search_repository,
            posts_repository,
            cert_repository,
            search_client,
            admin_user_id,
            secret_key,
            static_path,
            is_development: opts.is_development,
            config,
        })
    }

    pub fn authorize(&self, id: &str) -> bool {
        id == self.admin_user_id
    }

    fn get_config(config_toml: &str) -> Result<Config> {
        let version = env!("VERGEN_BUILD_SEMVER");
        let timestamp = env!("VERGEN_BUILD_TIMESTAMP");
        let config = ConfigBuilder::<DefaultState>::default()
            .add_source(File::from_str(config_toml, FileFormat::Toml))
            .set_override(
                "site.generator",
                format!("Nocturne v{} {}", version, timestamp),
            )
            .context("Failed to set site.generator")?
            .set_override(
                "auth.google_client_id",
                env::var("GOOGLE_OAUTH_CLIENT_ID")
                    .context("Failed to get GOOGLE_OAUTH_CLIENT_ID")?,
            )
            .context("Failed to set auth.google_client_id")?
            .set_override(
                "auth.admin_user_id",
                env::var("ADMIN_USER_ID").context("Failed to get ADMIN_USER_ID env")?,
            )
            .context("Failed to set auth.admin_user_id")?
            .set_override(
                "hatena_star_token",
                env::var("HATENA_STAR_TOKEN").context("Failed to get HATENA_STAR_TOKEN")?,
            )
            .context("Failed to set hatena_star_token")?
            .set_override(
                "ga_code",
                env::var("GA_TRACKING_CODE").context("Failed to get GA_TRACKING_CODE")?,
            )
            .context("Failed to set ga_code")?
            .build()
            .context("Failed to read config.toml")?;
        let config = config.try_deserialize().context("Failed to build Config")?;
        Ok(config)
    }
}
