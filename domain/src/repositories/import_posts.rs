use crate::entities::Post;
use anyhow::Result;

pub trait ImportPostsRepository {
    fn import(&self, post: &Post) -> Result<Post>;
}
