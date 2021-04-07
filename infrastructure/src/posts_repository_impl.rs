use crate::models::Post as PostModel;
use anyhow::Result;
use diesel::prelude::*;
use domain::entities::Post;
use domain::repositories::posts::PostsRepository;

pub struct PostsRepositoryImpl {
    connection: PgConnection,
}

impl PostsRepositoryImpl {
    pub fn new(database_url: String) -> PostsRepositoryImpl {
        let connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        PostsRepositoryImpl { connection }
    }
}

impl PostsRepository for PostsRepositoryImpl {
    fn get(&self, id: i32) -> Result<Post> {
        use crate::schema::posts::dsl::{id as post_id, posts};
        let post = posts
            .filter(post_id.eq(id))
            .first::<PostModel>(&self.connection)?;
        Ok(Post {
            id: post.id,
            title: post.title,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }

    fn insert(&self, post: &Post) -> Result<Post> {
        use crate::schema::posts;
        let post: PostModel = diesel::insert_into(posts::table)
            .values(&PostModel {
                id: post.id,
                title: post.title.clone(),
                body: post.body.clone(),
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
            .get_result(&self.connection)?;
        Ok(Post {
            id: post.id,
            title: post.title,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }
}
