// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::DateTime;
use chrono::offset::Utc;
use super::schema::posts;

#[derive(Queryable, Insertable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
