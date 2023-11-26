use domain::entities::Post;

use crate::{
    adapters::{PostsRepository, SearchClient},
    ApplicationResult,
};

pub struct UpdatePostUseCase;

impl UpdatePostUseCase {
    pub async fn execute(
        posts: &impl PostsRepository,
        search_client: &impl SearchClient,
        post: &Post,
    ) -> ApplicationResult<()> {
        _ = posts.save(post).await?;
        if let Err(e) = search_client.save(post).await {
            log::warn!("failed to update search index: {e}");
        }
        Ok(())
    }
}
