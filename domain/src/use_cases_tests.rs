use crate::entities::{date::*, *};
use crate::repositories::posts::PostsRepository;
use crate::use_cases::*;
use anyhow::{Context, Result};
use chrono::{Local, TimeZone, Utc};

#[derive(Debug, Clone)]
struct PostRepositoryMock {
    posts: Vec<Post>,
}

impl PostRepositoryMock {
    fn new() -> PostRepositoryMock {
        let posts = (1..=6)
            .flat_map(|m| {
                (1..=14).flat_map(move |d| {
                    let date = Local.ymd(2020i32, (m * 2) as u32, (d * 2 - m % 2) as u32);
                    let date_time00 = date.and_hms(0, 0, 0).with_timezone(&Utc);
                    let date_time12 = date.and_hms(12, 0, 0).with_timezone(&Utc);
                    vec![
                        Post {
                            id: m * 2 * 100 + d * 2,
                            title: String::new(),
                            body: String::new(),
                            created_at: date_time00,
                            updated_at: date_time00,
                        },
                        Post {
                            id: m * 2 * 100 + d * 2 + 1,
                            title: String::new(),
                            body: String::new(),
                            created_at: date_time12,
                            updated_at: date_time12,
                        },
                    ]
                })
            })
            .collect();

        PostRepositoryMock { posts }
    }
}

impl PostsRepository for PostRepositoryMock {
    fn get(&self, id: PostId) -> Result<Post> {
        Ok(self
            .posts
            .iter()
            .find(|p| p.id == id)
            .context("Not Found")?
            .clone())
    }

    fn get_from_date<Tz: TimeZone>(
        &self,
        from: chrono::DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Post>> {
        let mut posts = self
            .posts
            .iter()
            .filter(|p| p.created_at >= from)
            .collect::<Vec<_>>();
        posts.sort_by_key(|p| p.created_at);
        Ok(posts
            .into_iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect())
    }

    fn get_until_date<Tz: TimeZone>(
        &self,
        until: chrono::DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Post>> {
        let mut posts = self
            .posts
            .iter()
            .filter(|p| p.created_at < until)
            .collect::<Vec<_>>();
        posts.sort_by_key(|p| p.created_at);
        Ok(posts
            .into_iter()
            .rev()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect())
    }

    fn get_all(&self) -> Result<Vec<Post>> {
        Ok(self.posts.clone())
    }

    fn get_year_months(&self) -> Result<Vec<YearMonth>> {
        Ok((1..=6).map(|m| YearMonth(2020, m * 2)).collect())
    }

    fn get_days(&self, ym: YearMonth) -> Result<Vec<u8>> {
        Ok((1..=14).map(|d| d * 2 - ym.1 % 2).collect())
    }

    fn insert(&self, _post: &Post) -> Result<Post> {
        unimplemented!("This is mock")
    }
}

#[test]
fn test_get_days() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let odd_days = get_days(&repo, YearMonth(2020, 1))?;
    assert_eq!(
        odd_days,
        [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27]
    );
    let even_days = get_days(&repo, YearMonth(2020, 2))?;
    assert_eq!(
        even_days,
        [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28]
    );
    Ok(())
}

#[test]
fn test_get_years() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let months = get_years(&repo)?
        .into_iter()
        .flat_map(|y| y.months)
        .collect::<Vec<_>>();
    assert_eq!(months, [2, 4, 6, 8, 10, 12]);
    Ok(())
}

#[test]
fn test_get_post_with_id_not_found() {
    let repo = PostRepositoryMock::new();
    let page = get_post_with_id(&repo, &9999);
    assert!(page.is_err());
}

#[test]
fn test_get_post_with_id_which_has_prev_only() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &1229)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [1229]);
    assert_eq!(prev_page, AdjacentPage::Condition(1228));
    assert_eq!(next_page, AdjacentPage::None);
    Ok(())
}

#[test]
fn test_get_post_with_id_which_has_next_only() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &202)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [202]);
    assert_eq!(prev_page, AdjacentPage::None);
    assert_eq!(next_page, AdjacentPage::Condition(203));
    Ok(())
}

#[test]
fn test_get_post_with_id_which_has_prev_and_next() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &228)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [228]);
    assert_eq!(prev_page, AdjacentPage::Condition(227));
    assert_eq!(next_page, AdjacentPage::Condition(229));
    Ok(())
}

#[test]
fn test_get_post_with_id_which_has_prev() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &228)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [228]);
    assert_eq!(prev_page, AdjacentPage::Condition(227));
    assert_eq!(next_page, AdjacentPage::Condition(229));
    Ok(())
}

#[test]
fn test_get_post_by_month_not_found() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 3),
        day: None,
    };
    let Page {
        posts, next_page, ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 1)?;
    assert!(posts.is_empty());
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 4),
            day: None
        })
    );
    Ok(())
}

#[test]
fn test_get_post_by_month_first_page() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: None,
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 1)?;
    assert_eq!(
        posts.into_iter().map(|p| p.id).collect::<Vec<_>>(),
        [202, 203, 204, 205, 206]
    );
    assert_eq!(next_page, AdjacentPage::Page(2));
    assert_eq!(prev_page, AdjacentPage::None);
    Ok(())
}

#[test]
fn test_get_post_by_month_second_page() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: None,
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 2)?;
    assert_eq!(
        posts.into_iter().map(|p| p.id).collect::<Vec<_>>(),
        [207, 208, 209, 210, 211]
    );
    assert_eq!(next_page, AdjacentPage::Page(3));
    assert_eq!(prev_page, AdjacentPage::Page(1));
    Ok(())
}

#[test]
fn test_get_post_by_month_last_page() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: None,
    };
    let Page {
        posts, next_page, ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 6)?;
    assert_eq!(
        posts.into_iter().map(|p| p.id).collect::<Vec<_>>(),
        [227, 228, 229]
    );
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 4),
            day: None
        })
    );
    Ok(())
}

#[test]
fn test_get_post_by_day_not_found() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 3),
        day: Some(1),
    };
    let Page {
        posts, next_page, ..
    } = get_posts_with_date_condition(&repo, &cond, 1, 1)?;
    assert!(posts.is_empty());
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 3),
            day: Some(3)
        })
    );
    Ok(())
}

#[test]
fn test_get_post_by_day_first_page() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: Some(1),
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 1, 1)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [202]);
    assert_eq!(next_page, AdjacentPage::Page(2));
    assert_eq!(prev_page, AdjacentPage::None);
    Ok(())
}

#[test]
fn test_get_post_by_day_last_page() -> Result<()> {
    let repo = PostRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: Some(1),
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 1, 2)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [203]);
    assert_eq!(prev_page, AdjacentPage::Page(1));
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 2),
            day: Some(2)
        })
    );
    Ok(())
}
