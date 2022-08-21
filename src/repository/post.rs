use crate::db::Db;
use crate::models::post::{
    Post,
    NewPost,
    PostId,
    PostList,
    PostFindTitle,
    PostFindContent,
};
use crate::error::Result;
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
    async fn find_by_user_id(&self, user_id: i32) -> Result<PostList>;
    async fn find_by_category_id(&self, category_id: i32) -> Result<PostList>;
    async fn find_by_titles(&self, title: &PostFindTitle) -> Result<PostList>;
    async fn find_by_content(&self, content: &PostFindContent) -> Result<PostList>;
    async fn find_by_id(&self, post_id: i32) -> Result<Post>;
    async fn add(&self, body: &NewPost) -> Result<PostId>;
    async fn edit(&self, post_id: i32, body: &NewPost) -> Result<PostId>;
    async fn delete(&self, post_id: i32) -> Result<String>;
}

#[async_trait]
impl PostRepoTrait for PostRepo {
    async fn find_all(&self) -> Result<PostList> {
        let rows = sqlx::query_as::<_, Post>("SELECT * FROM posts")
            .fetch_all(&*self.pool)
            .await
            .unwrap();

        Ok(rows)
    }


    async fn find_by_user_id(&self, user_id: i32) -> Result<PostList> {
        let rows = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
        .unwrap();

        Ok(rows)
    }


    async fn find_by_category_id(&self, category_id: i32) -> Result<PostList> {
        let rows = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE category_id = $1"
        )
        .bind(category_id)
        .fetch_all(&*self.pool)
        .await
        .unwrap();

        Ok(rows)
    }


    async fn find_by_titles(&self, titles: &PostFindTitle) -> Result<PostList> {
        let rows = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE titles LIKE $1"
        )
        .bind(format!("%{}%", &titles.titles))
        .fetch_all(&*self.pool)
        .await
        .unwrap();

        Ok(rows)
    }


    async fn find_by_content(&self, content: &PostFindContent) -> Result<PostList> {
        let rows = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE titles LIKE $1"
        )
        .bind(format!("%{}%", &content.content))
        .fetch_all(&*self.pool)
        .await
        .unwrap();

        Ok(rows)
    }


    async fn find_by_id(&self, post_id: i32) -> Result<Post> {
        let row = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
            .bind(post_id)
            .fetch_one(&*self.pool)
            .await
            .unwrap();

        Ok(row)
    }


    async fn add(&self, body: &NewPost) -> Result<PostId> {
        let row = sqlx::query_as::<_, PostId>(
            r#"
                INSERT INTO posts (user_id, category_id, titles, content)
                VALUES ($1, $2, $3, $4)
                RETURNING *;
            "#,
        )
        .bind(&body.user_id)
        .bind(&body.category_id)
        .bind(&body.titles)
        .bind(&body.content)
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        Ok(row)
    }

    async fn edit(&self, post_id: i32, body: &NewPost) -> Result<PostId> {
        let row = sqlx::query_as::<_, PostId>(
            r#"
                UPDATE posts
                SET user_id = $2, category_id = $3, titles = $4, content = $5
                WHERE id = $1
                RETURNING id;
            "#,
        )
        .bind(post_id)
        .bind(&body.user_id)
        .bind(&body.category_id)
        .bind(&body.titles)
        .bind(&body.content)
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        Ok(row)
    }


    async fn delete(&self, post_id: i32) -> Result<String> {
        sqlx::query(
            "DELETE FROM posts WHERE id = $1"
        )
        .bind(post_id)
        .execute(&*self.pool)
        .await
        .unwrap();

        Ok(format!("Delete post id {}", post_id))
    }
}