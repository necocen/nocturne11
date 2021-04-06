use anyhow::Result;
use crate::entities::post::Post;
use crate::repositories::posts::PostsRepository;

pub fn get_post(repository: &impl PostsRepository) -> Result<Post> {
    todo!();
}
