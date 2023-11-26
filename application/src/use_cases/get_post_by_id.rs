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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::mocks::{PostsRepositoryMock, SearchClientMock};
    use chrono::Utc;
    use domain::entities::Post;

    #[tokio::test]
    async fn test_get_post_by_id() {
        let post = Post::new(
            PostId(629),
            "test title",
            "test body",
            Utc::now(),
            Utc::now(),
        );
        let posts = PostsRepositoryMock::new(vec![post.clone()]);
        let search_client = SearchClientMock::new(vec![post.clone()]);
        let post_id = PostId(629);
        let page = GetPostByIdUseCase::execute(&posts, &search_client, &post_id)
            .await
            .unwrap();
        assert_eq!(page.condition, &post_id);
        assert_eq!(page.posts.len(), 1);
        assert!(page.next_page.is_none());
        assert!(page.prev_page.is_none());
        assert_eq!(page.post().unwrap().id, post.id);
    }

    #[tokio::test]
    async fn test_get_post_by_id_not_found() {
        let posts = PostsRepositoryMock::new(vec![]);
        let search_client = SearchClientMock::new(vec![]);
        let page = GetPostByIdUseCase::execute(&posts, &search_client, &PostId(1)).await;
        assert!(page.is_err());
    }
}
