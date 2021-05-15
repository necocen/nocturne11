pub trait SearchCondition {
    fn subtitle(&self) -> String;
}

impl SearchCondition for () {
    fn subtitle(&self) -> String {
        "".to_owned()
    }
}
