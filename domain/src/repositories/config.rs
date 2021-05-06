use crate::entities::config::Config;
use anyhow::Result;

pub trait ConfigRepository {
    fn get(&self) -> Result<Config>;

    fn set_about(&self, about: &str) -> Result<()>;
}
