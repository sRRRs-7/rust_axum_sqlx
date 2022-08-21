use crate::db::Db;
use crate::error::Result;
use crate::models::user::{User, NewUser, UserCondition, UserId, UserList};
use async_trait::async_trait;
use mockall::automock;

pub struct UserRepo {
    pool: Db,
}
impl UserRepo {
    pub fn new(pool: Db) -> Self {
        Self {pool}
    }
}

#[automock]
#[async_trait]
pub trait UserRepoTrait {
    async fn find_all(&self, conditions: &UserCondition) -> Result<UserList>;
    async fn find_by_id(&self, user_id: i32) -> Result<User>;
    async fn add(&self, body: &NewUser) -> Result<UserId>;
    async fn edit(&self, user_id: i32, body: &NewUser) -> Result<UserId>;
    async fn delete(&self, user_id: i32) -> Result<String>;
}

#[async_trait]
impl UserRepoTrait for UserRepo {
    async fn find_all(&self, conditions: &UserCondition) -> Result<UserList> {
        let mut query = sqlx::query_as::<_, User>("SELECT * FROM users");
        if let Some(name) = &conditions.name {
            query = sqlx::query_as::<_, User>("SELECT * FROM users WHERE name LIKE $1")
                .bind(format!("%{}%", name));
        }
        let result = query
            .fetch_all(&*self.pool)
            .await
            .unwrap();

        Ok(result)
    }

    async fn find_by_id(&self, user_id: i32) -> Result<User> {
        let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await
            .unwrap();

            Ok(row)
    }

    async fn add(&self, body: &NewUser) -> Result<UserId> {
        let row = sqlx::query_as::<_, UserId>(
            r#"
                INSERT INTO users (name, msg ,age)
                VALUES ($1, $2, $3)
                RETURNING id;
            "#,
        )
        .bind(&body.name)
        .bind(&body.msg)
        .bind(&body.age)
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        Ok(row)
    }


    async fn edit(&self, user_id: i32, body: &NewUser) -> Result<UserId> {
        let row = sqlx::query_as::<_, UserId>(
            r#"
            UPDATE posts
            SET name = $2, msg = $3, age = $4
            WHERE id = $1
            RETURNING id;
            "#,
        )
        .bind(user_id)
        .bind(&body.name)
        .bind(&body.msg)
        .bind(&body.age)
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        Ok(row)
    }


    async fn delete(&self, user_id: i32) -> Result<String> {
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .execute(&*self.pool)
            .await
            .unwrap();

        Ok(format!("Delete user {}", user_id))
    }
}