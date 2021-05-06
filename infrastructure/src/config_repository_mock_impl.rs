use anyhow::Result;
use domain::{entities::config::Config, repositories::config::ConfigRepository};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ConfigRepositoryMockImpl {
    config: Arc<Mutex<Config>>,
}

impl ConfigRepositoryMockImpl {
    pub fn new() -> ConfigRepositoryMockImpl {
        ConfigRepositoryMockImpl {
            config: Arc::new(Mutex::new(Config {
                title: "andante".to_string(),
                about: "単なる日記です\n\n\n単なる日記なんやで".to_string(),
                mathjax_options: "".to_string(),
                links: vec![],
            })),
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
        Ok(self.config.lock().unwrap().clone())
    }

    fn set_about(&self, about: &str) -> Result<()> {
        let mut config = self.config.lock().unwrap();
        config.about = about.to_owned();
        Ok(())
    }
}
