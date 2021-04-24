use anyhow::Result;
use domain::repositories::posts::PostsRepository;
use infrastructure::{
    legacy::posts_repository_impl::OldPostsRepositoryImpl,
    posts_repository_impl::PostsRepositoryImpl,
};

fn main() -> Result<()> {
    env_logger::init();
    let old_repo =
        OldPostsRepositoryImpl::new(&url::Url::parse("mysql://root:password@127.0.0.1/andante")?)?;
    let new_repo = PostsRepositoryImpl::new(&url::Url::parse(
        "postgres://root:password@127.0.0.1/andante",
    )?)?;
    transport(&old_repo, &new_repo)?;
    Ok(())
}

fn transport(
    old_repository: &impl PostsRepository,
    new_repository: &impl PostsRepository,
) -> Result<()> {
    let mut old_posts = old_repository.get_all()?;
    old_posts.sort_by_key(|p| p.id);
    for post in old_posts.into_iter() {
        new_repository.insert(&post)?;
    }
    Ok(())
}
