use super::Post;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResult {
    pub posts: Vec<Post>,
    pub total_count: u64,
    pub search_after: Option<(u64, u64)>,
}
