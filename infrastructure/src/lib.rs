#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("migrations/");

pub mod config_repository_impl;
pub mod config_repository_mock_impl;
mod diesel_helpers;
pub mod legacy;
pub mod migration;
mod models;
pub mod pg_repository_impls;
pub mod posts_repository_impl;
mod schema;
pub mod search_repository_impl;
