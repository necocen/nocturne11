use domain::entities::{
    date::{DateCondition, YearMonth},
    PostId,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct IdArguments {
    pub id: PostId,
}
#[derive(Debug, Clone, Deserialize)]
pub(super) struct DateArguments {
    year: u16,
    month: u8,
    day: Option<u8>,
}

impl From<DateArguments> for DateCondition {
    fn from(args: DateArguments) -> DateCondition {
        DateCondition {
            ym: YearMonth(args.year, args.month),
            day: args.day,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct PageQuery {
    pub page: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct CreateFormParams {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct UpdateFormParams {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct DeleteFormParams {
    pub id: i32,
}
