use crate::entities::{Post, PostId};
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait::async_trait]
pub trait SearchRepository {
    async fn insert(&self, post: &Post) -> Result<()>;

    async fn insert_bulk(&self, posts: &[Post]) -> Result<()>;

    async fn update(&self, post: &Post) -> Result<()>;

    async fn delete(&self, id: PostId) -> Result<()>;

    async fn save_snapshot(&self) -> Result<()>;

    async fn reset(&self) -> Result<()>;

    async fn newest_snapshot_name(&self) -> Result<String>;

    async fn restore_snapshot(&self, snapshot_name: &str) -> Result<()>;
}
