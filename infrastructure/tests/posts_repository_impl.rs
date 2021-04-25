use anyhow::Result;
use chrono::Utc;
use domain::entities::*;
use domain::repositories::posts::*;
use function_name::*;
use infrastructure::posts_repository_impl::*;
mod test_database;
use test_database::*;

#[named]
#[test]
fn test_insert_and_find() -> Result<()> {
    let TestDatabase { ref pg_url, .. } = test_db(function_name!())?;
    let repo = PostsRepositoryImpl::new(pg_url)?;
    repo.insert(&Post {
        id: 1,
        title: "1".to_string(),
        body: "1111".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })?;
    let post = repo.get(1)?;
    assert_eq!(post.id, 1);
    assert_eq!(post.title, "1");
    assert_eq!(post.body, "1111");
    Ok(())
}
