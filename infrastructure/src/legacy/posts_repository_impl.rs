use crate::legacy::models::Article as OldArticle;
use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use diesel::prelude::*;
use domain::{entities::Post, repositories::export_posts::ExportPostsRepository};

pub struct OldPostsRepositoryImpl {
    connection: MysqlConnection,
}

impl OldPostsRepositoryImpl {
    pub fn new(database_url: &url::Url) -> Result<OldPostsRepositoryImpl> {
        let connection = MysqlConnection::establish(database_url.as_str())?;
        Ok(OldPostsRepositoryImpl { connection })
    }
}

impl ExportPostsRepository for OldPostsRepositoryImpl {
    fn get_all(&self, offset: usize, limit: usize) -> Result<Vec<Post>> {
        use crate::legacy::schema::articles::dsl::{articles, created_at};
        articles
            .order_by(created_at.desc())
            .offset(offset as i64)
            .limit(limit as i64)
            .get_results::<OldArticle>(&self.connection)?
            .into_iter()
            .map(|article| {
                Ok(Post::new(
                    article.id,
                    article.title,
                    article.text,
                    Utc.from_local_datetime(&article.created_at)
                        .single()
                        .context("Failed to fetch created_at")?,
                    Utc.from_local_datetime(&article.updated_at)
                        .single()
                        .context("Failed to fetch updated_at")?,
                ))
            })
            .collect()
    }
}
