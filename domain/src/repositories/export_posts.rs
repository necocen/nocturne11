use crate::entities::Post;
use anyhow::Result;

pub trait ExportPostsRepository {
    /// すべてのPostを`created_at`降順で最大`limit`件返します
    fn get_all(&self, offset: usize, limit: usize) -> Result<Vec<Post>>;
}
