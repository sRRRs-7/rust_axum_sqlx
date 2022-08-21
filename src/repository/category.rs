use crate::db::Db;
use crate::error::Result;
use crate::models::category::{Category, CategoryList, NewCategory, CategoryCondition};
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
     async fn edit(&self, category_id: i32, body: &NewCategory) -> Result<Category>;
     async fn delete(&self, category_id: i32) -> Result<String>;
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
            .unwrap();

        Ok(result)
    }

    async fn find_by_id(&self, category_id: i32) -> Result<Category> {
        let row = sqlx::query_as:: <_, Category>("SELECT * FROM categories WHERE id = $1")
            .bind(category_id)
            .fetch_one(&*self.pool)
            .await
            .unwrap();

        Ok(row)
    }

    async fn add(&self, body: &NewCategory) -> Result<Category> {
        let row = sqlx::query_as::<_, Category>(
            r#"
                INSERT INTO categories (category)
                VALUES ($1)
                RETURNING *;
            "#,
        )
        .bind(&body.category)
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        Ok(row)
    }


    async fn edit(&self, category_id: i32, new_category: &NewCategory) -> Result<Category> {
        let row = sqlx::query_as::<_, Category>(
            r#"
                UPDATE categories
                SET category = $2
                WHERE id = $1
                RETURNING *
            "#,
        )
        .bind(category_id)
        .bind(&new_category.category)
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        Ok(row)
    }


    async fn delete(&self, category_id: i32) -> Result<String> {
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(category_id)
            .execute(&*self.pool)
            .await
            .unwrap();

        Ok(format!("Delete category {}", category_id))
    }
}