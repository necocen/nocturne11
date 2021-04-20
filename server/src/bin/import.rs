use anyhow::Result;
use domain::use_cases::transport;
use infrastructure::{
    legacy::posts_repository_impl::OldPostsRepositoryImpl,
    posts_repository_impl::PostsRepositoryImpl,
};

fn main() -> Result<()> {
    env_logger::init();
    let old_repo =
        OldPostsRepositoryImpl::new(url::Url::parse("mysql://root:password@127.0.0.1/andante")?)?;
    let new_repo = PostsRepositoryImpl::new(url::Url::parse(
        "postgres://root:password@127.0.0.1/andante",
    )?)?;
    transport(&old_repo, &new_repo)?;
    Ok(())
}
