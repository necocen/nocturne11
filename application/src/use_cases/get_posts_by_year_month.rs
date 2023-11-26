use anyhow::Context as _;
use chrono::{DateTime, Utc};

use crate::{
    adapters::{PostsRepository, SearchClient},
    models::{AdjacentPageInfo, Page, YearMonth},
    ApplicationResult,
};

pub struct GetPostsByYearMonthUseCase;

impl GetPostsByYearMonthUseCase {
    pub async fn execute<'a>(
        posts: &impl PostsRepository,
        search_client: &impl SearchClient,
        year_month: &'a YearMonth,
        page_index: usize,
    ) -> ApplicationResult<Page<'a, YearMonth, usize>> {
        let result = search_client
            .find_by_year_month(year_month, (page_index - 1) * 10, 10) // TODO: per_page
            .await?;
        let result_posts = posts.get_by_ids(&result.post_ids).await?;

        let next_page = if page_index * 10 < result.total_count {
            Some(AdjacentPageInfo::PageIndex(page_index + 1))
        } else {
            let next_post_ids = if result.total_count > 0 {
                let last_post = result_posts.last().unwrap();
                search_client
                    .get_from_date(last_post.created_at, 1, 1)
                    .await?
            } else {
                search_client
                    .get_from_date(DateTime::<Utc>::from(*year_month), 0, 1)
                    .await?
            };
            if let Some(next_post_id) = next_post_ids.first() {
                let post = posts
                    .get_by_id(next_post_id)
                    .await?
                    .context("post of next month not found")?;
                Some(AdjacentPageInfo::Condition(YearMonth::from(
                    post.created_at,
                )))
            } else {
                None
            }
        };
        let prev_page = if page_index > 1 {
            Some(AdjacentPageInfo::PageIndex(page_index - 1))
        } else {
            let prev_post_ids = if result.total_count > 0 {
                let first_post = result_posts.first().unwrap();
                search_client
                    .get_until_date(first_post.created_at, 0, 1)
                    .await?
            } else {
                search_client
                    .get_until_date(DateTime::<Utc>::from(*year_month), 0, 1)
                    .await?
            };
            if let Some(prev_post_id) = prev_post_ids.first() {
                let post = posts
                    .get_by_id(prev_post_id)
                    .await?
                    .context("post of prev month not found")?;
                Some(AdjacentPageInfo::Condition(YearMonth::from(
                    post.created_at,
                )))
            } else {
                None
            }
        };

        Ok(Page {
            condition: year_month,
            index: page_index,
            posts: posts.get_by_ids(&result.post_ids).await?,
            next_page,
            prev_page,
        })
    }
}
