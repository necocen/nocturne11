use crate::diesel_helpers::{date_part, TimezoneCustomizer};
use crate::models::Post as PostModel;
use anyhow::Result;
use chrono::offset::Local;
use chrono::{DateTime, TimeZone};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use domain::entities::{date::YearMonth, Post, PostId};
use domain::repositories::posts::PostsRepository;
use r2d2::Pool;

#[derive(Clone)]
pub struct PostsRepositoryImpl {
    conn_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostsRepositoryImpl {
    pub fn new(pq_url: url::Url) -> Result<PostsRepositoryImpl> {
        let conn_manager = ConnectionManager::<PgConnection>::new(pq_url.as_str());
        let customizer = TimezoneCustomizer {
            offset: *Local::now().offset(),
        };
        let conn_pool = Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(conn_manager)?;
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
            .order_by(created_at.asc())
            .filter(created_at.lt(until))
            .offset(offset as i64)
            .limit(limit as i64)
            .get_results::<PostModel>(&self.conn_pool.get()?)?;
        Ok(results.into_vec())
    }

    fn get_all(&self) -> Result<Vec<Post>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let results = posts
            .order_by(created_at.desc())
            .get_results::<PostModel>(&self.conn_pool.get()?)?;
        Ok(results.into_vec())
    }

    fn get_year_months(&self) -> Result<Vec<YearMonth>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let results = posts
            .select((
                date_part("YEAR", created_at),
                date_part("MONTH", created_at),
            ))
            .distinct()
            .get_results::<(f64, f64)>(&self.conn_pool.get()?)?;
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
            .select(date_part("DAY", created_at))
            .distinct()
            .get_results::<f64>(&self.conn_pool.get()?)?;
        Ok(results.into_iter().map(|d| d as u8).collect())
    }

    fn insert(&self, post: &Post) -> Result<Post> {
        use crate::schema::posts;
        let post = diesel::insert_into(posts::table)
            .values(&PostModel {
                id: post.id,
                title: post.title.clone(),
                body: post.body.clone(),
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
            .get_result::<PostModel>(&self.conn_pool.get()?)?;
        Ok(post.into())
    }
}
