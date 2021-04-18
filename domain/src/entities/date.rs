use serde::Serialize;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth(pub u16, pub u8);

#[derive(Debug, Clone, Serialize)]
pub struct Year {
    pub year: u16,
    pub months: Vec<u8>,
}
