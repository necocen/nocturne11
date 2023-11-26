use chrono::{DateTime, NaiveDate, Utc};
use domain::entities::{date::YearMonth, Post, PostId};
#[cfg(test)]
use mockall::{automock, mock, predicate::*};

use crate::models::SearchResult;

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait SearchClient {
    async fn find_by_keywords<'a>(
        &self,
        keywords: &'a [&'a str],
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult>;
    async fn find_by_year_month(
        &self,
        year_month: &YearMonth,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult>;
    async fn find_by_date(
        &self,
        date: &NaiveDate,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult>;

    async fn get_year_months(&self) -> anyhow::Result<Vec<YearMonth>>;
    async fn get_days_in_year_month(&self, ym: &YearMonth) -> anyhow::Result<Vec<u8>>;
    async fn get_latest_posts(&self, offset: usize, limit: usize) -> anyhow::Result<SearchResult>;
    async fn get_last_updated(&self) -> anyhow::Result<Option<DateTime<Utc>>>;

    /// `from`以降（`from`を**含む**）のPostのIDを`created_at`昇順で最大`limit`件返します
    async fn get_from_date(
        &self,
        from: DateTime<Utc>,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<PostId>>;
    /// `until`以前（`until`を**含まない**）のPostのIDを`created_at`降順で最大`limit`件返します
    async fn get_until_date(
        &self,
        until: DateTime<Utc>,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<PostId>>;

    async fn save(&self, post: &Post) -> anyhow::Result<()>;
    async fn delete(&self, id: &PostId) -> anyhow::Result<()>;
}
