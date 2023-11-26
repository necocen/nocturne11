use anyhow::Context as _;
use chrono::{NaiveDate, TimeZone as _, Utc};

use crate::{
    adapters::{PostsRepository, SearchClient},
    models::{AdjacentPageInfo, Page},
};

pub struct GetPostsByDateUseCase;

impl GetPostsByDateUseCase {
    pub async fn execute<'a>(
        posts: &impl PostsRepository,
        search_client: &impl SearchClient,
        date: &'a NaiveDate,
        page_index: usize,
    ) -> anyhow::Result<Page<'a, NaiveDate, usize>> {
        let result = search_client
            .find_by_date(date, (page_index - 1) * 10, 10)
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
                    .get_from_date(
                        Utc.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
                            .unwrap(),
                        0,
                        1,
                    )
                    .await?
            };
            if let Some(next_post_id) = next_post_ids.first() {
                let post = posts
                    .get_by_id(next_post_id)
                    .await?
                    .context("post of next month not found")?;
                Some(AdjacentPageInfo::Condition(
                    post.created_at.naive_local().date(),
                ))
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
                    .get_until_date(
                        Utc.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
                            .unwrap(),
                        0,
                        1,
                    )
                    .await?
            };
            if let Some(prev_post_id) = prev_post_ids.first() {
                let post = posts
                    .get_by_id(prev_post_id)
                    .await?
                    .context("post of prev month not found")?;
                Some(AdjacentPageInfo::Condition(
                    post.created_at.naive_local().date(),
                ))
            } else {
                None
            }
        };

        Ok(Page {
            condition: date,
            index: page_index,
            posts: result_posts,
            next_page,
            prev_page,
        })
    }
}
