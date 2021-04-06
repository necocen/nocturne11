#[macro_use]
extern crate diesel;

mod models;
mod schema;
pub mod legacy;
pub mod posts_repository_impl;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
