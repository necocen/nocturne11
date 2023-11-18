use crate::{
    entities::{
        config::Config,
        date::{DateCondition, Year, YearMonth},
        AdjacentPage, KeywordsCondition, NewPost, Page, Post, PostId,
    },
    repositories::{
        config::ConfigRepository, google_auth_cert::GoogleAuthCertRepository,
        posts::Error as PostsError, posts::PostsRepository, search::SearchRepository,
    },
    Result,
};
use anyhow::Context;
use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Utc};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

pub fn get_posts(
    repository: &impl PostsRepository,
    per_page: usize,
    page: usize,
) -> Result<Page<'static, ()>> {
    let posts = repository.get_all(per_page * (page - 1), per_page + 1)?;
    let next_page = if posts.len() > per_page {
        AdjacentPage::Page(page + 1)
    } else {
        AdjacentPage::None
    };
    Ok(Page {
        condition: &(),
        posts: posts.into_iter().take(per_page).collect(),
        per_page,
        page,
        prev_page: match page {
            1 => AdjacentPage::None,
            2 => AdjacentPage::Condition(()),
            _ => AdjacentPage::Page(page - 1),
        },
        next_page,
    })
}

pub async fn search_posts<'a>(
    posts_repository: &impl PostsRepository,
    search_repository: &impl SearchRepository,
    keywords: &'a KeywordsCondition<'a>,
    per_page: usize,
    search_after: Option<(u64, u64)>,
) -> Result<Page<'a, KeywordsCondition<'a>>> {
    let search_result = search_repository
        .search(&keywords.0, search_after, per_page + 1)
        .await?;
    let posts = search_result
        .posts
        .iter()
        .filter_map(|p| match posts_repository.get(p.id) {
            Ok(post) => Some(Ok(post)),
            Err(PostsError::NotFound(_)) => None,
            Err(e) => Some(Err(e)),
        })
        .map(|r| Ok(r?))
        .collect::<Result<Vec<Post>>>()?;
    let next_page = if posts.len() > per_page {
        AdjacentPage::<KeywordsCondition<'_>>::Page(search_result.search_after)
    } else {
        AdjacentPage::None
    };
    Ok(Page {
        condition: keywords,
        posts: posts.into_iter().take(per_page).collect(),
        per_page,
        page: search_after,
        next_page,
        prev_page: AdjacentPage::None,
    })
}

pub fn get_post_with_id<'a>(
    repository: &impl PostsRepository,
    id: &'a PostId,
) -> Result<Page<'a, PostId>> {
    let post = repository.get(*id)?;
    let prev_post = repository
        .get_until_date(post.created_at, 0, 1)?
        .first()
        .cloned();
    let next_post = repository
        .get_from_date(post.created_at, 1, 1)?
        .first()
        .cloned();
    Ok(Page {
        condition: id,
        posts: vec![post],
        per_page: 1,
        page: 1,
        next_page: next_post
            .map(|p| p.id)
            .map_or(AdjacentPage::None, AdjacentPage::Condition),
        prev_page: prev_post
            .map(|p| p.id)
            .map_or(AdjacentPage::None, AdjacentPage::Condition),
    })
}

pub fn get_posts_with_date_condition<'a>(
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
            prev_page: match page {
                1 => AdjacentPage::None,
                2 => AdjacentPage::Condition(condition.clone()),
                _ => AdjacentPage::Page(page - 1),
            },
            next_page: AdjacentPage::Page(page + 1),
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
            prev_page: if page <= 1 {
                AdjacentPage::None
            } else {
                AdjacentPage::Page(page - 1)
            },
            next_page: next_condition.map_or(AdjacentPage::None, AdjacentPage::Condition),
        })
    }
}

pub fn get_last_updated_date(repository: &impl PostsRepository) -> Result<Option<DateTime<Utc>>> {
    Ok(repository.get_last_updated()?)
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

pub async fn create_post(
    posts_repository: &impl PostsRepository,
    search_repository: &impl SearchRepository,
    new_post: &NewPost,
) -> Result<Post> {
    let post = posts_repository.create(new_post)?;
    search_repository.insert(&post).await?;
    Ok(post)
}

pub async fn update_post(
    posts_repository: &impl PostsRepository,
    search_repository: &impl SearchRepository,
    id: PostId,
    new_post: &NewPost,
) -> Result<Post> {
    let post = posts_repository.update(id, new_post)?;
    search_repository.update(&post).await?;
    Ok(post)
}

pub async fn delete_post(
    posts_repository: &impl PostsRepository,
    search_repository: &impl SearchRepository,
    id: PostId,
) -> Result<()> {
    posts_repository.delete(id)?;
    search_repository.delete(id).await?;
    Ok(())
}

pub fn get_config(config_repository: &impl ConfigRepository) -> Result<Config> {
    Ok(config_repository.get()?)
}

pub async fn check_login(
    config_repository: &impl ConfigRepository,
    cert_repository: &impl GoogleAuthCertRepository,
    jwt: &str,
) -> Result<String> {
    const ISSUERS: [&str; 2] = ["accounts.google.com", "https://accounts.google.com"];
    let header = decode_header(jwt)?;
    let kid = header
        .kid
        .context("JWT token does not contain 'kid' field.")?;
    let config = config_repository.get()?;
    let (n, e) = cert_repository.key(&kid).await?;
    let key = DecodingKey::from_rsa_components(&n, &e)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.sub = Some(config.auth.admin_user_id);
    validation.set_audience([config.auth.google_client_id].as_ref());
    validation.set_issuer(&ISSUERS);

    #[derive(Debug, Deserialize)]
    struct Claims {
        sub: String,
    }
    let data = decode::<Claims>(jwt, &key, &validation)?;

    // IDを返す
    Ok(data.claims.sub)
}
