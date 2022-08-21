use crate::error::Result;
use crate::models::user::{User, NewUser, UserCondition, UserId, UserList, ImgUrl};
use crate::repository::{user::UserRepoTrait, Repositories};
use std::sync::Arc;

pub async fn get_all<R: Repositories>(
        repo: Arc<R>,
        conditions: &UserCondition,
) -> Result<UserList> {
    let users = repo.user().find_all(conditions).await?;
    Ok(users)
}


pub async fn get<R: Repositories>(
    repo: Arc<R>,
    user_id: i32,
) -> Result<User> {
    let user = repo.user().find_by_id(user_id).await?;
    Ok(user)
}


pub async fn add<R: Repositories>(
    repo: Arc<R>,
    new_user: NewUser,
) -> Result<UserId> {
    let user_id = repo.user().add(&new_user).await?;
    Ok(user_id)
}


pub async fn add_image<R: Repositories>(
    repo: Arc<R>,
    user_id: i32,
    img: ImgUrl,
) -> Result<UserId> {
    let user_id = repo.user().add_image(user_id, &img).await?;
    Ok(user_id)
}


pub async fn edit<R: Repositories>(
    repo: Arc<R>,
    user_id: i32,
    new_user: NewUser,
) -> Result<UserId> {
    let user_id = repo.user().edit(user_id, &new_user).await?;
    Ok(user_id)
}


pub async fn delete<R: Repositories>(
    repo: Arc<R>,
    user_id: i32,
) -> Result<String> {
    let res = repo.user().delete(user_id).await?;
    Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{
        mock::user::users_mock,
        repository::create_repository_test,
    };

    #[tokio::test]
    async fn test_query() {
        let mut mock_repo = create_repository_test().await;
        mock_repo
            .user
            .expect_find_all()
            .returning(|_| Ok(users_mock(5)));

        let conditions = UserCondition{ name: None };
        let users = get_all(Arc::new(mock_repo), &conditions).await.unwrap();
        assert_eq!(users.len(), 5);
    }
}