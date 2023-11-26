use domain::entities::PostId;

#[derive(Debug)]
pub struct SearchResult {
    pub post_ids: Vec<PostId>,
    pub total_count: usize,
}
