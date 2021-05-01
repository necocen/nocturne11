use anyhow::Result;
use assert_matches::assert_matches;
use chrono::{Local, TimeZone, Utc};
use domain::{
    entities::*,
    repositories::{import_posts::*, posts::*},
};
use infrastructure::posts_repository_impl::*;
use pretty_assertions::assert_eq;
mod database_mock;
use database_mock::*;

#[test]
fn insert_and_find() -> Result<()> {
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
    use diesel::result::*;
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    let post = repo.get(1);
    assert_eq!(post.unwrap_err().downcast::<Error>()?, Error::NotFound);
    Ok(())
}

#[test]
fn insert_duplicated_id() -> Result<()> {
    use diesel::result::*;
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.import(&[Post::new(1, "1", "1111", Utc::now(), Utc::now())])?;
    let result = repo.import(&[Post::new(1, "1", "1111", Utc::now(), Utc::now())]);
    assert_matches!(
        result.unwrap_err().downcast()?,
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
    );
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
