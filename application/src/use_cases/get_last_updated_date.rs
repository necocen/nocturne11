use chrono::{DateTime, Utc};

use crate::adapters::SearchClient;

pub struct GetLastUpdatedDateUseCase;

impl GetLastUpdatedDateUseCase {
    pub async fn execute(
        search_client: &impl SearchClient,
    ) -> anyhow::Result<Option<DateTime<Utc>>> {
        search_client.get_last_updated().await
    }
}
