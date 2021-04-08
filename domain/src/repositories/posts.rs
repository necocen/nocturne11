use crate::entities::Post;
use anyhow::Result;

pub trait PostsRepository {
    fn get(&self, id: i32) -> Result<Post>;

    fn get_all(&self) -> Result<Vec<Post>>;

    fn insert(&self, post: &Post) -> Result<Post>;
}
