table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        text -> Mediumtext,
        is_commentable -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
