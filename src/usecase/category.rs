use crate::error::Result;
use crate::models::category::{Category, NewCategory, CategoryCondition, CategoryList};
use crate::repository::{category::CategoryRepoTrait, Repositories};
use std::sync::Arc;

pub async fn get_all<R: Repositories>(
        repo: Arc<R>,
        conditions: &CategoryCondition,
) -> Result<CategoryList> {
    let users = repo.category().find_all(conditions).await?;
    Ok(users)
}


pub async fn get<R: Repositories>(
    repo: Arc<R>,
    user_id: i32,
) -> Result<Category> {
    let user = repo.category().find_by_id(user_id).await?;
    Ok(user)
}


pub async fn add<R: Repositories>(
    repo: Arc<R>,
    new_user: NewCategory,
) -> Result<Category> {
    let user_id = repo.category().add(&new_user).await?;
    Ok(user_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{
        mock::category::mock_categories,
        repository::create_repository_test,
    };

    #[tokio::test]
    async fn test_query() {
        let mut mock_repo = create_repository_test().await;
        mock_repo
            .category
            .expect_find_all()
            .returning(|_| Ok(mock_categories(5)));

        let conditions = CategoryCondition{ category: None };
        let users = get_all(Arc::new(mock_repo), &conditions).await.unwrap();
        println!("{:?}", users);
        assert_eq!(users.len(), 5);
    }
}