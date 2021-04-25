use crate::legacy::models::Article as OldArticle;
use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use domain::entities::{date::YearMonth, Post, PostId};
use domain::repositories::posts::PostsRepository;

pub struct OldPostsRepositoryImpl {
    connection: MysqlConnection,
}

impl OldPostsRepositoryImpl {
    pub fn new(database_url: &url::Url) -> Result<OldPostsRepositoryImpl> {
        let connection = MysqlConnection::establish(database_url.as_str())?;
        Ok(OldPostsRepositoryImpl { connection })
    }
}

impl PostsRepository for OldPostsRepositoryImpl {
    fn get(&self, id: PostId) -> Result<Post> {
        use crate::legacy::schema::articles::dsl::{articles, id as article_id};
        let article = articles
            .filter(article_id.eq(id))
            .first::<OldArticle>(&self.connection)?;
        Ok(Post {
            id: article.id,
            title: article.title,
            body: article.text.replace("\r\n", "\n").replace("\r", "\n"),
            created_at: Utc
                .from_local_datetime(&article.created_at)
                .single()
                .context("Failed to fetch created_at")?,
            updated_at: Utc
                .from_local_datetime(&article.updated_at)
                .single()
                .context("Failed to fetch updated_at")?,
        })
    }

    fn get_all(&self, offset: usize, limit: usize) -> Result<Vec<Post>> {
        use crate::legacy::schema::articles::dsl::{articles, created_at};
        articles
            .order_by(created_at.desc())
            .offset(offset as i64)
            .limit(limit as i64)
            .get_results::<OldArticle>(&self.connection)?
            .into_iter()
            .map(|article| {
                Ok(Post {
                    id: article.id,
                    title: article.title,
                    body: article.text.replace("\r\n", "\n").replace("\r", "\n"),
                    created_at: Utc
                        .from_local_datetime(&article.created_at)
                        .single()
                        .context("Failed to fetch created_at")?,
                    updated_at: Utc
                        .from_local_datetime(&article.updated_at)
                        .single()
                        .context("Failed to fetch updated_at")?,
                })
            })
            .collect()
    }

    fn get_from_date<Tz: TimeZone>(
        &self,
        _from: DateTime<Tz>,
        _offset: usize,
        _limit: usize,
    ) -> Result<Vec<Post>> {
        unimplemented!("This impl is Legacy");
    }

    fn get_until_date<Tz: TimeZone>(
        &self,
        _until: DateTime<Tz>,
        _offset: usize,
        _limit: usize,
    ) -> Result<Vec<Post>> {
        unimplemented!("This impl is Legacy");
    }

    fn get_year_months(&self) -> Result<Vec<YearMonth>> {
        unimplemented!("This impl is Legacy");
    }

    fn get_days(&self, _ym: YearMonth) -> Result<Vec<u8>> {
        unimplemented!("This impl is Legacy");
    }

    fn insert(&self, _post: &Post) -> Result<Post> {
        unimplemented!("This impl is Legacy");
    }
}
