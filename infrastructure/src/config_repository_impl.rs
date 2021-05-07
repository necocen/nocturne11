use anyhow::Result;
use config::{File, FileFormat};
use domain::{entities::config::Config, repositories::config::ConfigRepository};

#[derive(Debug, Clone)]
pub struct ConfigRepositoryImpl {
    config: Config,
}

impl ConfigRepositoryImpl {
    pub fn new(config_toml: &str, version: &Version<'_>) -> Result<ConfigRepositoryImpl> {
        let mut config = config::Config::default();
        config.merge(File::from_str(config_toml, FileFormat::Toml))?;
        config.set(
            "site.generator",
            format!("Nocturne v{} {}", version.version, version.timestamp),
        )?;
        let config = config.try_into::<Config>()?;
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
