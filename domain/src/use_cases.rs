use crate::entities::post::Post;
use crate::repositories::posts::PostsRepository;
use anyhow::Result;

pub fn get_posts(repository: &impl PostsRepository) -> Result<Vec<Post>> {
    Ok(repository.get_all()?[..10].to_vec())
}
