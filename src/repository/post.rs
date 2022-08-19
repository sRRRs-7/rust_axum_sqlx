use crate::db::Db;
use crate::models::post::{Post, PostList, NewPost, PostId};
use crate::error::Result;
use anyhow::Context;
use async_trait::async_trait;
use mockall::automock;

pub struct PostRepo {
    pool: Db,
}
impl PostRepo {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[automock]
#[async_trait]
pub trait PostRepoTrait {
    async fn find_all(&self) -> Result<PostList>;
    async fn find_by_id(&self, post_id: i32) -> Result<Post>;
    async fn add(&self, body: &NewPost) -> Result<PostId>;
}

#[async_trait]
impl PostRepoTrait for PostRepo {
    async fn find_all(&self) -> Result<PostList> {
        let row = sqlx::query_as::<_, Post>("SELECT * FROM posts")
            .fetch_all(&*self.pool)
            .await
            .context("DB Query Error (find all posts)")?;

        Ok(row)
    }

    async fn find_by_id(&self, post_id: i32) -> Result<Post> {
        let row = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE post_id = $1")
            .bind(post_id)
            .fetch_one(&*self.pool)
            .await
            .context("DB Query Error (find one posts)")?;

        Ok(row)
    }

    async fn add(&self, body: &NewPost) -> Result<PostId> {
        let row = sqlx::query_as::<_, PostId>(
            r#"
                INSERT INTO posts (user_id, category_id, title, content)
                VALUES ($1, $2, $3, $4)
                RETURNING *;
            "#,
        )
        .bind(body.user_id)
        .bind(body.category_id)
        .bind(body.title)
        .bind(body.content)
        .fetch_one(&*self.pool)
        .await
        .context("DB Query Error (add posts)")?;

        Ok(row)
    }
}