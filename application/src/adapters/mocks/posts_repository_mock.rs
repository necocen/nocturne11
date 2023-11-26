use std::sync::{Arc, RwLock};

use domain::entities::{NewPost, Post, PostId};

use crate::adapters::PostsRepository;

pub struct PostsRepositoryMock {
    pub posts: Arc<RwLock<Vec<Post>>>,
    pub sequence: Arc<RwLock<PostId>>,
}

impl PostsRepositoryMock {
    pub fn new(mut posts: Vec<Post>) -> PostsRepositoryMock {
        posts.sort_by_key(|post| post.created_at);
        let sequence = posts.last().map(|post| post.id).unwrap_or(PostId(0));
        PostsRepositoryMock {
            posts: Arc::new(RwLock::new(posts)),
            sequence: Arc::new(RwLock::new(sequence)),
        }
    }
}

#[async_trait::async_trait]
impl PostsRepository for PostsRepositoryMock {
    async fn get_by_id(&self, id: &PostId) -> anyhow::Result<Option<Post>> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let post = posts.iter().find(|p| p.id == *id).cloned();
        Ok(post)
    }

    async fn get_by_ids(&self, ids: &[PostId]) -> anyhow::Result<Vec<Post>> {
        let posts = self
            .posts
            .read()
            .map_err(|_| anyhow::anyhow!("RwLock read error"))?;
        let posts = posts
            .iter()
            .filter(|p| ids.contains(&p.id))
            .cloned()
            .collect::<Vec<_>>();
        Ok(posts)
    }

    async fn add(&self, new_post: NewPost) -> anyhow::Result<Post> {
        let mut sequence = self
            .sequence
            .write()
            .map_err(|_| anyhow::anyhow!("RwLock write error"))?;
        let post = Post::new(
            *sequence,
            new_post.title,
            new_post.body,
            new_post.timestamp,
            new_post.timestamp,
        );
        *sequence = PostId(sequence.0 + 1);
        let mut posts = self
            .posts
            .write()
            .map_err(|_| anyhow::anyhow!("RwLock write error"))?;
        posts.push(post.clone());
        Ok(post)
    }

    async fn save(&self, post: &Post) -> anyhow::Result<Post> {
        let mut posts = self
            .posts
            .write()
            .map_err(|_| anyhow::anyhow!("RwLock write error"))?;
        let index = posts
            .iter()
            .position(|p| p.id == post.id)
            .ok_or(anyhow::anyhow!("post not found"))?;
        posts[index] = post.clone();
        posts.sort_by_key(|post| post.created_at);
        Ok(post.clone())
    }

    async fn remove(&self, id: &PostId) -> anyhow::Result<()> {
        let mut posts = self
            .posts
            .write()
            .map_err(|_| anyhow::anyhow!("RwLock write error"))?;
        let index = posts
            .iter()
            .position(|p| p.id == *id)
            .ok_or(anyhow::anyhow!("post not found"))?;
        posts.remove(index);
        Ok(())
    }
}
