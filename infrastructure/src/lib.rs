#[macro_use]
extern crate diesel;

mod old_models;
mod old_schema;
mod models;
mod schema;
pub mod old_posts_repository_impl;
pub mod posts_repository_impl;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
