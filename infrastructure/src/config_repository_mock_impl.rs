use anyhow::Result;
use domain::{entities::config::Config, repositories::config::ConfigRepository};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct ConfigRepositoryMockImpl {
    config: RefCell<Config>,
}

impl ConfigRepositoryMockImpl {
    pub fn new() -> ConfigRepositoryMockImpl {
        ConfigRepositoryMockImpl {
            config: RefCell::new(Config {
                about: "単なる日記です\n\n\n単なる日記なんやで".to_string(),
                mathjax_options: "".to_string(),
                links: vec![],
            }),
        }
    }
}

impl Default for ConfigRepositoryMockImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigRepository for ConfigRepositoryMockImpl {
    fn get(&self) -> Result<Config> {
        Ok(self.config.borrow().clone())
    }

    fn set(&self, config: &Config) -> Result<()> {
        self.config.replace(config.clone());
        Ok(())
    }
}
