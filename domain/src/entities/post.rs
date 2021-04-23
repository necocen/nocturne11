use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Page<'a, C> {
    pub condition: &'a C,
    pub posts: Vec<Post>,
    pub per_page: usize,
    pub page: usize,
    pub prev_page: AdjacentPage<C>,
    pub next_page: AdjacentPage<C>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdjacentPage<C> {
    /// 次のページに該当するものはない
    None,
    /// 次のページ番号がある
    Page(usize),
    /// 次の条件がある
    Condition(C),
}
