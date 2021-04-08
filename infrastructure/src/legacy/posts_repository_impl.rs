use crate::legacy::models::Article as OldArticle;
use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use diesel::prelude::*;
use domain::entities::Post;
use domain::repositories::posts::PostsRepository;

pub struct OldPostsRepositoryImpl {
    connection: MysqlConnection,
}

impl OldPostsRepositoryImpl {
    pub fn new(database_url: String) -> OldPostsRepositoryImpl {
        let connection = MysqlConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        OldPostsRepositoryImpl { connection }
    }
}

impl PostsRepository for OldPostsRepositoryImpl {
    fn get(&self, id: i32) -> Result<Post> {
        use crate::legacy::schema::articles::dsl::{articles, id as article_id};
        let article = articles
            .filter(article_id.eq(id))
            .first::<OldArticle>(&self.connection)?;
        Ok(Post {
            id: article.id,
            title: article.title,
            body: article.text,
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

    fn get_all(&self) -> Result<Vec<Post>> {
        use crate::legacy::schema::articles::dsl::{articles, created_at};
        articles
            .order_by(created_at)
            .get_results::<OldArticle>(&self.connection)?
            .into_iter()
            .map(|article| {
                Ok(Post {
                    id: article.id,
                    title: article.title,
                    body: article.text,
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

    fn insert(&self, _post: &Post) -> Result<Post> {
        panic!("Do not call this");
    }
}
