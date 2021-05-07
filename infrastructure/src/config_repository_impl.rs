use anyhow::Result;
use domain::{
    entities::config::{Author, Config, MathJax, Site},
    repositories::config::ConfigRepository,
};

#[derive(Debug, Clone)]
pub struct ConfigRepositoryImpl {}

impl ConfigRepositoryImpl {
    pub fn new() -> ConfigRepositoryImpl {
        ConfigRepositoryImpl {}
    }
}

impl Default for ConfigRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigRepository for ConfigRepositoryImpl {
    fn get(&self) -> Result<Config> {
        Ok(Config {
            site: Site {
                title: "andante".to_owned(),
                description: "個人的な日記".to_owned(),
                about: "単なる日記です\n\n\n単なる日記なんやで".to_string(),
                links: vec![],
                generator: "Nocturne v11".to_owned(),
            },
            author: Author {
                name: "κねこせん".to_owned(),
                email: "necocen@gmail.com".to_owned(),
            },
            mathjax: MathJax {
                options: "".to_string(),
            },
        })
    }
}
