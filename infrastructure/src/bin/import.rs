#[macro_use]
extern crate log;

use anyhow::Result;
use domain::repositories::{
    export_posts::ExportPostsRepository, import_posts::ImportPostsRepository,
};
use dotenv::dotenv;
use infrastructure::{
    legacy::posts_repository_impl::OldPostsRepositoryImpl,
    posts_repository_impl::PostsRepositoryImpl,
};
use std::env;

fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let old_repo = OldPostsRepositoryImpl::new(&url::Url::parse(&env::var("OLD_DATABASE_URL")?)?)?;
    let new_repo = PostsRepositoryImpl::new(&url::Url::parse(&env::var("DATABASE_URL")?)?)?;
    transport(&old_repo, &new_repo)?;
    Ok(())
}

fn transport(
    old_repository: &impl ExportPostsRepository,
    new_repository: &impl ImportPostsRepository,
) -> Result<()> {
    let mut offset = 0_usize;
    let page_size = 100_usize;
    loop {
        let old_posts = old_repository.get_all(offset, page_size)?;
        for old_post in old_posts.iter() {
            new_repository.import(old_post)?;
        }
        if !old_posts.is_empty() {
            let first_date = old_posts
                .first()
                .unwrap()
                .created_at
                .with_timezone(&chrono::Local)
                .to_rfc3339();
            let last_date = old_posts
                .last()
                .unwrap()
                .created_at
                .with_timezone(&chrono::Local)
                .to_rfc3339();
            info!(
                "Imported {} posts ({} -- {})",
                old_posts.len(),
                first_date,
                last_date,
            );
        }
        if old_posts.len() < page_size {
            break;
        }
        offset += old_posts.len();
    }
    Ok(())
}
