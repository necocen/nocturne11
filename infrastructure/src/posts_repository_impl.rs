use domain::repositories::posts::PostsRepository;
use diesel::prelude::*;
use crate::models::Post as PostModel;
use domain::entities::Post;
use anyhow::Result;

pub struct PostsRepositoryImpl {
    connection: PgConnection,
}

impl PostsRepositoryImpl {
    pub fn new(database_url: String) -> PostsRepositoryImpl {
        let connection = PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        PostsRepositoryImpl {
            connection
        }
    }
}

impl PostsRepository for PostsRepositoryImpl {
    fn get(&self, id: i32) -> Result<Post> {
        use crate::schema::posts::dsl::{posts, id as post_id};
        let post = posts.filter(post_id.eq(id)).first::<PostModel>(&self.connection)?;
        Ok(Post {
            id: post.id,
            title: post.title,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }

    fn insert(&self, post: Post) -> Result<()> {
        use crate::schema::posts;
        let _: PostModel = diesel::insert_into(posts::table).values(&PostModel {
            id: post.id,
            title: post.title,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }).get_result(&self.connection)?;
        Ok(())
    }
}
