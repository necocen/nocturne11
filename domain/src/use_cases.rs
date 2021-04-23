use crate::entities::{
    date::{DateCondition, Year, YearMonth},
    NextPage, Page, Post,
};
use crate::repositories::posts::PostsRepository;
use anyhow::{Context, Result};
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

pub fn get_posts_with_day<'a>(
    repository: &impl PostsRepository,
    condition: &'a DateCondition,
    per_page: usize,
    page: usize,
) -> Result<Page<'a, DateCondition>> {
    let from_date = Local
        .ymd(
            condition.ym.0.into(),
            condition.ym.1.into(),
            condition.day.unwrap_or(1).into(),
        )
        .and_hms(0, 0, 0);
    let to_date = if condition.day.is_some() {
        from_date + Duration::days(1)
    } else {
        let (next_year, next_month) = if from_date.month() == 12 {
            (from_date.year() + 1, 1)
        } else {
            (from_date.year(), from_date.month() + 1)
        };
        Local.ymd(next_year, next_month, 1).and_hms(0, 0, 0)
    };
    let posts = repository
        .get_from_date(from_date, per_page * (page - 1), per_page + 1)?
        .into_iter()
        .filter(|post| post.created_at.with_timezone(&Local) < to_date)
        .collect::<Vec<_>>();

    if posts.len() > per_page {
        // 残りがある場合は次のページがある
        Ok(Page {
            condition,
            posts: posts.into_iter().take(per_page).collect(),
            per_page,
            page,
            prev_page: if page <= 1 { None } else { Some(page - 1) },
            next_page: NextPage::Page(page + 1),
        })
    } else {
        // 次のページがない場合、次の区間を探す
        // 次の月（日付単位の場合は月末でなければ使わない）
        let next_ym = repository
            .get_year_months()?
            .into_iter()
            .filter(|ym| *ym > condition.ym)
            .min();

        let next_condition = if let Some(day) = condition.day {
            // 日単位の場合、その月の次の日を探す
            let next_day = repository
                .get_days(condition.ym)?
                .into_iter()
                .filter(|d| *d > day)
                .min();
            if next_day.is_some() {
                Some(DateCondition {
                    ym: condition.ym,
                    day: next_day,
                })
            } else if let Some(ym) = next_ym {
                // 次の日がない場合は次の月の最初の日を返す
                let day = repository
                    .get_days(ym)?
                    .into_iter()
                    .min()
                    .context("Logic Error")?;
                Some(DateCondition { ym, day: Some(day) })
            } else {
                // 次の月もない場合はNone
                None
            }
        } else {
            next_ym.map(|ym| DateCondition { ym, day: None })
        };

        Ok(Page {
            condition,
            posts: posts.into_iter().take(per_page).collect(),
            per_page,
            page,
            prev_page: if page <= 1 { None } else { Some(page - 1) },
            next_page: next_condition.map_or(NextPage::None, NextPage::Condition),
        })
    }
}

pub fn get_years(repository: &impl PostsRepository) -> Result<Vec<Year>> {
    let mut year_months = repository.get_year_months()?;
    year_months.sort();
    let years = year_months
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
        });
    Ok(years)
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
