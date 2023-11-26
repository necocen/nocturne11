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
