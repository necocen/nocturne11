use domain::entities::PostId;

use crate::{
    adapters::{PostsRepository, SearchClient},
    errors::ApplicationError,
    models::{AdjacentPageInfo, Page},
    ApplicationResult,
};

pub struct GetPostByIdUseCase;

impl GetPostByIdUseCase {
    pub async fn execute<'a>(
        posts: &impl PostsRepository,
        search_client: &impl SearchClient,
        id: &'a PostId,
    ) -> ApplicationResult<Page<'a, PostId, ()>> {
        let Some(post) = posts.get_by_id(id).await? else {
            return Err(ApplicationError::PostNotFound);
        };

        let next_post_id = search_client
            .get_from_date(post.created_at, 1, 1)
            .await?
            .first()
            .cloned();
        let prev_post_id = search_client
            .get_until_date(post.created_at, 0, 1)
            .await?
            .first()
            .cloned();

        Ok(Page {
            condition: id,
            index: (),
            posts: vec![post],
            next_page: next_post_id.map(AdjacentPageInfo::Condition),
            prev_page: prev_post_id.map(AdjacentPageInfo::Condition),
        })
    }
}
