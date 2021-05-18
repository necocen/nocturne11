use anyhow::Result;
use assert_matches::assert_matches;
use chrono::{Local, TimeZone, Utc};
use domain::{
    entities::*,
    repositories::{import_posts::*, posts::Error, posts::*},
};
use infrastructure::posts_repository_impl::*;
use pretty_assertions::assert_eq;
mod database_mock;
use database_mock::*;

#[test]
fn import_and_find() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.import(&[Post::new(1, "1", "1111", Utc::now(), Utc::now())])?;
    let post = repo.get(1)?;
    assert_eq!(post.id, 1);
    assert_eq!(post.title, "1");
    assert_eq!(post.body, "1111");
    Ok(())
}

#[test]
fn id_not_found() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    let post = repo.get(1);
    assert_matches!(post, Err(Error::NotFound(_)));
    Ok(())
}

#[test]
fn import_duplicated_id() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.import(&[Post::new(1, "1", "1111", Utc::now(), Utc::now())])?;
    let result = repo.import(&[Post::new(1, "1", "1111", Utc::now(), Utc::now())]);
    assert_matches!(result, Err(_));
    Ok(())
}

#[test]
fn import_update_sequence() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.import(&mock_data())?;
    repo.reset_id_sequence()?;
    let post = repo.create(&NewPost::new("1230", "1230", Utc::now()))?;
    assert_eq!(post.id, 1230);
    Ok(())
}

#[test]
fn create_and_find() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.create(&NewPost::new("1", "1111", Utc::now()))?;
    let post = repo.get(1)?;
    assert_eq!(post.id, 1);
    assert_eq!(post.title, "1");
    assert_eq!(post.body, "1111");
    Ok(())
}

#[test]
fn create_and_increment_id() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.create(&NewPost::new("1", "1111", Utc::now()))?;
    repo.create(&NewPost::new("2", "2222", Utc::now()))?;
    let post = repo.get(2)?;
    assert_eq!(post.id, 2);
    assert_eq!(post.title, "2");
    assert_eq!(post.body, "2222");
    Ok(())
}

#[test]
fn create_and_update() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    let created_at = Local.ymd(2021, 5, 2).and_hms(2, 10, 28).with_timezone(&Utc);
    let post = repo.create(&NewPost::new("1", "1111", created_at))?;
    let updated_at = Local.ymd(2021, 5, 2).and_hms(2, 11, 24).with_timezone(&Utc);
    let post = repo.update(post.id, &NewPost::new("1'", "1111'", updated_at))?;
    assert_eq!(post.id, 1);
    assert_eq!(post.title, "1'");
    assert_eq!(post.body, "1111'");
    assert_eq!(post.created_at, created_at);
    assert_eq!(post.updated_at, updated_at);
    Ok(())
}

#[test]
fn create_and_delete() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    let created_at = Local.ymd(2021, 5, 2).and_hms(2, 10, 28).with_timezone(&Utc);
    let post = repo.create(&NewPost::new("1", "1111", created_at))?;
    let post = repo.get(post.id)?;
    assert_eq!(post.id, 1);
    repo.delete(1)?;
    let result = repo.get(post.id);
    assert_matches!(result, Err(Error::NotFound(_)));
    Ok(())
}

#[test]
fn find_all() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.import(&mock_data())?;
    let posts = repo.get_all(0, 1000)?;
    let ids = posts.into_iter().map(|p| p.id).collect::<Vec<_>>();
    // get_all()は日付降順なので逆向き
    let expected_ids = (1..=6)
        .rev()
        .flat_map(|m| {
            (1..=14)
                .rev()
                .flat_map(move |d| vec![m * 2 * 100 + d * 2 + 1, m * 2 * 100 + d * 2])
        })
        .collect::<Vec<_>>();
    assert_eq!(ids, expected_ids);
    Ok(())
}

fn mock_data() -> Vec<Post> {
    (1..=6)
        .flat_map(|m| {
            (1..=14).flat_map(move |d| {
                let date = Local.ymd(2020i32, (m * 2) as u32, (d * 2 - m % 2) as u32);
                let date_time00 = date.and_hms(0, 0, 0).with_timezone(&Utc);
                let date_time12 = date.and_hms(12, 0, 0).with_timezone(&Utc);
                vec![
                    Post::new(m * 2 * 100 + d * 2, "", "", date_time00, date_time00),
                    Post::new(m * 2 * 100 + d * 2 + 1, "", "", date_time12, date_time12),
                ]
            })
        })
        .collect()
}
