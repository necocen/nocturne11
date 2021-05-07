use crate::config_repository_partial_impl::ConfigRepositoryPartialImpl;
use crate::diesel_helpers::TimezoneCustomizer;
use crate::posts_repository_impl::PostsRepositoryImpl;
use anyhow::Result;
use chrono::Local;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;
use std::sync::Arc;

pub fn pg_repositories(
    pg_url: &url::Url,
) -> Result<(PostsRepositoryImpl, ConfigRepositoryPartialImpl)> {
    let conn_manager = ConnectionManager::<PgConnection>::new(pg_url.as_str());
    let customizer = TimezoneCustomizer {
        offset: *Local::now().offset(),
    };
    let conn_pool = Arc::new(
        Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(conn_manager)?,
    );
    Ok((
        PostsRepositoryImpl {
            conn_pool: conn_pool.clone(),
        },
        ConfigRepositoryPartialImpl { conn_pool },
    ))
}
