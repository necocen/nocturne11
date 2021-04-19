#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod diesel_helpers;
pub mod legacy;
mod models;
pub mod posts_repository_impl;
mod schema;
pub mod search_repository_impl;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
