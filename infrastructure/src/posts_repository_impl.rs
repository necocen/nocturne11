use crate::models::Post as PostModel;
use anyhow::Result;
use chrono::offset::Local;
use chrono::TimeZone;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::sql_types::*;
use domain::entities::{date::YearMonth, Post};
use domain::repositories::posts::PostsRepository;
use r2d2::{CustomizeConnection, Pool};

#[derive(Clone)]
pub struct PostsRepositoryImpl {
    conn_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostsRepositoryImpl {
    pub fn new(database_url: String) -> PostsRepositoryImpl {
        let conn_manager = ConnectionManager::<PgConnection>::new(&database_url);
        let customizer = TimezoneCustomizer {
            timezone: "Asia/Tokyo".to_string(),
        };
        let conn_pool: Pool<ConnectionManager<PgConnection>> = Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(conn_manager)
            .expect("Failed to create pool");
        PostsRepositoryImpl { conn_pool }
    }
}

#[derive(Debug, Clone)]
struct TimezoneCustomizer {
    timezone: String,
}

impl CustomizeConnection<PgConnection, diesel::r2d2::Error> for TimezoneCustomizer {
    fn on_acquire(&self, conn: &mut PgConnection) -> std::result::Result<(), diesel::r2d2::Error> {
        conn.execute(&format!("SET TIME ZONE '{}';", self.timezone))
            .map_err(|e| diesel::r2d2::Error::QueryError(e))?;
        Ok(())
    }
}

impl PostsRepository for PostsRepositoryImpl {
    fn get(&self, id: i32) -> Result<Post> {
        use crate::schema::posts::dsl::{id as post_id, posts};
        let post = posts
            .filter(post_id.eq(id))
            .first::<PostModel>(&self.conn_pool.get()?)?;
        Ok(Post {
            id: post.id,
            title: post.title,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }

    fn get_all(&self) -> Result<Vec<Post>> {
        use crate::schema::posts::dsl::{created_at, posts};
        posts
            .order_by(created_at)
            .get_results::<PostModel>(&self.conn_pool.get()?)?
            .into_iter()
            .map(|post| {
                Ok(Post {
                    id: post.id,
                    title: post.title,
                    body: post.body,
                    created_at: post.created_at,
                    updated_at: post.updated_at,
                })
            })
            .collect()
    }

    fn get_year_months(&self) -> Result<Vec<YearMonth>> {
        use crate::schema::posts::dsl::{created_at, posts};
        Ok(posts
            .select((
                date_part("YEAR", created_at),
                date_part("MONTH", created_at),
            ))
            .distinct()
            .get_results::<(f64, f64)>(&self.conn_pool.get()?)?
            .into_iter()
            .map(|(y, m)| YearMonth(y as u16, m as u8))
            .collect::<Vec<_>>())
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
        Ok(posts
            .filter(created_at.ge(created_after))
            .filter(created_at.lt(created_before))
            .select(date_part("DAY", created_at))
            .distinct()
            .get_results::<f64>(&self.conn_pool.get()?)?
            .into_iter()
            .map(|d| d as u8)
            .collect::<Vec<_>>())
    }

    fn insert(&self, post: &Post) -> Result<Post> {
        use crate::schema::posts;
        let post: PostModel = diesel::insert_into(posts::table)
            .values(&PostModel {
                id: post.id,
                title: post.title.clone(),
                body: post.body.clone(),
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
            .get_result(&self.conn_pool.get()?)?;
        Ok(Post {
            id: post.id,
            title: post.title,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }
}

sql_function! {
    #[sql_name = "DATE_PART"]
    fn date_part(part: Text, ts: Timestamptz) -> Double;
}
