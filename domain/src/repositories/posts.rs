use crate::entities::{date::YearMonth, Post};
use anyhow::Result;

pub trait PostsRepository {
    fn get(&self, id: i32) -> Result<Post>;

    fn get_all(&self) -> Result<Vec<Post>>;

    fn get_year_months(&self) -> Result<Vec<YearMonth>>;

    fn insert(&self, post: &Post) -> Result<Post>;
}
