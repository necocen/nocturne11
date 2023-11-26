use crate::{adapters::SearchClient, models::YearMonth, ApplicationResult};

pub struct GetYearMonthsUseCase;

impl GetYearMonthsUseCase {
    pub async fn execute(search_client: &impl SearchClient) -> ApplicationResult<Vec<YearMonth>> {
        Ok(search_client.get_year_months().await?)
    }
}
