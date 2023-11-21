use chrono::{DateTime, Local, TimeZone, Utc};
use domain::entities::{date::*, *};
use domain::repositories::posts::{Error, PostsRepository, Result as PostsResult};
use std::cell::{Cell, RefCell};

#[derive(Debug, Clone, Default)]
pub struct PostsRepositoryMock {
    posts: RefCell<Vec<Post>>,
    sequence: Cell<PostId>,
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
                        Post::new(PostId(m * 2 * 100 + d * 2), "", "", date_time00, date_time00),
                        Post::new(PostId(m * 2 * 100 + d * 2 + 1), "", "", date_time12, date_time12),
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
    fn get(&self, id: PostId) -> PostsResult<Post> {
        Ok(self
            .posts
            .borrow()
            .iter()
            .find(|p| p.id == id)
            .ok_or(Error::NotFound(id))?
            .clone())
    }

    fn get_from_date<Tz: TimeZone>(
        &self,
        from: chrono::DateTime<Tz>,
        offset: usize,
        limit: usize,
    ) -> PostsResult<Vec<Post>> {
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
    ) -> PostsResult<Vec<Post>> {
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

    fn get_all(&self, offset: usize, limit: usize) -> PostsResult<Vec<Post>> {
        let mut posts = self.posts.borrow().clone().into_iter().collect::<Vec<_>>();
        posts.sort_by_key(|p| p.created_at);
        Ok(posts.into_iter().rev().skip(offset).take(limit).collect())
    }

    fn get_year_months(&self) -> PostsResult<Vec<YearMonth>> {
        Ok((1..=6).map(|m| YearMonth(2020, m * 2)).collect())
    }

    fn get_days(&self, ym: YearMonth) -> PostsResult<Vec<u8>> {
        Ok((1..=14).map(|d| d * 2 - ym.1 % 2).collect())
    }

    fn get_last_updated(&self) -> PostsResult<Option<DateTime<Utc>>> {
        let date = Local.ymd(2020i32, 12, 28);
        Ok(Some(date.and_hms(12, 0, 0).with_timezone(&Utc)))
    }

    fn create(&self, new_post: &NewPost) -> PostsResult<Post> {
        let NewPost {
            title,
            body,
            timestamp: created_at,
            ..
        } = new_post.clone();
        self.sequence.set(PostId(self.sequence.get().0 + 1));
        let post = Post::new(self.sequence.get(), title, body, created_at, created_at);
        self.posts.borrow_mut().push(post.clone());
        Ok(post)
    }

    fn update(&self, id: PostId, new_post: &NewPost) -> PostsResult<Post> {
        let NewPost {
            title,
            body,
            timestamp: updated_at,
            ..
        } = new_post.clone();
        let mut posts = self.posts.borrow_mut();
        let post = posts
            .iter_mut()
            .find(|post| post.id == id)
            .ok_or(Error::NotFound(id))?;
        post.title = title;
        post.body = body;
        post.updated_at = updated_at;
        Ok(post.clone())
    }

    fn delete(&self, id: PostId) -> PostsResult<()> {
        self.posts.borrow_mut().retain(|post| post.id != id);
        Ok(())
    }
}
