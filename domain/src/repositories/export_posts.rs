use crate::entities::Post;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);

pub type Result<T> = std::result::Result<T, Error>;

pub trait ExportPostsRepository {
    /// すべてのPostを`created_at`降順で最大`limit`件返します
    fn get_all(&self, offset: usize, limit: usize) -> Result<Vec<Post>>;
}
