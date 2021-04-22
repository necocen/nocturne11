use crate::entities::{
    date::{Year, YearMonth},
    Page, Post,
};
use crate::repositories::posts::PostsRepository;
use anyhow::Result;
use chrono::{Datelike, Duration, Local, TimeZone};

pub fn get_posts(repository: &impl PostsRepository) -> Result<Vec<Post>> {
    Ok(repository.get_all()?[..10].to_vec())
}

pub fn get_post_with_id(repository: &impl PostsRepository, id: i32) -> Result<(Post, bool)> {
    let post = repository.get(id)?;
    let posts = repository.get_from_date(post.created_at, 0, 2)?;
    if posts.len() > 1 {
        Ok((post, true))
    } else {
        Ok((post, false))
    }
}

pub fn get_posts_with_day(
    repository: &impl PostsRepository,
    ym: YearMonth,
    day: Option<u8>,
    per_page: usize,
    page: usize,
) -> Result<Page> {
    let from_date = Local
        .ymd(ym.0 as i32, ym.1 as u32, day.unwrap_or(1) as u32)
        .and_hms(0, 0, 0);
    let to_date = if day.is_some() {
        from_date + Duration::days(1)
    } else {
        let (next_year, next_month) = if from_date.month() == 12 {
            (from_date.year() + 1, 1)
        } else {
            (from_date.year(), from_date.month() + 1)
        };
        Local
            .ymd(next_year.into(), next_month.into(), 1)
            .and_hms(0, 0, 0)
    };
    let posts = repository
        .get_from_date(from_date, per_page * (page - 1), per_page + 1)?
        .into_iter()
        .filter(|post| post.created_at.with_timezone(&Local) < to_date)
        .take(per_page)
        .collect::<Vec<_>>();
    if posts.len() > per_page {
        // 残りがある場合は次のページがある
        Ok(Page {
            posts,
            per_page,
            page,
            prev_page: if page == 0 { None } else { Some(page - 1) },
            next_page: Some(page + 1),
        })
    } else {
        Ok(Page {
            posts,
            per_page,
            page,
            prev_page: if page == 0 { None } else { Some(page - 1) },
            next_page: None,
        })
    }
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
