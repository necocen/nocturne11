use std::collections::HashMap;

use crate::diesel_helpers::TimezoneCustomizer;
use crate::models::Post as PostModel;
use anyhow::{Context, Result as AnyhowResult};
use application::adapters::PostsRepository;
use chrono::offset::Local;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use domain::entities::{NewPost, Post, PostId};
use r2d2::{Pool, PooledConnection};

#[derive(Clone)]
pub struct PostsRepositoryImpl {
    pub(crate) conn_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostsRepositoryImpl {
    pub fn new(pg_url: &url::Url) -> anyhow::Result<PostsRepositoryImpl> {
        let conn_manager = ConnectionManager::<PgConnection>::new(pg_url.as_str());
        let customizer = TimezoneCustomizer {
            offset: *Local::now().offset(),
        };
        let conn_pool = Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(conn_manager)
            .context("Failed to build connection pool")?;
        Ok(PostsRepositoryImpl { conn_pool })
    }

    fn get_conn(&self) -> AnyhowResult<PooledConnection<ConnectionManager<PgConnection>>> {
        self.conn_pool.get().context("Failed to get connection")
    }
}

pub trait PostsRepositoryImplTestHelper {
    fn import(&self, posts: &[Post]) -> anyhow::Result<Vec<Post>>;
    fn reset_id_sequence(&self) -> anyhow::Result<()>;
}

impl PostsRepositoryImplTestHelper for PostsRepositoryImpl {
    fn import(&self, posts: &[Post]) -> anyhow::Result<Vec<Post>> {
        use crate::schema::posts::{self, body, created_at, id, title, updated_at};
        let records = posts
            .iter()
            .map(|post| {
                (
                    id.eq(post.id.0),
                    title.eq(post.title.clone()),
                    body.eq(post.body.clone()),
                    created_at.eq(post.created_at),
                    updated_at.eq(post.updated_at),
                )
            })
            .collect::<Vec<_>>();
        let post = diesel::insert_into(posts::table)
            .values(&records)
            .get_results::<PostModel>(&mut self.get_conn()?)
            .context("Failed to get results")?;
        Ok(post.into_iter().map(Into::into).collect())
    }

    fn reset_id_sequence(&self) -> anyhow::Result<()> {
        diesel::sql_query("SELECT reset_posts_id_sequence();")
            .execute(&mut self.get_conn()?)
            .context("Failed to reset id sequence")?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl PostsRepository for PostsRepositoryImpl {
    async fn get_by_id(&self, id: &PostId) -> anyhow::Result<Option<Post>> {
        use crate::schema::posts::dsl::posts;
        let post = posts
            .find(id.0)
            .get_result::<PostModel>(&mut self.get_conn()?)
            .optional()
            .context("Failed to get result")?;
        Ok(post.map(Post::from))
    }

    async fn get_by_ids(&self, ids: &[PostId]) -> anyhow::Result<Vec<Post>> {
        use crate::schema::posts::{dsl::posts, id};
        let post_ids = ids.iter().map(|post_id| post_id.0).collect::<Vec<_>>();
        let posts_map: HashMap<_, _> = posts
            .filter(id.eq_any(&post_ids))
            .get_results::<PostModel>(&mut self.get_conn()?)
            .context("Failed to get results")?
            .into_iter()
            .map(|post| (post.id, Post::from(post)))
            .collect();

        Ok(post_ids
            .iter()
            .filter_map(|&post_id| posts_map.get(&post_id).cloned())
            .collect())
    }

    async fn add(&self, new_post: NewPost) -> anyhow::Result<Post> {
        use crate::schema::posts::{self, body, created_at, title, updated_at};
        let post = diesel::insert_into(posts::table)
            .values((
                title.eq(new_post.title),
                body.eq(new_post.body),
                created_at.eq(new_post.timestamp),
                updated_at.eq(new_post.timestamp),
            ))
            .get_result::<PostModel>(&mut self.get_conn()?)?;
        Ok(post.into())
    }

    async fn save(&self, post: &Post) -> anyhow::Result<Post> {
        use crate::schema::posts::dsl::{body, posts, title, updated_at};
        let post = diesel::update(posts.find(post.id.0))
            .set((
                title.eq(post.title.clone()),
                body.eq(post.body.clone()),
                updated_at.eq(post.updated_at),
            ))
            .get_result::<PostModel>(&mut self.get_conn()?)?;
        Ok(post.into())
    }

    async fn remove(&self, id: &PostId) -> anyhow::Result<()> {
        use crate::schema::posts::dsl::posts;
        diesel::delete(posts.find(id.0)).execute(&mut self.get_conn()?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
