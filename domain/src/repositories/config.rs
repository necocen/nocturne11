use crate::entities::config::Config;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);

pub type Result<T> = std::result::Result<T, Error>;

pub trait ConfigRepository {
    fn get(&self) -> Result<Config>;
}
