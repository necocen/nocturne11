use domain::entities::PostId;

use crate::{
    adapters::{PostsRepository, SearchClient},
    ApplicationResult,
};

pub struct DeletePostUseCase;

impl DeletePostUseCase {
    pub async fn execute(
        posts: &impl PostsRepository,
        search_client: &impl SearchClient,
        id: &PostId,
    ) -> ApplicationResult<()> {
        posts.remove(id).await?;
        if let Err(e) = search_client.delete(id).await {
            log::warn!("failed to delete search index: {e}");
        }
        Ok(())
    }
}
