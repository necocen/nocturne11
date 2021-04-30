#[macro_use]
extern crate log;

use anyhow::Result;
use domain::{
    entities::Post,
    repositories::{export_posts::ExportPostsRepository, import_posts::ImportPostsRepository},
};
use dotenv::dotenv;
use infrastructure::{
    legacy::posts_repository_impl::OldPostsRepositoryImpl,
    posts_repository_impl::PostsRepositoryImpl,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
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
    let page_size = 100_usize;
    let mut old_posts: Vec<Post> = vec![];
    loop {
        let mut chunk = old_repository.get_all(old_posts.len(), page_size)?;
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

    // FIXME: 普通にバルクインサートを実装すべきという話はある
    let imported = old_posts
        .into_par_iter()
        .map(|post| new_repository.import(post))
        .collect::<Result<Vec<_>>>()?;
    new_repository.reset_id_sequence()?;
    info!("Imported {} posts successfully.", imported.len());

    Ok(())
}
