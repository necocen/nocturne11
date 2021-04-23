use crate::entities::*;
use crate::use_cases::*;
use crate::{entities::date::YearMonth, repositories::posts::PostsRepository};
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
                    vec![
                        Post {
                            id: m * 2 * 100 + d * 2 as i32,
                            title: format!("{}-{}-00", m * 2, d * 2 - m % 2).to_string(),
                            body: format!("{}-{}-00", m * 2, d * 2 - m % 2).to_string(),
                            created_at: Local
                                .ymd(2020, (m * 2) as u32, (d * 2 - m % 2) as u32)
                                .and_hms(0, 0, 0)
                                .with_timezone(&Utc),
                            updated_at: Local
                                .ymd(2020, (m * 2) as u32, (d * 2 - m % 2) as u32)
                                .and_hms(0, 0, 0)
                                .with_timezone(&Utc),
                        },
                        Post {
                            id: m * 2 * 100 + d * 2 + 1 as i32,
                            title: format!("{}-{}-12", m * 2, d * 2 - m % 2).to_string(),
                            body: format!("{}-{}-12", m * 2, d * 2 - m % 2).to_string(),
                            created_at: Local
                                .ymd(2020, (m * 2) as u32, (d * 2 - m % 2) as u32)
                                .and_hms(12, 0, 0)
                                .with_timezone(&Utc),
                            updated_at: Local
                                .ymd(2020, (m * 2) as u32, (d * 2 - m % 2) as u32)
                                .and_hms(12, 0, 0)
                                .with_timezone(&Utc),
                        },
                    ]
                })
            })
            .collect();

        PostRepositoryMock { posts }
    }
}

impl PostsRepository for PostRepositoryMock {
    fn get(&self, id: i32) -> Result<Post> {
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
