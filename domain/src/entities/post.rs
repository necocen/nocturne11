use core::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct PostId(pub i32);

impl fmt::Display for PostId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(
        id: PostId,
        title: impl Into<String>,
        body: impl Into<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Post {
        Post {
            id,
            title: title.into(),
            body: body.into().replace("\r\n", "\n").replace('\r', "\n"),
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

impl NewPost {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        timestamp: DateTime<Utc>,
    ) -> NewPost {
        NewPost {
            title: title.into(),
            body: body.into().replace("\r\n", "\n").replace('\r', "\n"),
            timestamp,
        }
    }
}

pub trait Condition {
    type Page;
}

impl Condition for PostId {
    type Page = usize;
}

impl Condition for () {
    type Page = usize;
}

#[derive(Debug, Clone)]
pub struct Page<'a, C: Condition> {
    pub condition: &'a C,
    pub posts: Vec<Post>,
    pub per_page: usize,
    pub page: C::Page,
    pub prev_page: AdjacentPage<C>,
    pub next_page: AdjacentPage<C>,
}

impl<'a> Page<'a, PostId> {
    pub fn post(&self) -> Option<&Post> {
        self.posts.first()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdjacentPage<C: Condition> {
    /// 次のページに該当するものはない
    None,
    /// 次のページ番号がある
    Page(C::Page),
    /// 次の条件がある
    Condition(C),
}
