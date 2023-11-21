use chrono::Utc;
use domain::entities::*;

#[test]
fn post_replace_linebreaks() {
    let post = Post::new(
        PostId(1),
        "TITLE",
        "LINE\r\nLINE\rLINE\nLINE\r\r\nLINE\n\r\nLINE",
        Utc::now(),
        Utc::now(),
    );
    assert_eq!(post.body, "LINE\nLINE\nLINE\nLINE\n\nLINE\n\nLINE");
}

#[test]
fn new_post_replace_linebreaks() {
    let new_post = NewPost::new(
        "TITLE",
        "LINE\r\nLINE\rLINE\nLINE\r\r\nLINE\n\r\nLINE",
        Utc::now(),
    );
    assert_eq!(new_post.body, "LINE\nLINE\nLINE\nLINE\n\nLINE\n\nLINE");
}
