use anyhow::Result;
use domain::entities::{date::*, *};
use domain::use_cases::*;
mod posts_repository_mock;
use posts_repository_mock::*;
use pretty_assertions::assert_eq;

#[test]
fn test_get_days() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let odd_days = get_days(&repo, YearMonth(2020, 1))?;
    assert_eq!(
        odd_days,
        [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27]
    );
    let even_days = get_days(&repo, YearMonth(2020, 2))?;
    assert_eq!(
        even_days,
        [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28]
    );
    Ok(())
}

#[test]
fn test_get_years() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let months = get_years(&repo)?
        .into_iter()
        .flat_map(|y| y.months)
        .collect::<Vec<_>>();
    assert_eq!(months, [2, 4, 6, 8, 10, 12]);
    Ok(())
}

#[test]
fn test_get_post_with_id_not_found() {
    let repo = PostsRepositoryMock::new();
    let page = get_post_with_id(&repo, &9999);
    assert!(page.is_err());
}

#[test]
fn test_get_post_with_id_which_has_prev_only() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &1229)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [1229]);
    assert_eq!(prev_page, AdjacentPage::Condition(1228));
    assert_eq!(next_page, AdjacentPage::None);
    Ok(())
}

#[test]
fn test_get_post_with_id_which_has_next_only() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &202)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [202]);
    assert_eq!(prev_page, AdjacentPage::None);
    assert_eq!(next_page, AdjacentPage::Condition(203));
    Ok(())
}

#[test]
fn test_get_post_with_id_which_has_prev_and_next() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &228)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [228]);
    assert_eq!(prev_page, AdjacentPage::Condition(227));
    assert_eq!(next_page, AdjacentPage::Condition(229));
    Ok(())
}

#[test]
fn test_get_post_with_id_which_has_prev() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let Page {
        posts,
        prev_page,
        next_page,
        ..
    } = get_post_with_id(&repo, &228)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [228]);
    assert_eq!(prev_page, AdjacentPage::Condition(227));
    assert_eq!(next_page, AdjacentPage::Condition(229));
    Ok(())
}

#[test]
fn test_get_post_by_month_not_found() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 3),
        day: None,
    };
    let Page {
        posts, next_page, ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 1)?;
    assert!(posts.is_empty());
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 4),
            day: None
        })
    );
    Ok(())
}

#[test]
fn test_get_post_by_month_first_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: None,
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 1)?;
    assert_eq!(
        posts.into_iter().map(|p| p.id).collect::<Vec<_>>(),
        [202, 203, 204, 205, 206]
    );
    assert_eq!(next_page, AdjacentPage::Page(2));
    assert_eq!(prev_page, AdjacentPage::None);
    Ok(())
}

#[test]
fn test_get_post_by_month_second_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: None,
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 2)?;
    assert_eq!(
        posts.into_iter().map(|p| p.id).collect::<Vec<_>>(),
        [207, 208, 209, 210, 211]
    );
    assert_eq!(next_page, AdjacentPage::Page(3));
    assert_eq!(prev_page, AdjacentPage::Page(1));
    Ok(())
}

#[test]
fn test_get_post_by_month_last_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: None,
    };
    let Page {
        posts, next_page, ..
    } = get_posts_with_date_condition(&repo, &cond, 5, 6)?;
    assert_eq!(
        posts.into_iter().map(|p| p.id).collect::<Vec<_>>(),
        [227, 228, 229]
    );
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 4),
            day: None
        })
    );
    Ok(())
}

#[test]
fn test_get_post_by_day_not_found() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 3),
        day: Some(1),
    };
    let Page {
        posts, next_page, ..
    } = get_posts_with_date_condition(&repo, &cond, 1, 1)?;
    assert!(posts.is_empty());
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 3),
            day: Some(3)
        })
    );
    Ok(())
}

#[test]
fn test_get_post_by_day_first_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: Some(1),
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 1, 1)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [202]);
    assert_eq!(next_page, AdjacentPage::Page(2));
    assert_eq!(prev_page, AdjacentPage::None);
    Ok(())
}

#[test]
fn test_get_post_by_day_last_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let cond = DateCondition {
        ym: YearMonth(2020, 2),
        day: Some(1),
    };
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts_with_date_condition(&repo, &cond, 1, 2)?;
    assert_eq!(posts.into_iter().map(|p| p.id).collect::<Vec<_>>(), [203]);
    assert_eq!(prev_page, AdjacentPage::Page(1));
    assert_eq!(
        next_page,
        AdjacentPage::<DateCondition>::Condition(DateCondition {
            ym: YearMonth(2020, 2),
            day: Some(2)
        })
    );
    Ok(())
}

#[test]
fn test_get_posts_first_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts(&repo, 10, 1)?;
    assert_eq!(posts.len(), 10);
    assert_eq!(prev_page, AdjacentPage::None);
    assert_eq!(next_page, AdjacentPage::Page(2));
    Ok(())
}

#[test]
fn test_get_posts_second_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts(&repo, 10, 2)?;
    assert_eq!(posts.len(), 10);
    assert_eq!(prev_page, AdjacentPage::Condition(()));
    assert_eq!(next_page, AdjacentPage::Page(3));
    Ok(())
}

#[test]
fn test_get_posts_last_page() -> Result<()> {
    let repo = PostsRepositoryMock::new();
    let Page {
        posts,
        next_page,
        prev_page,
        ..
    } = get_posts(&repo, 10, 17)?;
    assert_eq!(posts.len(), 8);
    assert_eq!(prev_page, AdjacentPage::Page(16));
    assert_eq!(next_page, AdjacentPage::None);
    Ok(())
}
