use crate::entities::{
    date::{Year, YearMonth},
    Post,
};
use crate::repositories::posts::PostsRepository;
use anyhow::Result;

pub fn get_posts(repository: &impl PostsRepository) -> Result<Vec<Post>> {
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
            } else {
                if years.last().unwrap().year == year {
                    years.last_mut().unwrap().months.push(month);
                } else {
                    years.push(Year {
                        year,
                        months: vec![month],
                    });
                }
            }
            years
        }))
}

pub fn get_days(repository: &impl PostsRepository, ym: YearMonth) -> Result<Vec<u8>> {
    todo!()
}
