use anyhow::Context;
use config::{File, FileFormat};
use domain::{
    entities::config::Config, repositories::config::ConfigRepository, repositories::config::Result,
};
use std::env;

#[derive(Debug, Clone)]
pub struct ConfigRepositoryImpl {
    config: Config,
}

impl ConfigRepositoryImpl {
    pub fn new(config_toml: &str, version: &Version<'_>) -> Result<ConfigRepositoryImpl> {
        // FIXME: こいつはそもそもエラーハンドリングではなくpanicすべきでは？
        let mut config = config::Config::default();
        config
            .merge(File::from_str(config_toml, FileFormat::Toml))
            .context("Failed to read config.toml")?;
        config
            .set(
                "site.generator",
                format!("Nocturne v{} {}", version.version, version.timestamp),
            )
            .context("Failed to set site.generator")?;
        config
            .set(
                "auth.google_client_id",
                env::var("GOOGLE_OAUTH_CLIENT_ID")
                    .context("Failed to get GOOGLE_OAUTH_CLIENT_ID")?,
            )
            .context("Failed to set auth.google_client_id")?;
        config
            .set(
                "auth.admin_user_id",
                env::var("ADMIN_USER_ID").context("Failed to get ADMIN_USER_ID env")?,
            )
            .context("Failed to set auth.admin_user_id")?;
        let config = config
            .try_into::<Config>()
            .context("Failed to build Config")?;
        Ok(ConfigRepositoryImpl { config })
    }
}

impl ConfigRepository for ConfigRepositoryImpl {
    fn get(&self) -> Result<Config> {
        Ok(self.config.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Version<'a> {
    pub version: &'a str,
    pub timestamp: &'a str,
}
