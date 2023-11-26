use domain::entities::{Post, PostId};

use crate::{errors::ApplicationError, ApplicationResult};

#[derive(Debug)]
pub struct Page<'a, C, I> {
    pub condition: &'a C,
    pub index: I,
    pub posts: Vec<Post>,
    pub next_page: Option<AdjacentPageInfo<C, I>>,
    pub prev_page: Option<AdjacentPageInfo<C, I>>,
}

impl Page<'_, PostId, ()> {
    pub fn post(self) -> ApplicationResult<Post> {
        self.posts
            .into_iter()
            .next()
            .ok_or(ApplicationError::PostNotFound)
    }
}

#[derive(Debug, PartialEq)]
pub enum AdjacentPageInfo<C, I> {
    Condition(C),
    PageIndex(I),
}
