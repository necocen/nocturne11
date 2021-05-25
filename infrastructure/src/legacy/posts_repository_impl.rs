use crate::legacy::models::Article as OldArticle;
use anyhow::Context;
use chrono::{Local, TimeZone, Utc};
use diesel::prelude::*;
use domain::{
    entities::Post,
    repositories::export_posts::{ExportPostsRepository, Result},
};

pub struct OldPostsRepositoryImpl {
    connection: MysqlConnection,
}

impl OldPostsRepositoryImpl {
    pub fn new(database_url: &url::Url) -> Result<OldPostsRepositoryImpl> {
        let connection = MysqlConnection::establish(database_url.as_str())
            .with_context(|| format!("Failed to connect export DB: {}", database_url))?;
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
            .get_results::<OldArticle>(&self.connection)
            .context("Failed to get old articles")?
            .into_iter()
            .map(|article| {
                Ok(Post::new(
                    article.id,
                    article.title,
                    article.text,
                    Local
                        .from_local_datetime(&article.created_at)
                        .single()
                        .context("Failed to fetch created_at")?
                        .with_timezone(&Utc),
                    Local
                        .from_local_datetime(&article.updated_at)
                        .single()
                        .context("Failed to fetch updated_at")?
                        .with_timezone(&Utc),
                ))
            })
            .collect()
    }
}
