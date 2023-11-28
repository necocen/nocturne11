use crate::{
    adapters::{PostsRepository, SearchClient},
    models::{AdjacentPageInfo, Page, PageNumber},
    ApplicationResult,
};

pub struct GetLatestPostsUseCase;

impl GetLatestPostsUseCase {
    pub async fn execute(
        posts: &impl PostsRepository,
        search_client: &impl SearchClient,
        page_index: PageNumber,
    ) -> ApplicationResult<Page<'static, (), PageNumber>> {
        let result = search_client
            .get_latest_posts((page_index.0 - 1) * 10, 10) // TODO: per_page
            .await?;

        let next_page = if page_index.0 * 10 < result.total_count {
            Some(AdjacentPageInfo::PageIndex(page_index.next()))
        } else {
            None
        };
        let prev_page = if page_index.0 > 1 {
            Some(AdjacentPageInfo::PageIndex(
                page_index.prev().expect("page_index > 1"),
            ))
        } else {
            None
        };

        Ok(Page {
            condition: &(),
            index: page_index,
            posts: posts.get_by_ids(&result.post_ids).await?,
            next_page,
            prev_page,
        })
    }
}
