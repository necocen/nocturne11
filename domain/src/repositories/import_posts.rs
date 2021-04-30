use crate::entities::Post;
use anyhow::Result;

pub trait ImportPostsRepository {
    fn import(&self, posts: Vec<Post>) -> Result<Vec<Post>>;

    fn reset_id_sequence(&self) -> Result<()>;
}
