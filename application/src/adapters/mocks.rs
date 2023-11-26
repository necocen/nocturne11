mod posts_repository_mock;
mod search_client_mock;

use domain::entities::{Post, PostId};
pub use posts_repository_mock::PostsRepositoryMock;
pub use search_client_mock::SearchClientMock;

pub fn mock_post_data() -> Vec<Post> {
    use chrono::{Local, NaiveDate, Utc};
    (1..=6)
        .flat_map(|m| {
            (1..=14).flat_map(move |d| {
                let date = NaiveDate::from_ymd_opt(2020i32, (m * 2) as u32, (d * 2 - m % 2) as u32)
                    .unwrap();
                let date_time00 = date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .with_timezone(&Utc);
                let date_time12 = date
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .with_timezone(&Utc);
                vec![
                    Post::new(
                        PostId(m * 2 * 100 + d * 2),
                        "",
                        "",
                        date_time00,
                        date_time00,
                    ),
                    Post::new(
                        PostId(m * 2 * 100 + d * 2 + 1),
                        "",
                        "",
                        date_time12,
                        date_time12,
                    ),
                ]
            })
        })
        .collect()
}
