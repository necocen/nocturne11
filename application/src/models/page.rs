use std::fmt;

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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
#[non_exhaustive]
pub struct PageNumber(pub usize);

impl fmt::Display for PageNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for PageNumber {
    fn default() -> Self {
        Self(1)
    }
}

impl PageNumber {
    pub fn new(page_number: usize) -> ApplicationResult<Self> {
        if page_number == 0 {
            return Err(ApplicationError::InvalidPageNumber);
        }
        Ok(Self(page_number))
    }

    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn prev(self) -> ApplicationResult<Self> {
        if self.0 == 1 {
            return Err(ApplicationError::InvalidPageNumber);
        }
        Ok(Self(self.0 - 1))
    }
}

impl From<PageNumber> for usize {
    fn from(page_number: PageNumber) -> Self {
        page_number.0
    }
}

impl TryFrom<usize> for PageNumber {
    type Error = ApplicationError;

    fn try_from(page_number: usize) -> ApplicationResult<Self> {
        Self::new(page_number)
    }
}
