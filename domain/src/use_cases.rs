use crate::entities::{
    date::{Year, YearMonth},
    Post,
};
use crate::repositories::posts::PostsRepository;
use anyhow::Result;

pub fn get_posts(repository: &impl PostsRepository) -> Result<Vec<Post>> {
    Ok(repository.get_all()?[..10].to_vec())
}

pub fn get_post_with_id(repository: &impl PostsRepository, id: i32) -> Result<(Post, bool)> {
    let post = repository.get(id)?;
    let posts = repository.get_from_date(post.created_at, 2)?;
    if posts.len() > 1 {
        Ok((post, true))
    } else {
        Ok((post, false))
    }
}

pub fn get_posts_with_day(
    repository: &impl PostsRepository,
    ym: YearMonth,
    day: u8,
    limit: usize,
) -> Result<Vec<Post>> {
    Ok(repository.get_all()?[..10].to_vec())
}

pub fn get_years(repository: &impl PostsRepository) -> Result<Vec<Year>> {
    let mut year_months = repository.get_year_months()?;
    year_months.sort();
    Ok(year_months
        .into_iter()
        .fold(vec![], |mut years, YearMonth(year, month)| {
            if years.is_empty() {
                years.push(Year {
                    year,
                    months: vec![month],
                });
            } else if years.last().unwrap().year == year {
                years.last_mut().unwrap().months.push(month);
            } else {
                years.push(Year {
                    year,
                    months: vec![month],
                });
            }
            years
        }))
}

pub fn get_days(repository: &impl PostsRepository, ym: YearMonth) -> Result<Vec<u8>> {
    let mut days = repository.get_days(ym)?;
    days.sort_unstable();
    Ok(days)
}

pub fn transport(
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
