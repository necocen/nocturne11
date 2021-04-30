use crate::entities::{Post, SearchResult};
use anyhow::Result;

#[async_trait::async_trait]
pub trait SearchRepository {
    async fn search(
        &self,
        keywords: &[String],
        search_after: Option<(u64, u64)>,
    ) -> Result<SearchResult>;

    async fn insert(&self, post: &Post) -> Result<()>;

    async fn insert_bulk(&self, posts: &[Post]) -> Result<()>;

    async fn save_snapshot(&self) -> Result<()>;

    async fn reset(&self) -> Result<()>;

    async fn newest_snapshot_name(&self) -> Result<String>;

    async fn restore_snapshot(&self, snapshot_name: &str) -> Result<()>;
}
