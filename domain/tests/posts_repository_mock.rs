use anyhow::{Context, Result};
use chrono::{Local, TimeZone, Utc};
use domain::repositories::posts::PostsRepository;
use domain::{
    entities::{date::*, *},
    repositories::import_posts::ImportPostsRepository,
};
use std::cell::{Cell, RefCell};

#[derive(Debug, Clone, Default)]
pub struct PostsRepositoryMock {
    posts: RefCell<Vec<Post>>,
    sequence: Cell<i32>,
}

impl PostsRepositoryMock {
    pub fn new() -> PostsRepositoryMock {
        let mut posts = (1..=6)
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
            .collect::<Vec<_>>();
        posts.sort_by_key(|post| post.id);
        let sequence = posts.last().unwrap().id;
        PostsRepositoryMock {
            posts: RefCell::new(posts),
            sequence: Cell::new(sequence),
        }
    }
}

impl PostsRepository for PostsRepositoryMock {
    fn get(&self, id: PostId) -> Result<Post> {
        Ok(self
            .posts
            .borrow()
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
            .borrow()
            .clone()
            .into_iter()
            .filter(|p| p.created_at >= from)
            .collect::<Vec<_>>();
        posts.sort_by_key(|p| p.created_at);
        Ok(posts.into_iter().skip(offset).take(limit).collect())
    }

    fn get_until_date<Tz: TimeZone>(
        &self,
        until: chrono::DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Post>> {
        let mut posts = self
            .posts
            .borrow()
            .clone()
            .into_iter()
            .filter(|p| p.created_at < until)
            .collect::<Vec<_>>();
        posts.sort_by_key(|p| p.created_at);
        Ok(posts.into_iter().rev().skip(offset).take(limit).collect())
    }

    fn get_all(&self, offset: usize, limit: usize) -> Result<Vec<Post>> {
        let mut posts = self.posts.borrow().clone().into_iter().collect::<Vec<_>>();
        posts.sort_by_key(|p| p.created_at);
        Ok(posts.into_iter().rev().skip(offset).take(limit).collect())
    }

    fn get_year_months(&self) -> Result<Vec<YearMonth>> {
        Ok((1..=6).map(|m| YearMonth(2020, m * 2)).collect())
    }

    fn get_days(&self, ym: YearMonth) -> Result<Vec<u8>> {
        Ok((1..=14).map(|d| d * 2 - ym.1 % 2).collect())
    }

    fn create(&self, new_post: NewPost) -> Result<Post> {
        let NewPost {
            title,
            body,
            created_at,
        } = new_post;
        self.sequence.set(self.sequence.get() + 1);
        let post = Post {
            id: self.sequence.get(),
            title,
            body,
            created_at,
            updated_at: created_at,
        };
        self.posts.borrow_mut().push(post.clone());
        Ok(post)
    }
}

impl ImportPostsRepository for PostsRepositoryMock {
    fn import(&self, posts: Vec<Post>) -> Result<Vec<Post>> {
        self.posts.borrow_mut().append(&mut posts.clone());
        Ok(posts)
    }

    fn reset_id_sequence(&self) -> Result<()> {
        let max_id = self
            .posts
            .borrow()
            .iter()
            .map(|post| post.id)
            .max()
            .unwrap_or(0);
        self.sequence.set(max_id);
        Ok(())
    }
}
