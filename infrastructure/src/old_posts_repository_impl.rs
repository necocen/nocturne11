use domain::repositories::posts::PostsRepository;
use diesel::prelude::*;
use crate::old_models::Article as OldArticle;
use domain::entities::Post;
use anyhow::Result;
use chrono::{Utc, TimeZone};

pub struct OldPostsRepositoryImpl {
    connection: MysqlConnection,
}

impl OldPostsRepositoryImpl {
    pub fn new(database_url: String) -> OldPostsRepositoryImpl {
        let connection = MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        OldPostsRepositoryImpl {
            connection
        }
    }
}

impl PostsRepository for OldPostsRepositoryImpl {
    fn get(&self, id: i32) -> Result<Post> {
        use crate::old_schema::articles::dsl::{articles, id as article_id};
        let article = articles.filter(article_id.eq(id)).first::<OldArticle>(&self.connection)?;
        Ok(Post {
            id: article.id,
            title: article.title,
            body: article.text,
            created_at: Utc.from_local_datetime(&article.created_at).unwrap(),
            updated_at: Utc.from_local_datetime(&article.updated_at).unwrap(),
        })
    }

    fn insert(&self, _post: Post) -> Result<()> {
        panic!("Do not call this");
    }
}
