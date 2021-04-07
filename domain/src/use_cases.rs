use crate::entities::post::Post;
use crate::repositories::posts::PostsRepository;
use anyhow::Result;

pub fn get_post(_repository: &impl PostsRepository) -> Result<Post> {
    todo!();
}
