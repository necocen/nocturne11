use domain::entities::{NewPost, Post};

use crate::{
    adapters::{PostsRepository, SearchClient},
    ApplicationResult,
};

pub struct CreateNewPostUseCase;

impl CreateNewPostUseCase {
    pub async fn execute(
        posts: &impl PostsRepository,
        search_client: &impl SearchClient,
        new_post: NewPost,
    ) -> ApplicationResult<Post> {
        let post = posts.add(new_post).await?;
        if let Err(e) = search_client.save(&post).await {
            log::warn!("failed to create search index: {e}");
        }
        Ok(post)
    }
}
