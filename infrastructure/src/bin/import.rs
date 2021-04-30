#[macro_use]
extern crate log;

use anyhow::Result;
use domain::{
    entities::Post,
    repositories::{
        export_posts::ExportPostsRepository, import_posts::ImportPostsRepository,
        search::SearchRepository,
    },
};
use dotenv::dotenv;
use infrastructure::{
    legacy::posts_repository_impl::OldPostsRepositoryImpl,
    posts_repository_impl::PostsRepositoryImpl, search_repository_impl::SearchRepositoryImpl,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let old_repo = OldPostsRepositoryImpl::new(&url::Url::parse(&env::var("OLD_DATABASE_URL")?)?)?;
    let new_repo = PostsRepositoryImpl::new(&url::Url::parse(&env::var("DATABASE_URL")?)?)?;
    let search_repo = SearchRepositoryImpl::new(&url::Url::parse(&env::var("ES_URL")?)?)?;
    transport(&old_repo, &new_repo, &search_repo).await?;
    Ok(())
}

async fn transport(
    old_repo: &impl ExportPostsRepository,
    new_repo: &impl ImportPostsRepository,
    search_repo: &impl SearchRepository,
) -> Result<()> {
    let page_size = 100_usize;
    let mut old_posts: Vec<Post> = vec![];
    loop {
        let mut chunk = old_repo.get_all(old_posts.len(), page_size)?;
        let count = chunk.len();
        if count == 0 {
            break;
        }
        let first_date = chunk
            .first()
            .unwrap()
            .created_at
            .with_timezone(&chrono::Local)
            .to_rfc3339();
        let last_date = chunk
            .last()
            .unwrap()
            .created_at
            .with_timezone(&chrono::Local)
            .to_rfc3339();
        info!("Exported {} posts ({} -- {})", count, first_date, last_date);
        old_posts.append(&mut chunk);
    }

    let imported = new_repo.import(&old_posts)?;
    info!("Imported {} posts successfully.", imported.len());
    search_repo.insert_bulk(&old_posts).await?;
    info!("Indexed {} posts successfully.", imported.len());
    new_repo.reset_id_sequence()?;

    Ok(())
}
