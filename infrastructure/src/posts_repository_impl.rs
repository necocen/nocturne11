use crate::diesel_helpers::{extract, DatePart, TimezoneCustomizer};
use crate::models::Post as PostModel;
use anyhow::Result;
use chrono::offset::Local;
use chrono::{DateTime, TimeZone};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use domain::{
    entities::{date::YearMonth, NewPost, Post, PostId},
    repositories::{import_posts::ImportPostsRepository, posts::PostsRepository},
};
use r2d2::Pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostsRepositoryImpl {
    pub(crate) conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl PostsRepositoryImpl {
    pub fn new_with_url(pg_url: &url::Url) -> Result<PostsRepositoryImpl> {
        let conn_manager = ConnectionManager::<PgConnection>::new(pg_url.as_str());
        let customizer = TimezoneCustomizer {
            offset: *Local::now().offset(),
        };
        let conn_pool = Arc::new(
            Pool::builder()
                .connection_customizer(Box::new(customizer))
                .build(conn_manager)?,
        );
        Ok(PostsRepositoryImpl { conn_pool })
    }
}

trait IntoVec<U> {
    fn into_vec(self) -> Vec<U>;
}

impl<T, U: From<T>> IntoVec<U> for Vec<T> {
    fn into_vec(self) -> Vec<U> {
        self.into_iter().map(|e| e.into()).collect()
    }
}

impl PostsRepository for PostsRepositoryImpl {
    fn get(&self, id: PostId) -> Result<Post> {
        use crate::schema::posts::dsl::posts;
        let post = posts
            .find(id)
            .get_result::<PostModel>(&self.conn_pool.get()?)?;
        Ok(post.into())
    }

    fn get_from_date<Tz: TimeZone>(
        &self,
        from: DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Post>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let results = posts
            .order_by(created_at.asc())
            .filter(created_at.ge(from))
            .offset(offset as i64)
            .limit(limit as i64)
            .get_results::<PostModel>(&self.conn_pool.get()?)?;
        Ok(results.into_vec())
    }

    fn get_until_date<Tz: TimeZone>(
        &self,
        until: DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Post>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let results = posts
            .order_by(created_at.desc())
            .filter(created_at.lt(until))
            .offset(offset as i64)
            .limit(limit as i64)
            .get_results::<PostModel>(&self.conn_pool.get()?)?;
        Ok(results.into_vec())
    }

    fn get_all(&self, offset: usize, limit: usize) -> Result<Vec<Post>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let results = posts
            .order_by(created_at.desc())
            .offset(offset as i64)
            .limit(limit as i64)
            .get_results::<PostModel>(&self.conn_pool.get()?)?;
        Ok(results.into_vec())
    }

    fn get_year_months(&self) -> Result<Vec<YearMonth>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let results = posts
            .select((
                extract(DatePart::Year, created_at),
                extract(DatePart::Month, created_at),
            ))
            .distinct()
            .get_results::<(i32, i32)>(&self.conn_pool.get()?)?;
        Ok(results
            .into_iter()
            .map(|(y, m)| YearMonth(y as u16, m as u8))
            .collect())
    }

    fn get_days(&self, YearMonth(year, month): YearMonth) -> Result<Vec<u8>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let created_after = Local.ymd(year.into(), month.into(), 1).and_hms(0, 0, 0);
        let created_before = Local
            .ymd(next_year.into(), next_month.into(), 1)
            .and_hms(0, 0, 0);

        let results = posts
            .filter(created_at.ge(created_after))
            .filter(created_at.lt(created_before))
            .select(extract(DatePart::Day, created_at))
            .distinct()
            .get_results::<i32>(&self.conn_pool.get()?)?;
        Ok(results.into_iter().map(|d| d as u8).collect())
    }

    fn create(&self, new_post: &NewPost) -> Result<Post> {
        use crate::schema::posts::{self, body, created_at, title, updated_at};
        let post = diesel::insert_into(posts::table)
            .values((
                title.eq(new_post.title.clone()),
                body.eq(new_post.body.clone()),
                created_at.eq(new_post.timestamp),
                updated_at.eq(new_post.timestamp),
            ))
            .get_result::<PostModel>(&self.conn_pool.get()?)?;
        Ok(post.into())
    }

    fn update(&self, id: PostId, new_post: &NewPost) -> Result<Post> {
        use crate::schema::posts::dsl::{body, posts, title, updated_at};
        let post = diesel::update(posts.find(id))
            .set((
                title.eq(new_post.title.clone()),
                body.eq(new_post.body.clone()),
                updated_at.eq(new_post.timestamp),
            ))
            .get_result::<PostModel>(&self.conn_pool.get()?)?;
        Ok(post.into())
    }

    fn delete(&self, id: PostId) -> Result<()> {
        use crate::schema::posts::dsl::posts;
        diesel::delete(posts.find(id)).execute(&self.conn_pool.get()?)?;
        Ok(())
    }
}

impl ImportPostsRepository for PostsRepositoryImpl {
    fn import(&self, posts: &[Post]) -> Result<Vec<Post>> {
        use crate::schema::posts::{self, body, created_at, id, title, updated_at};
        let records = posts
            .iter()
            .map(|post| {
                (
                    id.eq(post.id),
                    title.eq(post.title.clone()),
                    body.eq(post.body.clone()),
                    created_at.eq(post.created_at),
                    updated_at.eq(post.updated_at),
                )
            })
            .collect::<Vec<_>>();
        let post = diesel::insert_into(posts::table)
            .values(&records)
            .get_results::<PostModel>(&self.conn_pool.get()?)?;
        Ok(post.into_iter().map(Into::into).collect())
    }

    fn reset_id_sequence(&self) -> Result<()> {
        diesel::sql_query("SELECT reset_posts_id_sequence();").execute(&self.conn_pool.get()?)?;
        Ok(())
    }
}
