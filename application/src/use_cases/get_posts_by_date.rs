use anyhow::Context as _;
use chrono::{Local, NaiveDate, Utc};

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
        let result_posts = if result.post_ids.is_empty() {
            vec![]
        } else {
            posts.get_by_ids(&result.post_ids).await?
        };
        let next_page = if page_index * 10 < result.total_count {
            Some(AdjacentPageInfo::PageIndex(page_index + 1))
        } else {
            let next_post_ids = if result.post_ids.is_empty() {
                search_client
                    .get_from_date(
                        date.and_hms_opt(0, 0, 0)
                            .unwrap()
                            .and_local_timezone(Local)
                            .unwrap()
                            .with_timezone(&Utc),
                        0,
                        1,
                    )
                    .await?
            } else {
                let last_post = result_posts.last().unwrap();
                search_client
                    .get_from_date(last_post.created_at, 1, 1)
                    .await?
            };
            if let Some(next_post_id) = next_post_ids.first() {
                let post = posts
                    .get_by_id(next_post_id)
                    .await?
                    .context("post of next month not found")?;
                Some(AdjacentPageInfo::Condition(
                    post.created_at.with_timezone(&Local).date_naive(),
                ))
            } else {
                None
            }
        };
        let prev_page = if page_index > 1 && result.total_count > 0 {
            let max_page_index = (result.total_count + 9) / 10;
            Some(AdjacentPageInfo::PageIndex(
                max_page_index.min(page_index - 1),
            ))
        } else {
            let prev_post_ids = if result.post_ids.is_empty() {
                search_client
                    .get_until_date(
                        date.and_hms_opt(0, 0, 0)
                            .unwrap()
                            .and_local_timezone(Local)
                            .unwrap()
                            .with_timezone(&Utc),
                        0,
                        1,
                    )
                    .await?
            } else {
                let first_post = result_posts.first().unwrap();
                search_client
                    .get_until_date(first_post.created_at, 0, 1)
                    .await?
            };
            if let Some(prev_post_id) = prev_post_ids.first() {
                let post = posts
                    .get_by_id(prev_post_id)
                    .await?
                    .context("post of prev month not found")?;
                Some(AdjacentPageInfo::Condition(
                    post.created_at.with_timezone(&Local).date_naive(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::*, models::SearchResult};
    use chrono::{Duration, Local, Utc};
    use domain::entities::{Post, PostId};
    use mockall::predicate::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_get_posts_by_date() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let date2 = date1 + Duration::hours(12);
        let next_date = date1 + Duration::days(3);
        let prev_date = date1 - Duration::days(3);
        let posts = vec![
            Post::new(PostId(629), "test title", "test body", date1, date1),
            Post::new(PostId(630), "test title2", "test body2", date2, date2),
        ];
        let next_post = Post::new(
            PostId(631),
            "test title3",
            "test body3",
            next_date,
            next_date,
        );
        let prev_post = Post::new(
            PostId(628),
            "test title4",
            "test body4",
            prev_date,
            prev_date,
        );
        let post_ids = posts.iter().map(|p| p.id).collect::<Vec<_>>();
        let post_ids_clone = post_ids.clone();
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &0 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: post_ids_clone.len(),
                    post_ids: post_ids_clone.clone(),
                })
            });
        mock_posts
            .expect_get_by_ids()
            .withf(move |ids| ids == post_ids.clone())
            .returning(move |_| Ok(posts.clone()));
        mock_search
            .expect_get_from_date()
            .with(eq(date2), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(631)]));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));
        mock_posts
            .expect_get_by_id()
            .with(eq(next_post.id))
            .returning(move |_| Ok(Some(next_post.clone())));
        mock_posts
            .expect_get_by_id()
            .with(eq(prev_post.id))
            .returning(move |_| Ok(Some(prev_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 1)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 1);
        assert_eq!(page.posts.len(), 2);
        assert_eq!(page.posts[0].id, PostId(629));
        assert_eq!(page.posts[1].id, PostId(630));
        assert_eq!(
            page.next_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 9, 4).unwrap()
            ))
        );
        assert_eq!(
            page.prev_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 8, 29).unwrap()
            ))
        );
    }

    #[tokio::test]
    async fn test_get_posts_by_date_in_first_day() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let date2 = date1 + Duration::hours(12);
        let next_date = date1 + Duration::days(3);
        let posts = vec![
            Post::new(PostId(629), "test title", "test body", date1, date1),
            Post::new(PostId(630), "test title2", "test body2", date2, date2),
        ];
        let next_post = Post::new(
            PostId(631),
            "test title3",
            "test body3",
            next_date,
            next_date,
        );
        let post_ids = posts.iter().map(|p| p.id).collect::<Vec<_>>();
        let post_ids_clone = post_ids.clone();
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &0 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: post_ids_clone.len(),
                    post_ids: post_ids_clone.clone(),
                })
            });
        mock_posts
            .expect_get_by_ids()
            .withf(move |ids| ids == post_ids.clone())
            .returning(move |_| Ok(posts.clone()));
        mock_search
            .expect_get_from_date()
            .with(eq(date2), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(631)]));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![]));
        mock_posts
            .expect_get_by_id()
            .with(eq(next_post.id))
            .returning(move |_| Ok(Some(next_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 1)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 1);
        assert_eq!(page.posts.len(), 2);
        assert_eq!(page.posts[0].id, PostId(629));
        assert_eq!(page.posts[1].id, PostId(630));
        assert_eq!(
            page.next_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 9, 4).unwrap()
            ))
        );
        assert_eq!(page.prev_page, None);
    }

    #[tokio::test]
    async fn test_get_posts_by_date_in_last_day() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let date2 = date1 + Duration::hours(12);
        let prev_date = date1 - Duration::days(3);
        let posts = vec![
            Post::new(PostId(629), "test title", "test body", date1, date1),
            Post::new(PostId(630), "test title2", "test body2", date2, date2),
        ];
        let prev_post = Post::new(
            PostId(628),
            "test title4",
            "test body4",
            prev_date,
            prev_date,
        );
        let post_ids = posts.iter().map(|p| p.id).collect::<Vec<_>>();
        let post_ids_clone = post_ids.clone();
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &0 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: post_ids_clone.len(),
                    post_ids: post_ids_clone.clone(),
                })
            });
        mock_posts
            .expect_get_by_ids()
            .withf(move |ids| ids == post_ids.clone())
            .returning(move |_| Ok(posts.clone()));
        mock_search
            .expect_get_from_date()
            .with(eq(date2), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![]));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));
        mock_posts
            .expect_get_by_id()
            .with(eq(prev_post.id))
            .returning(move |_| Ok(Some(prev_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 1)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 1);
        assert_eq!(page.posts.len(), 2);
        assert_eq!(page.posts[0].id, PostId(629));
        assert_eq!(page.posts[1].id, PostId(630));
        assert_eq!(page.next_page, None);
        assert_eq!(
            page.prev_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 8, 29).unwrap()
            ))
        );
    }

    #[tokio::test]
    async fn test_get_posts_by_date_with_many_pages() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let date2 = date1 + Duration::hours(12);
        let prev_date = date1 - Duration::days(3);
        let posts = vec![
            Post::new(PostId(629), "test title", "test body", date1, date1),
            Post::new(PostId(630), "test title2", "test body2", date2, date2),
            Post::new(PostId(631), "test title2", "test body2", date2, date2),
            Post::new(PostId(632), "test title2", "test body2", date2, date2),
            Post::new(PostId(633), "test title2", "test body2", date2, date2),
            Post::new(PostId(634), "test title2", "test body2", date2, date2),
            Post::new(PostId(635), "test title2", "test body2", date2, date2),
            Post::new(PostId(636), "test title2", "test body2", date2, date2),
            Post::new(PostId(637), "test title2", "test body2", date2, date2),
            Post::new(PostId(638), "test title2", "test body2", date2, date2),
        ];
        let posts_in_next_page = vec![
            Post::new(PostId(639), "test title2", "test body2", date2, date2),
            Post::new(PostId(640), "test title2", "test body2", date2, date2),
            Post::new(PostId(641), "test title2", "test body2", date2, date2),
        ];
        let prev_post = Post::new(
            PostId(628),
            "test title4",
            "test body4",
            prev_date,
            prev_date,
        );
        let post_ids = posts.iter().map(|p| p.id).collect::<Vec<_>>();
        let post_ids_clone = post_ids.clone();
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &0 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: post_ids_clone.len() + posts_in_next_page.len(),
                    post_ids: post_ids_clone.clone(),
                })
            });
        mock_posts
            .expect_get_by_ids()
            .withf(move |ids| ids == post_ids.clone())
            .returning(move |_| Ok(posts.clone()));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));
        mock_posts
            .expect_get_by_id()
            .with(eq(prev_post.id))
            .returning(move |_| Ok(Some(prev_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 1)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 1);
        assert_eq!(page.posts.len(), 10);
        assert_eq!(page.posts[0].id, PostId(629));
        assert_eq!(page.posts[1].id, PostId(630));
        assert_eq!(page.posts[9].id, PostId(638));
        assert_eq!(page.next_page, Some(AdjacentPageInfo::PageIndex(2)));
        assert_eq!(
            page.prev_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 8, 29).unwrap()
            ))
        );
    }

    #[tokio::test]
    async fn test_get_posts_by_date_with_many_pages_in_final_page() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let date2 = date1 + Duration::hours(12);
        let next_date = date1 + Duration::days(3);
        let posts_in_prev_page = vec![
            Post::new(PostId(629), "test title", "test body", date1, date1),
            Post::new(PostId(630), "test title2", "test body2", date2, date2),
            Post::new(PostId(631), "test title2", "test body2", date2, date2),
            Post::new(PostId(632), "test title2", "test body2", date2, date2),
            Post::new(PostId(633), "test title2", "test body2", date2, date2),
            Post::new(PostId(634), "test title2", "test body2", date2, date2),
            Post::new(PostId(635), "test title2", "test body2", date2, date2),
            Post::new(PostId(636), "test title2", "test body2", date2, date2),
            Post::new(PostId(637), "test title2", "test body2", date2, date2),
            Post::new(PostId(638), "test title2", "test body2", date2, date2),
        ];
        let posts = vec![
            Post::new(PostId(639), "test title2", "test body2", date2, date2),
            Post::new(PostId(640), "test title2", "test body2", date2, date2),
            Post::new(PostId(641), "test title2", "test body2", date2, date2),
        ];
        let next_post = Post::new(
            PostId(642),
            "test title3",
            "test body3",
            next_date,
            next_date,
        );
        let post_ids = posts.iter().map(|p| p.id).collect::<Vec<_>>();
        let post_ids_clone = post_ids.clone();
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &10 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: post_ids_clone.len() + posts_in_prev_page.len(),
                    post_ids: post_ids_clone.clone(),
                })
            });
        mock_posts
            .expect_get_by_ids()
            .withf(move |ids| ids == post_ids.clone())
            .returning(move |_| Ok(posts.clone()));
        mock_search
            .expect_get_from_date()
            .with(eq(date2), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(642)]));
        mock_posts
            .expect_get_by_id()
            .with(eq(PostId(642)))
            .returning(move |_| Ok(Some(next_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 2)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 2);
        assert_eq!(page.posts.len(), 3);
        assert_eq!(page.posts[0].id, PostId(639));
        assert_eq!(page.posts[1].id, PostId(640));
        assert_eq!(page.posts[2].id, PostId(641));
        assert_eq!(
            page.next_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 9, 4).unwrap()
            ))
        );
        assert_eq!(page.prev_page, Some(AdjacentPageInfo::PageIndex(1)));
    }

    #[tokio::test]
    async fn test_get_posts_by_date_on_empty_day_with_page() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let next_date = date1 + Duration::days(3);
        let prev_date = date1 - Duration::days(3);
        let next_post = Post::new(
            PostId(631),
            "test title3",
            "test body3",
            next_date,
            next_date,
        );
        let prev_post = Post::new(
            PostId(628),
            "test title4",
            "test body4",
            prev_date,
            prev_date,
        );
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &10 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: 0,
                    post_ids: vec![],
                })
            });
        mock_search
            .expect_get_from_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(631)]));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));
        mock_posts
            .expect_get_by_id()
            .with(eq(next_post.id))
            .returning(move |_| Ok(Some(next_post.clone())));
        mock_posts
            .expect_get_by_id()
            .with(eq(prev_post.id))
            .returning(move |_| Ok(Some(prev_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 2)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 2);
        assert_eq!(page.posts.len(), 0);
        assert_eq!(
            page.next_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 9, 4).unwrap()
            ))
        );
        assert_eq!(
            page.prev_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 8, 29).unwrap()
            ))
        );
    }

    #[tokio::test]
    async fn test_get_posts_by_date_on_empty_day() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let next_date = date1 + Duration::days(3);
        let prev_date = date1 - Duration::days(3);
        let next_post = Post::new(
            PostId(631),
            "test title3",
            "test body3",
            next_date,
            next_date,
        );
        let prev_post = Post::new(
            PostId(628),
            "test title4",
            "test body4",
            prev_date,
            prev_date,
        );
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &0 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: 0,
                    post_ids: vec![],
                })
            });
        mock_search
            .expect_get_from_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(631)]));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));
        mock_posts
            .expect_get_by_id()
            .with(eq(next_post.id))
            .returning(move |_| Ok(Some(next_post.clone())));
        mock_posts
            .expect_get_by_id()
            .with(eq(prev_post.id))
            .returning(move |_| Ok(Some(prev_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 1)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 1);
        assert_eq!(page.posts.len(), 0);
        assert_eq!(
            page.next_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 9, 4).unwrap()
            ))
        );
        assert_eq!(
            page.prev_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 8, 29).unwrap()
            ))
        );
    }

    #[tokio::test]
    async fn test_get_posts_by_date_in_sole_day() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let date2 = date1 + Duration::hours(12);
        let posts = vec![
            Post::new(PostId(629), "test title", "test body", date1, date1),
            Post::new(PostId(630), "test title2", "test body2", date2, date2),
        ];
        let post_ids = posts.iter().map(|p| p.id).collect::<Vec<_>>();
        let post_ids_clone = post_ids.clone();
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &0 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: post_ids_clone.len(),
                    post_ids: post_ids_clone.clone(),
                })
            });
        mock_posts
            .expect_get_by_ids()
            .withf(move |ids| ids == post_ids.clone())
            .returning(move |_| Ok(posts.clone()));
        mock_search
            .expect_get_from_date()
            .with(eq(date2), eq(1), eq(1))
            .returning(|_, _, _| Ok(vec![]));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![]));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 1)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 1);
        assert_eq!(page.posts.len(), 2);
        assert_eq!(page.posts[0].id, PostId(629));
        assert_eq!(page.posts[1].id, PostId(630));
        assert_eq!(page.next_page, None);
        assert_eq!(page.prev_page, None);
    }

    #[tokio::test]
    async fn test_get_posts_by_date_in_empty_page() {
        let mut mock_posts = MockPostsRepository::new();
        let mut mock_search = MockSearchClient::new();
        let date = NaiveDate::from_ymd_opt(1989, 9, 1).unwrap();
        let date1 = date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        let next_date = date1 + Duration::days(3);
        let prev_date = date1 - Duration::days(3);
        let next_post = Post::new(
            PostId(631),
            "test title3",
            "test body3",
            next_date,
            next_date,
        );
        let prev_post = Post::new(
            PostId(628),
            "test title4",
            "test body4",
            prev_date,
            prev_date,
        );
        mock_search
            .expect_find_by_date()
            .withf(move |d, o, l| d == &date && o == &20 && l == &10)
            .returning(move |_, _, _| {
                Ok(SearchResult {
                    total_count: 2,
                    post_ids: vec![],
                })
            });
        mock_search
            .expect_get_from_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(631)]));
        mock_search
            .expect_get_until_date()
            .with(eq(date1), eq(0), eq(1))
            .returning(|_, _, _| Ok(vec![PostId(628)]));
        mock_posts
            .expect_get_by_id()
            .with(eq(next_post.id))
            .returning(move |_| Ok(Some(next_post.clone())));
        mock_posts
            .expect_get_by_id()
            .with(eq(prev_post.id))
            .returning(move |_| Ok(Some(prev_post.clone())));

        let page = GetPostsByDateUseCase::execute(&mock_posts, &mock_search, &date, 3)
            .await
            .unwrap();
        assert_eq!(page.condition, &date);
        assert_eq!(page.index, 3);
        assert_eq!(page.posts.len(), 0);
        assert_eq!(
            page.next_page,
            Some(AdjacentPageInfo::Condition(
                NaiveDate::from_ymd_opt(1989, 9, 4).unwrap()
            ))
        );
        assert_eq!(page.prev_page, Some(AdjacentPageInfo::PageIndex(1)));
    }
}
