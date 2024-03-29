use crate::{adapters::SearchClient, models::YearMonth, ApplicationResult};

pub struct GetDaysInYearMonthUseCase;

impl GetDaysInYearMonthUseCase {
    pub async fn execute(
        search_client: &impl SearchClient,
        ym: &YearMonth,
    ) -> ApplicationResult<Vec<u8>> {
        Ok(search_client.get_days_in_year_month(ym).await?)
    }
}
