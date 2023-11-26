use domain::entities::{NewPost, Post, PostId};

#[async_trait::async_trait]
pub trait PostsRepository {
    async fn get_by_id(&self, id: &PostId) -> anyhow::Result<Option<Post>>;
    async fn get_by_ids(&self, ids: &[PostId]) -> anyhow::Result<Vec<Post>>;
    async fn add(&self, new_post: NewPost) -> anyhow::Result<Post>;
    async fn save(&self, post: &Post) -> anyhow::Result<Post>;
    async fn remove(&self, id: &PostId) -> anyhow::Result<()>;
}
