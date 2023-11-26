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
    use crate::adapters::*;
    use assert_matches::assert_matches;
    use chrono::Utc;
    use domain::entities::Post;
    use mockall::predicate::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_get_post_by_id() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let now = Utc::now();
        let post_id = PostId(629);
        mock_posts
            .expect_get_by_id()
            .with(eq(post_id))
            .returning(move |_| {
                Ok(Some(Post::new(
                    post_id,
                    "test title",
                    "test body",
                    now,
                    now,
                )))
            });
        mock_search
            .expect_get_from_date()
            .with(eq(now), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(630)]));
        mock_search
            .expect_get_until_date()
            .with(eq(now), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));

        let page = GetPostByIdUseCase::execute(&mock_posts, &mock_search, &post_id)
            .await
            .unwrap();

        assert_eq!(page.condition, &post_id);
        assert_eq!(page.posts.len(), 1);
        assert_matches!(
            page.next_page.as_ref().unwrap(),
            AdjacentPageInfo::Condition(PostId(630))
        );
        assert_matches!(
            page.prev_page.as_ref().unwrap(),
            AdjacentPageInfo::Condition(PostId(628))
        );
        assert_eq!(page.post().unwrap().id, post_id);
    }

    #[tokio::test]
    async fn test_get_post_by_id_first_post() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let now = Utc::now();
        let post_id = PostId(1);
        mock_posts
            .expect_get_by_id()
            .with(eq(post_id))
            .returning(move |_| {
                Ok(Some(Post::new(
                    post_id,
                    "test title",
                    "test body",
                    now,
                    now,
                )))
            });
        mock_search
            .expect_get_from_date()
            .with(eq(now), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(2)]));
        mock_search
            .expect_get_until_date()
            .with(eq(now), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![]));

        let page = GetPostByIdUseCase::execute(&mock_posts, &mock_search, &post_id)
            .await
            .unwrap();

        assert_eq!(page.condition, &post_id);
        assert_eq!(page.posts.len(), 1);
        assert_matches!(
            page.next_page.as_ref().unwrap(),
            AdjacentPageInfo::Condition(PostId(2))
        );
        assert!(page.prev_page.is_none());
        assert_eq!(page.post().unwrap().id, post_id);
    }

    #[tokio::test]
    async fn test_get_post_by_id_last_post() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let now = Utc::now();
        let post_id = PostId(629);
        mock_posts
            .expect_get_by_id()
            .with(eq(post_id))
            .returning(move |_| {
                Ok(Some(Post::new(
                    post_id,
                    "test title",
                    "test body",
                    now,
                    now,
                )))
            });
        mock_search
            .expect_get_from_date()
            .with(eq(now), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![]));
        mock_search
            .expect_get_until_date()
            .with(eq(now), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));

        let page = GetPostByIdUseCase::execute(&mock_posts, &mock_search, &post_id)
            .await
            .unwrap();

        assert_eq!(page.condition, &post_id);
        assert_eq!(page.posts.len(), 1);
        assert!(page.next_page.is_none());
        assert_matches!(
            page.prev_page.as_ref().unwrap(),
            AdjacentPageInfo::Condition(PostId(628))
        );
        assert_eq!(page.post().unwrap().id, post_id);
    }

    #[tokio::test]
    async fn test_get_post_by_id_only_one() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let now = Utc::now();
        let post_id = PostId(629);
        mock_posts
            .expect_get_by_id()
            .with(eq(post_id))
            .returning(move |_| {
                Ok(Some(Post::new(
                    post_id,
                    "test title",
                    "test body",
                    now,
                    now,
                )))
            });
        mock_search
            .expect_get_from_date()
            .with(eq(now), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![]));
        mock_search
            .expect_get_until_date()
            .with(eq(now), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![]));

        let page = GetPostByIdUseCase::execute(&mock_posts, &mock_search, &post_id)
            .await
            .unwrap();

        assert_eq!(page.condition, &post_id);
        assert_eq!(page.posts.len(), 1);
        assert!(page.next_page.is_none());
        assert!(page.prev_page.is_none());
        assert_eq!(page.post().unwrap().id, post_id);
    }

    #[tokio::test]
    async fn test_get_post_by_id_not_found() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let post_id = PostId(629);
        mock_posts
            .expect_get_by_id()
            .with(eq(post_id))
            .returning(move |_| Ok(None));
        mock_search.expect_get_from_date().never();
        mock_search.expect_get_until_date().never();

        let result = GetPostByIdUseCase::execute(&mock_posts, &mock_search, &post_id).await;

        assert_matches!(result, Err(ApplicationError::PostNotFound));
    }
}
