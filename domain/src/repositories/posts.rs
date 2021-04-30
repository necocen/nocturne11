use crate::entities::{date::YearMonth, NewPost, Post, PostId};
use anyhow::Result;
use chrono::{DateTime, TimeZone};

pub trait PostsRepository {
    fn get(&self, id: PostId) -> Result<Post>;

    /// `from`以降（`from`を**含む**）のPostを`created_at`昇順で最大`limit`件返します
    fn get_from_date<Tz: TimeZone>(
        &self,
        from: DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Post>>;

    /// `until`以前（`until`を**含まない**）のPostを`created_at`降順で最大`limit`件返します
    fn get_until_date<Tz: TimeZone>(
        &self,
        until: DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Post>>;

    /// すべてのPostを`created_at`降順で最大`limit`件返します
    fn get_all(&self, offset: usize, limit: usize) -> Result<Vec<Post>>;

    fn get_year_months(&self) -> Result<Vec<YearMonth>>;

    fn get_days(&self, ym: YearMonth) -> Result<Vec<u8>>;

    fn create(&self, new_post: &NewPost) -> Result<Post>;
}
