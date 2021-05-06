use crate::models::Config as ConfigModel;
use anyhow::Result;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, RunQueryDsl};
use domain::{entities::config::Config, repositories::config::ConfigRepository};
use r2d2::Pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConfigRepositoryImpl {
    pub(crate) conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl ConfigRepository for ConfigRepositoryImpl {
    fn get(&self) -> Result<Config> {
        use crate::schema::configs::dsl::configs;
        let records = configs.get_results::<ConfigModel>(&self.conn_pool.get()?)?;
        let about = records
            .iter()
            .find(|c| c.key == "about")
            .map_or(String::new(), |c| c.value.to_owned());
        let mathjax_options = records
            .iter()
            .find(|c| c.key == "mathjax_options")
            .map_or(String::new(), |c| c.value.to_owned());
        Ok(Config {
            title: "andante".to_owned(),
            description: "個人的な日記".to_owned(),
            author: "κねこせん".to_owned(),
            email: "necocen@gmail.com".to_owned(),
            generator: "Nocturne v11".to_owned(),
            about,
            mathjax_options,
            links: vec![],
        })
    }

    fn set_about(&self, about: &str) -> Result<()> {
        use crate::schema::configs::{self, key, value};
        diesel::insert_into(configs::table)
            .values((key.eq("about"), value.eq(about)))
            .on_conflict(key)
            .do_update()
            .set(value.eq(about))
            .execute(&self.conn_pool.get()?)?;
        Ok(())
    }
}
