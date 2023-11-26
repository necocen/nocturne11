use anyhow::Context as _;
use application::adapters::AppConfigProvider;
use config::{builder::DefaultState, ConfigBuilder, File, FileFormat};
use domain::{
    entities::config::{AuthenticationSettings, Config},
    repositories::config::ConfigRepository,
    repositories::config::Result,
};
use std::env;

#[derive(Debug, Clone)]
pub struct ConfigRepositoryImpl {
    config: Config,
}

impl ConfigRepositoryImpl {
    pub fn new(config_toml: &str, version: &Version<'_>) -> Result<ConfigRepositoryImpl> {
        // FIXME: こいつはそもそもエラーハンドリングではなくpanicすべきでは？
        let config = ConfigBuilder::<DefaultState>::default()
            .add_source(File::from_str(config_toml, FileFormat::Toml))
            .set_override(
                "site.generator",
                format!("Nocturne v{} {}", version.version, version.timestamp),
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
        Ok(ConfigRepositoryImpl { config })
    }
}

impl ConfigRepository for ConfigRepositoryImpl {
    fn get(&self) -> Result<Config> {
        Ok(self.config.clone())
    }
}

impl AppConfigProvider for ConfigRepositoryImpl {
    fn get_all(&self) -> anyhow::Result<Config> {
        Ok(self.config.clone())
    }

    fn get_auth_settings(&self) -> anyhow::Result<AuthenticationSettings> {
        Ok(self.config.auth.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Version<'a> {
    pub version: &'a str,
    pub timestamp: &'a str,
}
