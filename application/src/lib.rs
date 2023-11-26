pub mod adapters;
pub mod errors;
pub mod models;
pub mod use_cases;

type ApplicationResult<T> = Result<T, errors::ApplicationError>;
