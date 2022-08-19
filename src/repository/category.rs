use std::sync::Arc;

use crate::db::Db;
use crate::error::Result;
use crate::models::category::{Category, CategoryList, NewCategory, CategoryCondition};
use anyhow::Context;
use async_trait::async_trait;
use mockall::automock;

pub struct CategoryRepo {
    pool: Db,
}
impl CategoryRepo {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[automock]
#[async_trait]
pub trait CategoryRepoTrait {
    async fn find_all(&self, conditions: &CategoryCondition) -> Result<CategoryList>;
    async fn find_by_id(&self, category_id: i32) -> Result<Category>;
    async fn add(&self, body: &NewCategory) -> Result<Category>;
}

#[async_trait]
impl CategoryRepoTrait for CategoryRepo {
    async fn find_all(&self, conditions: &CategoryCondition) -> Result<CategoryList> {
        let mut query = sqlx::query_as::<_, Category>("SELECT * FROM categories");
        if let Some(category) = &conditions.category {
            query = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE category LIKE $1")
                .bind(format!("%{}%", category));
        }

        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB Query Error (find all categories)")?;

        Ok(result)
    }

    async fn find_by_id(&self, category_id: i32) -> Result<Category> {
        let row = Arc::new(sqlx::query_as:: <_,Category>("SELECT * FROM category WHERE category = $1"))
            .bind(category_id)
            .fetch_one(&*self.pool)
            .await
            .context("DB Query Error (find one categories)")?;

        Ok(row)
    }

    async fn add(&self, body: &NewCategory) -> Result<Category> {
        let row = sqlx::query_as::<_, Category>(
            r#"
                INSERT INTO categories (category)
                VALUE ($1)
                RETURNING *;
            "#,
        )
        .bind(body.category)
        .fetch_one(&*self.pool)
        .await
        .context("DB Query Error (add categories)")?;

        Ok(row)
    }
}