use crate::entities::config::Config;
use anyhow::Result;

pub trait ConfigRepository {
    fn get(&self) -> Result<Config>;

    fn set(&self, config: &Config) -> Result<()>;
}
