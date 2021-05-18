use crate::entities::Post;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);

pub type Result<T> = std::result::Result<T, Error>;

pub trait ImportPostsRepository {
    fn import(&self, posts: &[Post]) -> Result<Vec<Post>>;

    fn reset_id_sequence(&self) -> Result<()>;
}
