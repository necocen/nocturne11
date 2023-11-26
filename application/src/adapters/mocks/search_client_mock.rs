use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Datelike as _, Local, NaiveDate, Utc};
use domain::entities::{date::YearMonth, Post, PostId};

use crate::{adapters::SearchClient, models::SearchResult};

pub struct SearchClientMock {
    pub posts: Arc<RwLock<Vec<Post>>>,
}

impl SearchClientMock {
    pub fn new(mut posts: Vec<Post>) -> SearchClientMock {
        posts.sort_by_key(|post| post.created_at);
        SearchClientMock {
            posts: Arc::new(RwLock::new(posts)),
        }
    }
}

#[async_trait::async_trait]
impl SearchClient for SearchClientMock {
    async fn get_last_updated(&self) -> anyhow::Result<Option<DateTime<Utc>>> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let last_updated = posts.iter().map(|post| post.updated_at).max();
        Ok(last_updated)
    }

    async fn save(&self, post: &Post) -> anyhow::Result<()> {
        let mut posts = self
            .posts
            .write()
            .map_err(|_| anyhow::anyhow!("RwLock write error"))?;
        if let Some(p) = posts.iter_mut().find(|p| p.id == post.id) {
            *p = post.clone();
        } else {
            posts.push(post.clone());
        }

        posts.sort_by_key(|post| post.created_at);

        Ok(())
    }

    async fn delete(&self, id: &PostId) -> anyhow::Result<()> {
        let mut posts = self
            .posts
            .write()
            .map_err(|_| anyhow::anyhow!("RwLock write error"))?;
        let index = posts
            .iter()
            .position(|p| p.id == *id)
            .ok_or_else(|| anyhow::anyhow!("Post not found"))?;
        posts.remove(index);
        Ok(())
    }

    async fn find_by_keywords<'a>(
        &self,
        keywords: &'a [&'a str],
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let posts = posts
            .iter()
            .filter(|post| keywords.iter().all(|keyword| post.body.contains(keyword)))
            .cloned()
            .collect::<Vec<_>>();
        let total_count = posts.len();
        let post_ids = posts
            .into_iter()
            .skip(offset)
            .take(limit)
            .map(|post| post.id)
            .collect();
        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn find_by_year_month(
        &self,
        year_month: &YearMonth,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let posts = posts
            .iter()
            .filter(|post| YearMonth::from(post.created_at) == *year_month)
            .cloned()
            .collect::<Vec<_>>();
        let total_count = posts.len();
        let post_ids = posts
            .into_iter()
            .skip(offset)
            .take(limit)
            .map(|post| post.id)
            .collect();
        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn find_by_date(
        &self,
        date: &NaiveDate,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let posts = posts
            .iter()
            .filter(|post| post.created_at.with_timezone(&Local).naive_local().date() == *date)
            .cloned()
            .collect::<Vec<_>>();
        let total_count = posts.len();
        let post_ids = posts
            .into_iter()
            .skip(offset)
            .take(limit)
            .map(|post| post.id)
            .collect();
        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn get_year_months(&self) -> anyhow::Result<Vec<YearMonth>> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let year_months = posts
            .iter()
            .map(|post| YearMonth::from(post.created_at))
            .collect::<Vec<_>>();
        let year_months = year_months
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        Ok(year_months)
    }

    async fn get_days_in_year_month(&self, ym: &YearMonth) -> anyhow::Result<Vec<u8>> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let days = posts
            .iter()
            .filter(|post| YearMonth::from(post.created_at) == *ym)
            .map(|post| post.created_at.with_timezone(&Local).naive_local().day() as u8)
            .collect::<Vec<_>>();
        let days = days
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        Ok(days)
    }

    async fn get_latest_posts(&self, offset: usize, limit: usize) -> anyhow::Result<SearchResult> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let total_count = posts.len();
        let post_ids = posts
            .iter()
            .rev()
            .skip(offset)
            .take(limit)
            .map(|post| post.id)
            .collect::<Vec<_>>();
        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn get_from_date(
        &self,
        from: DateTime<Utc>,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<PostId>> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let post_ids = posts
            .iter()
            .filter(|post| post.created_at >= from)
            .map(|post| post.id)
            .collect::<Vec<_>>();
        let post_ids = post_ids.into_iter().skip(offset).take(limit).collect();
        Ok(post_ids)
    }

    async fn get_until_date(
        &self,
        until: DateTime<Utc>,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<PostId>> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let post_ids = posts
            .iter()
            .filter(|post| post.created_at < until)
            .map(|post| post.id)
            .collect::<Vec<_>>();
        let post_ids = post_ids.into_iter().skip(offset).take(limit).collect();
        Ok(post_ids)
    }
}
