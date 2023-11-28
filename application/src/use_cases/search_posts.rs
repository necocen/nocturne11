use crate::{
    adapters::{PostsRepository, SearchClient},
    models::{AdjacentPageInfo, Page, PageNumber},
    ApplicationResult,
};

pub struct SearchPostsUseCase;

impl SearchPostsUseCase {
    pub async fn execute<'a, S: SearchClient>(
        search_client: &S,
        posts: &impl PostsRepository,
        keywords: &'a Vec<&'a str>,
        page_index: PageNumber,
    ) -> ApplicationResult<Page<'a, Vec<&'a str>, PageNumber>> {
        let result = search_client
            .find_by_keywords(keywords, (page_index.0 - 1) * 10, 10) // TODO: per_page
            .await?;

        let next_page = if page_index.0 * 10 < result.total_count {
            Some(AdjacentPageInfo::PageIndex(page_index.next()))
        } else {
            None
        };
        let prev_page = if page_index.0 > 1 {
            let max_page_index = (result.total_count + 9) / 10;
            Some(AdjacentPageInfo::PageIndex(
                PageNumber::new(max_page_index.min(page_index.0 - 1)).expect("page_index > 1"),
            ))
        } else {
            None
        };

        Ok(Page {
            condition: keywords,
            index: page_index,
            posts: if result.post_ids.is_empty() {
                vec![]
            } else {
                posts.get_by_ids(&result.post_ids).await?
            },
            next_page,
            prev_page,
        })
    }
}
