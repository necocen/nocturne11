use crate::entities::Post;
use anyhow::Result;

pub trait ImportPostsRepository {
    fn insert(&self, post: &Post) -> Result<Post>;
}
