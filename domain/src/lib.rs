pub mod entities;
pub mod repositories;
pub mod use_cases;
#[cfg(test)]
mod use_cases_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
