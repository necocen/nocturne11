use crate::{entities::config::Config, repositories::config::ConfigRepository, Result};

pub fn get_config(config_repository: &impl ConfigRepository) -> Result<Config> {
    Ok(config_repository.get()?)
}
