use anyhow::Result;
use application::adapters::PostsRepository;
use chrono::{Local, TimeZone, Utc};
use domain::entities::*;
use infrastructure::posts_repository_impl::*;
use pretty_assertions::assert_eq;
mod database_mock;
use database_mock::*;

#[tokio::test]
async fn get_by_id() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.import(&[Post::new(PostId(1), "1", "1111", Utc::now(), Utc::now())])?;
    let post = repo.get_by_id(&PostId(1)).await?.expect("post not found");
    assert_eq!(post.id.0, 1);
    assert_eq!(post.title, "1");
    assert_eq!(post.body, "1111");
    Ok(())
}

#[tokio::test]
async fn get_by_id_not_found() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    let post = repo.get_by_id(&PostId(1)).await?;
    assert!(post.is_none());
    Ok(())
}

#[tokio::test]
async fn import_update_sequence() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.import(&mock_data())?;
    repo.reset_id_sequence()?;
    let post = repo.add(NewPost::new("1230", "1230", Utc::now())).await?;
    assert_eq!(post.id.0, 1230);
    Ok(())
}

#[tokio::test]
async fn create_and_find() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.add(NewPost::new("1", "1111", Utc::now())).await?;
    let post = repo.get_by_id(&PostId(1)).await?.expect("post not found");
    assert_eq!(post.id.0, 1);
    assert_eq!(post.title, "1");
    assert_eq!(post.body, "1111");
    Ok(())
}

#[tokio::test]
async fn create_and_increment_id() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.add(NewPost::new("1", "1111", Utc::now())).await?;
    repo.add(NewPost::new("2", "2222", Utc::now())).await?;
    let post = repo.get_by_id(&PostId(2)).await?.expect("post not found");
    assert_eq!(post.id.0, 2);
    assert_eq!(post.title, "2");
    assert_eq!(post.body, "2222");
    Ok(())
}

#[tokio::test]
async fn create_and_update() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    let created_at = Local
        .with_ymd_and_hms(2021, 5, 2, 2, 10, 28)
        .unwrap()
        .with_timezone(&Utc);
    let mut post = repo.add(NewPost::new("1", "1111", created_at)).await?;
    let updated_at = Local
        .with_ymd_and_hms(2021, 5, 2, 2, 11, 24)
        .unwrap()
        .with_timezone(&Utc);
    post.title = "1'".to_string();
    post.body = "1111'".to_string();
    post.updated_at = updated_at;
    let post = repo.save(&post).await?;
    assert_eq!(post.id.0, 1);
    assert_eq!(post.title, "1'");
    assert_eq!(post.body, "1111'");
    assert_eq!(post.created_at, created_at);
    assert_eq!(post.updated_at, updated_at);
    Ok(())
}

#[tokio::test]
async fn create_and_delete() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    let created_at = Local
        .with_ymd_and_hms(2021, 5, 2, 2, 10, 28)
        .unwrap()
        .with_timezone(&Utc);
    let post = repo.add(NewPost::new("1", "1111", created_at)).await?;
    let post = repo
        .get_by_id(&PostId(post.id.0))
        .await?
        .expect("post not found");
    assert_eq!(post.id.0, 1);
    repo.remove(&PostId(1)).await?;
    let result = repo.get_by_id(&PostId(1)).await?;
    assert!(result.is_none());
    Ok(())
}
