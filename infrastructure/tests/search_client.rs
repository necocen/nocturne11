use std::env;

use anyhow::Result;
use application::adapters::SearchClient as _;
use domain::entities::*;
use infrastructure::{posts_repository_impl::*, search_client::*};
use pretty_assertions::assert_eq;
mod database_mock;
use database_mock::*;

#[tokio::test]
async fn get_latest_posts() -> Result<()> {
    let DatabaseMock { ref pg_url, .. } = mock_db()?;
    let es_url = url::Url::parse(&env::var("ES_URL")?)?;
    let posts = PostsRepositoryImpl::new(pg_url)?;
    let client = SearchClient::with_es_index_name(&es_url, pg_url, "test_import_and_find")?;
    let mock_data = mock_data();
    for post in mock_data.iter() {
        client.save(post).await?;
    }
    posts.import(&mock_data)?;

    let post_ids = client.get_latest_posts(0, 1000).await?.post_ids;
    let expected_ids = (1..=6)
        .rev()
        .flat_map(|m| {
            (1..=14).rev().flat_map(move |d| {
                vec![PostId(m * 2 * 100 + d * 2 + 1), PostId(m * 2 * 100 + d * 2)]
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(post_ids, expected_ids);
    Ok(())
}
